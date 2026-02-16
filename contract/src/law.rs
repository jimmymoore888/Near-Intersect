use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;
use std::collections::BTreeSet;

pub type UInt = U128;
pub type Address = AccountId;

/// ----------------------------
/// Module 1: FixedSupply
/// ----------------------------
#[derive(Clone, Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FixedSupply {
    pub amount: UInt,
}

/// ----------------------------
/// Module 2: BurnCap
/// ----------------------------
#[derive(Clone, Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct BurnCap {
    pub cap: UInt,
}

/// ----------------------------
/// Module 3: TimeLock
/// NOTE: duration is expressed in seconds at the schema level (policy).
/// Execution of locking/unlocking belongs to the token contract template.
/// ----------------------------
#[derive(Clone, Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TimeLock {
    pub duration: UInt,
}

/// ----------------------------
/// Module 4: Airdrop (uniform amount per recipient)
/// ----------------------------
#[derive(Clone, Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Airdrop {
    pub recipients: Vec<Address>,
    pub amount: UInt,
}

/// ----------------------------
/// Module 5: VestingSchedule (uniform amount per recipient)
/// cliff/duration expressed in seconds at schema level.
/// ----------------------------
#[derive(Clone, Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct VestingSchedule {
    pub cliff: UInt,
    pub duration: UInt,
    pub recipients: Vec<Address>,
    pub amount: UInt,
}

/// ----------------------------
/// Module 6: LiquidityBootstrap
/// pair = destination account (DEX pool account or controller)
/// ----------------------------
#[derive(Clone, Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct LiquidityBootstrap {
    pub pair: Address,
    pub amount: UInt,
}

/// ----------------------------
/// Module 7: PercentageDistribution
/// percentage interpreted as BASIS POINTS (0..=10000).
/// ----------------------------
#[derive(Clone, Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PercentageDistribution {
    pub percentage: UInt,
    pub recipients: Vec<Address>,
}

/// ----------------------------
/// LAW v1 schema (factory-side binding)
/// - fixed_supply is required
/// - everything else optional
/// - percentage_distributions can be empty or many
/// ----------------------------
#[derive(Clone, Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct LawV1Schema {
    pub fixed_supply: FixedSupply,

    pub burn_cap: Option<BurnCap>,
    pub time_lock: Option<TimeLock>,

    pub airdrop: Option<Airdrop>,
    pub vesting_schedule: Option<VestingSchedule>,
    pub liquidity_bootstrap: Option<LiquidityBootstrap>,

    pub percentage_distributions: Vec<PercentageDistribution>,
}

impl LawV1Schema {
    /// Hard-fail invariant enforcement (deterministic).
    pub fn validate(&self) {
        let supply: u128 = self.fixed_supply.amount.0;
        assert!(supply > 0, "LAW: fixed_supply.amount must be > 0");

        // Module 2
        if let Some(bc) = &self.burn_cap {
            assert!(bc.cap.0 <= supply, "LAW: burn_cap.cap exceeds fixed supply");
        }

        // Module 3
        if let Some(tl) = &self.time_lock {
            assert!(tl.duration.0 > 0, "LAW: time_lock.duration must be > 0");
        }

        // Explicit-reserve totals
        let mut reserved: u128 = 0;

        // Module 4
        if let Some(ad) = &self.airdrop {
            assert!(!ad.recipients.is_empty(), "LAW: airdrop.recipients empty");
            assert_unique_accounts(&ad.recipients, "LAW: airdrop.recipients not unique");
            assert!(ad.amount.0 > 0, "LAW: airdrop.amount must be > 0");

            let n = ad.recipients.len() as u128;
            let total = ad.amount.0.checked_mul(n).expect("LAW: airdrop total overflow");
            reserved = reserved.checked_add(total).expect("LAW: reserved overflow");
        }

        // Module 5
        if let Some(vs) = &self.vesting_schedule {
            assert!(!vs.recipients.is_empty(), "LAW: vesting_schedule.recipients empty");
            assert_unique_accounts(&vs.recipients, "LAW: vesting_schedule.recipients not unique");
            assert!(vs.amount.0 > 0, "LAW: vesting_schedule.amount must be > 0");
            assert!(vs.duration.0 > 0, "LAW: vesting_schedule.duration must be > 0");
            assert!(vs.cliff.0 <= vs.duration.0, "LAW: vesting_schedule.cliff > duration");

            let n = vs.recipients.len() as u128;
            let total = vs.amount.0.checked_mul(n).expect("LAW: vesting total overflow");
            reserved = reserved.checked_add(total).expect("LAW: reserved overflow");
        }

        // Module 6
        if let Some(lb) = &self.liquidity_bootstrap {
            assert!(lb.amount.0 > 0, "LAW: liquidity_bootstrap.amount must be > 0");
            reserved = reserved
                .checked_add(lb.amount.0)
                .expect("LAW: reserved overflow");
        }

        // Module 7 (bps)
        let mut sum_bps: u128 = 0;
        for pd in &self.percentage_distributions {
            assert!(!pd.recipients.is_empty(), "LAW: percentage_distribution.recipients empty");
            assert_unique_accounts(&pd.recipients, "LAW: percentage_distribution.recipients not unique");
            let bps = pd.percentage.0;
            assert!(bps <= 10_000, "LAW: percentage_distribution.percentage must be <= 10000 bps");
            sum_bps = sum_bps.checked_add(bps).expect("LAW: bps sum overflow");
        }
        assert!(sum_bps <= 10_000, "LAW: percentage_distributions sum > 10000 bps");

        // Convert bps into reserved amount (rounding down is deterministic)
        let pct_reserved = supply
            .checked_mul(sum_bps)
            .expect("LAW: pct_reserved overflow")
            / 10_000;

        reserved = reserved.checked_add(pct_reserved).expect("LAW: reserved overflow");

        // Global allocation safety
        assert!(
            reserved <= supply,
            "LAW: allocations exceed fixed supply"
        );
    }
}

fn assert_unique_accounts(accts: &[AccountId], err: &str) {
    let mut set = BTreeSet::new();
    for a in accts {
        assert!(set.insert(a.as_str().to_string()), "{err}");
    }
}
