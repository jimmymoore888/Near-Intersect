// contract/src/oim.rs
//
// OutpaceInflation Module (OIM) — Rust skeleton (pure wiring)
// - structs + storage + events + auth + staleness + cooldown
// - NO strategy execution, NO fund movement, NO price feeds
//
// Designed to be embedded inside your main contract struct.
// Near-SDK version assumptions: near-sdk 4.x/5.x style macros.
//
// You will need to adapt import paths and your contract struct name.

use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault,
};

/// ----------------------------
/// Storage Keys
/// ----------------------------
#[derive(BorshSerialize, BorshStorageKey)]
pub enum OimStorageKey {
    Config,
    State,
}

/// ----------------------------
/// Public Types (JSON)
/// ----------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct InflationIndex {
    pub index_id: String,   // e.g. "US_CPI_U"
    pub period: String,     // e.g. "2026-01"
    pub value_bps: u32,     // convention: inflation bps for that period OR index-level bps (pick and stay consistent)
    pub posted_at_sec: u64, // block timestamp in seconds
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum OimMode {
    Oracle,
    FixedHurdle,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct OimConfig {
    pub mode: OimMode,

    /// Required iff mode == Oracle
    pub oracle_account: Option<AccountId>,

    /// Required iff mode == FixedHurdle
    pub fixed_hurdle_bps_annual: Option<u32>,

    /// Measurement window (policy metadata)
    pub measurement_window_days: u16, // e.g. 365

    /// Desired real return buffer (bps) above inflation/hurdle
    pub min_real_return_bps: i32, // e.g. +200

    /// Cooldown between rebalances
    pub rebalance_cooldown_sec: u32, // e.g. 86400

    /// Oracle data max age. If exceeded => OracleStale => freeze rebalance.
    pub max_oracle_age_sec: u32, // e.g. 45 days

    /// Bucket caps (bps; should sum to <= 10000 across the treasury buckets you govern)
    pub safety_cap_bps: u32,
    pub growth_cap_bps: u32,
    pub liquidity_cap_bps: u32,

    /// Optional per-rebalance step limit (safety rail)
    pub max_rebalance_step_bps: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum OimStatus {
    Healthy,
    Behind,
    OracleStale,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct OimState {
    pub last_index: Option<InflationIndex>,
    pub last_rebalance_at_sec: u64,
    pub real_return_score_bps: i32,
    pub status: OimStatus,

    /// Current policy weights (bps). These are just “intent weights”.
    /// Actual fund movement is OUT OF SCOPE for this skeleton.
    pub safety_weight_bps: u32,
    pub growth_weight_bps: u32,
    pub liquidity_weight_bps: u32,
}

/// ----------------------------
/// Internal Storage (Borsh)
/// ----------------------------

#[derive(BorshDeserialize, BorshSerialize)]
pub struct OimConfigBorsh(pub OimConfig);

#[derive(BorshDeserialize, BorshSerialize)]
pub struct OimStateBorsh(pub OimState);

/// ----------------------------
/// Events (JSON log)
/// ----------------------------
/// Keep events stable. Auditors will depend on this.
#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct OimIndexPostedEvent<'a> {
    pub event: &'a str, // "OIM_INDEX_POSTED"
    pub index_id: &'a str,
    pub period: &'a str,
    pub value_bps: u32,
    pub posted_at_sec: u64,
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct OimStatusUpdatedEvent<'a> {
    pub event: &'a str, // "OIM_STATUS_UPDATED"
    pub status: &'a OimStatus,
    pub real_return_score_bps: i32,
    pub at_sec: u64,
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct OimRebalancedEvent<'a> {
    pub event: &'a str, // "OIM_REBALANCED"
    pub from_safety_bps: u32,
    pub to_growth_bps: u32,
    pub at_sec: u64,
}

/// Helper: emit JSON event
fn emit<T: Serialize>(event: &T) {
    if let Ok(s) = near_sdk::serde_json::to_string(event) {
        env::log_str(&s);
    }
}

/// Helper: block timestamp seconds
fn now_sec() -> u64 {
    env::block_timestamp() / 1_000_000_000
}

/// ----------------------------
/// OIM Container (stored inside your main contract)
/// ----------------------------

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Oim {
    config: LazyOption<OimConfigBorsh>,
    state: LazyOption<OimStateBorsh>,
}

impl Oim {
    pub fn new() -> Self {
        Self {
            config: LazyOption::new(OimStorageKey::Config, None),
            state: LazyOption::new(OimStorageKey::State, None),
        }
    }

    /// Initialize OIM once at genesis / token creation.
    /// Recommended: call this from factory create_token() when wiring modules.
    pub fn init(&mut self, cfg: OimConfig) {
        // Minimal validation (pure wiring; strict policy validation can be expanded later)
        match cfg.mode {
            OimMode::Oracle => {
                assert!(cfg.oracle_account.is_some(), "OIM: oracle_account required in Oracle mode");
                assert!(cfg.fixed_hurdle_bps_annual.is_none(), "OIM: fixed_hurdle_bps_annual must be None in Oracle mode");
            }
            OimMode::FixedHurdle => {
                assert!(cfg.fixed_hurdle_bps_annual.is_some(), "OIM: fixed_hurdle_bps_annual required in FixedHurdle mode");
                assert!(cfg.oracle_account.is_none(), "OIM: oracle_account must be None in FixedHurdle mode");
            }
        }
        assert!(cfg.safety_cap_bps <= 10_000, "OIM: invalid safety_cap_bps");
        assert!(cfg.growth_cap_bps <= 10_000, "OIM: invalid growth_cap_bps");
        assert!(cfg.liquidity_cap_bps <= 10_000, "OIM: invalid liquidity_cap_bps");

        self.config.set(&OimConfigBorsh(cfg));

        // Default state: fully safe unless you prefer otherwise
        let st = OimState {
            last_index: None,
            last_rebalance_at_sec: 0,
            real_return_score_bps: 0,
            status: OimStatus::Healthy,
            safety_weight_bps: 10_000,
            growth_weight_bps: 0,
            liquidity_weight_bps: 0,
        };
        self.state.set(&OimStateBorsh(st));
    }

    pub fn is_initialized(&self) -> bool {
        self.config.get().is_some() && self.state.get().is_some()
    }

    fn cfg(&self) -> OimConfig {
        self.config.get().expect("OIM: not initialized").0
    }

    fn st(&self) -> OimState {
        self.state.get().expect("OIM: not initialized").0
    }

    fn set_state(&mut self, st: OimState) {
        self.state.set(&OimStateBorsh(st));
    }

    fn assert_oracle(&self) {
        let cfg = self.cfg();
        assert_eq!(cfg.mode, OimMode::Oracle, "OIM: not in Oracle mode");
        let oracle = cfg.oracle_account.expect("OIM: oracle_account missing");
        assert_eq!(env::predecessor_account_id(), oracle, "OIM: oracle auth failed");
    }

    fn assert_fresh_oracle(&self, st: &OimState, cfg: &OimConfig, now: u64) -> bool {
        // returns true if fresh, false if stale/missing
        if cfg.mode != OimMode::Oracle {
            return true;
        }
        let Some(ix) = st.last_index.as_ref() else { return false; };
        let age = now.saturating_sub(ix.posted_at_sec);
        age <= cfg.max_oracle_age_sec as u64
    }

    fn enforce_cooldown(&self, st: &OimState, cfg: &OimConfig, now: u64) {
        if st.last_rebalance_at_sec == 0 {
            return;
        }
        let elapsed = now.saturating_sub(st.last_rebalance_at_sec);
        assert!(
            elapsed >= cfg.rebalance_cooldown_sec as u64,
            "OIM: cooldown"
        );
    }

    fn clamp_weights_to_caps(st: &mut OimState, cfg: &OimConfig) {
        // Pure safety: ensure weights never exceed caps and sum <= 10000.
        if st.safety_weight_bps > cfg.safety_cap_bps {
            st.safety_weight_bps = cfg.safety_cap_bps;
        }
        if st.growth_weight_bps > cfg.growth_cap_bps {
            st.growth_weight_bps = cfg.growth_cap_bps;
        }
        if st.liquidity_weight_bps > cfg.liquidity_cap_bps {
            st.liquidity_weight_bps = cfg.liquidity_cap_bps;
        }

        let sum = st
            .safety_weight_bps
            .saturating_add(st.growth_weight_bps)
            .saturating_add(st.liquidity_weight_bps);

        if sum > 10_000 {
            // Normalize by trimming safety first (deterministic).
            let overflow = sum - 10_000;
            st.safety_weight_bps = st.safety_weight_bps.saturating_sub(overflow);
        }
    }

    /// ----------------------------
    /// Public-read equivalents
    /// ----------------------------
    pub fn get_config(&self) -> OimConfig {
        self.cfg()
    }

    pub fn get_state(&self) -> OimState {
        self.st()
    }

    /// ----------------------------
    /// Oracle feed
    /// ----------------------------
    pub fn post_inflation_index(&mut self, index_id: String, period: String, value_bps: u32) {
        self.assert_oracle();
        let mut st = self.st();
        let ts = now_sec();

        st.last_index = Some(InflationIndex {
            index_id: index_id.clone(),
            period: period.clone(),
            value_bps,
            posted_at_sec: ts,
        });

        // Optional: if previously stale, allow status to recover on next rebalance.
        // We do not change status here; status updates happen in rebalance.

        self.set_state(st);

        emit(&OimIndexPostedEvent {
            event: "OIM_INDEX_POSTED",
            index_id: &index_id,
            period: &period,
            value_bps,
            posted_at_sec: ts,
        });
    }

    /// ----------------------------
    /// Rebalance (pure policy weights only)
    /// ----------------------------
    pub fn oim_rebalance(&mut self) {
        let cfg = self.cfg();
        let mut st = self.st();
        let ts = now_sec();

        self.enforce_cooldown(&st, &cfg, ts);

        // Oracle freshness gate (fail-safe)
        if cfg.mode == OimMode::Oracle {
            let fresh = self.assert_fresh_oracle(&st, &cfg, ts);
            if !fresh {
                st.status = OimStatus::OracleStale;
                st.real_return_score_bps = 0;
                self.set_state(st.clone());
                emit(&OimStatusUpdatedEvent {
                    event: "OIM_STATUS_UPDATED",
                    status: &st.status,
                    real_return_score_bps: st.real_return_score_bps,
                    at_sec: ts,
                });
                // Freeze: do not change weights, do not advance last_rebalance timestamp
                return;
            }
        }

        // --- Score computation placeholder (NO strategy risk)
        // In v1 skeleton, we do not compute vault_growth_bps on-chain.
        // You will later wire deterministic accounting from your treasury/vault primitives.
        //
        // For now:
        // - Oracle mode: treat inflation_bps = last_index.value_bps (whatever convention you adopt)
        // - FixedHurdle mode: use fixed_hurdle_bps_annual
        let inflation_bps: i32 = match cfg.mode {
            OimMode::Oracle => st
                .last_index
                .as_ref()
                .map(|ix| ix.value_bps as i32)
                .unwrap_or(0),
            OimMode::FixedHurdle => cfg.fixed_hurdle_bps_annual.unwrap_or(0) as i32,
        };

        // Placeholder: vault_growth_bps = 0 until you wire deterministic accounting
        let vault_growth_bps: i32 = 0;

        st.real_return_score_bps = vault_growth_bps - inflation_bps - cfg.min_real_return_bps;

        st.status = if st.real_return_score_bps < 0 {
            OimStatus::Behind
        } else {
            OimStatus::Healthy
        };

        emit(&OimStatusUpdatedEvent {
            event: "OIM_STATUS_UPDATED",
            status: &st.status,
            real_return_score_bps: st.real_return_score_bps,
            at_sec: ts,
        });

        // --- Pure reweighting logic (NO fund movement)
        // If Behind: shift some bps from Safety -> Growth within caps and step cap.
        let mut from_safety = 0u32;
        let mut to_growth = 0u32;

        if st.status == OimStatus::Behind {
            let step = cfg.max_rebalance_step_bps.unwrap_or(100); // default 1% step if not set
            let available_from_safety = st.safety_weight_bps;
            let growth_headroom = cfg.growth_cap_bps.saturating_sub(st.growth_weight_bps);

            let delta = step.min(available_from_safety).min(growth_headroom);

            if delta > 0 {
                st.safety_weight_bps = st.safety_weight_bps.saturating_sub(delta);
                st.growth_weight_bps = st.growth_weight_bps.saturating_add(delta);
                from_safety = delta;
                to_growth = delta;
            }
        }

        // Always clamp to caps
        Self::clamp_weights_to_caps(&mut st, &cfg);

        // Commit rebalance timestamp only on successful policy application (even if delta=0, it’s a “checked” rebalance)
        st.last_rebalance_at_sec = ts;
        self.set_state(st);

        emit(&OimRebalancedEvent {
            event: "OIM_REBALANCED",
            from_safety_bps: from_safety,
            to_growth_bps: to_growth,
            at_sec: ts,
        });
    }
}

/// ----------------------------
/// Example Integration Pattern
/// ----------------------------
///
/// In your main contract:
///
/// #[near_bindgen]
/// #[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
/// pub struct Contract {
///     // ...
///     oim: Oim,
/// }
///
/// impl Contract {
///     #[init]
///     pub fn new(...) -> Self {
///         let mut c = Self { /* ... */ oim: Oim::new() };
///         // optional: c.oim.init(cfg) during token instantiation
///         c
///     }
///
///     // expose pass-through view methods
///     pub fn get_oim_config(&self) -> OimConfig { self.oim.get_config() }
///     pub fn get_oim_state(&self) -> OimState { self.oim.get_state() }
///
///     // oracle method
///     pub fn post_inflation_index(&mut self, index_id: String, period: String, value_bps: u32) {
///         self.oim.post_inflation_index(index_id, period, value_bps)
///     }
///
///     // governance/multisig call boundary should wrap this
///     pub fn oim_rebalance(&mut self) { self.oim.oim_rebalance() }
/// }
