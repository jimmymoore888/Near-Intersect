use near_sdk::{
    env, near, require, AccountId, NearToken, PanicOnDefault, Promise,
};

const BPS_DENOMINATOR: u128 = 10_000;
const SYSTEM_FEE_BPS: u128 = 500; // 5.00%

const OPERATIONS_WALLET: &str = "lawdeploy.near";

const TREASURY_BPS: u128 = 140;   // 1.40%
const GROWTH_BPS: u128 = 90;      // 0.90%
const VOLCANO_BPS: u128 = 105;    // 1.05%
const RESERVE_BPS: u128 = 55;     // 0.55%
const CORE_OPS_BPS: u128 = 110;   // 1.10%

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,

    pub operations_wallet: AccountId,
    pub treasury_wallet: AccountId,
    pub growth_wallet: AccountId,
    pub reserve_wallet: AccountId,

    pub participants: Vec<AccountId>,

    pub volcano_pressure: u128,
    pub next_eruption_threshold: u128,
    pub next_distribution_amount: u128,
    pub eruption_count: u64,
}

#[near]
impl Contract {
    #[init]
    pub fn new(
        owner_id: AccountId,
        treasury_wallet: AccountId,
        growth_wallet: AccountId,
        reserve_wallet: AccountId,
    ) -> Self {
        require!(!env::state_exists(), "Already initialized");

        require!(
            TREASURY_BPS + GROWTH_BPS + VOLCANO_BPS + RESERVE_BPS + CORE_OPS_BPS
                == SYSTEM_FEE_BPS,
            "Fee split must equal 5%"
        );

        let operations_wallet: AccountId = OPERATIONS_WALLET
            .parse()
            .expect("Invalid operations wallet");

        Self {
            owner_id,
            operations_wallet,
            treasury_wallet,
            growth_wallet,
            reserve_wallet,

            participants: Vec::new(),

            volcano_pressure: 0,
            next_eruption_threshold: NearToken::from_near(100_000).as_yoctonear(),
            next_distribution_amount: NearToken::from_near(75_000).as_yoctonear(),
            eruption_count: 0,
        }
    }

    #[payable]
    pub fn deposit(&mut self) {
        let amount = env::attached_deposit().as_yoctonear();
        require!(amount > 0, "Attach deposit");

        let caller = env::predecessor_account_id();

        if !self.participants.contains(&caller) {
            self.participants.push(caller.clone());
        }

        let treasury = amount * TREASURY_BPS / BPS_DENOMINATOR;
        let growth = amount * GROWTH_BPS / BPS_DENOMINATOR;
        let volcano = amount * VOLCANO_BPS / BPS_DENOMINATOR;
        let reserve = amount * RESERVE_BPS / BPS_DENOMINATOR;
        let core_ops = amount * CORE_OPS_BPS / BPS_DENOMINATOR;

        let total_fee = treasury + growth + volcano + reserve + core_ops;
        let expected_fee = amount * SYSTEM_FEE_BPS / BPS_DENOMINATOR;

        require!(total_fee <= expected_fee, "Fee math error");

        Promise::new(self.treasury_wallet.clone())
            .transfer(NearToken::from_yoctonear(treasury));

        Promise::new(self.growth_wallet.clone())
            .transfer(NearToken::from_yoctonear(growth));

        Promise::new(self.reserve_wallet.clone())
            .transfer(NearToken::from_yoctonear(reserve));

        Promise::new(self.operations_wallet.clone())
            .transfer(NearToken::from_yoctonear(core_ops));

        self.volcano_pressure = self
            .volcano_pressure
            .checked_add(volcano)
            .expect("Pressure overflow");

        if self.volcano_pressure >= self.next_eruption_threshold {
            self.trigger_eruption();
        }

        env::log_str(&format!(
            "DEPOSIT amount={} caller={} total_fee={} treasury={} growth={} volcano={} reserve={} core_ops={} pressure={} participants={}",
            amount,
            caller,
            total_fee,
            treasury,
            growth,
            volcano,
            reserve,
            core_ops,
            self.volcano_pressure,
            self.participants.len()
        ));
    }

    fn trigger_eruption(&mut self) {
        let threshold = self.next_eruption_threshold;
        let distribution = self.next_distribution_amount;

        require!(self.volcano_pressure >= threshold, "Threshold not reached");

        let reserved = threshold.saturating_sub(distribution);
        let extra_pressure = self.volcano_pressure.saturating_sub(threshold);

        self.volcano_pressure = reserved
            .checked_add(extra_pressure)
            .expect("Carry overflow");

        self.next_eruption_threshold = threshold
            .checked_mul(250)
            .and_then(|v| v.checked_div(100))
            .expect("Threshold overflow");

        self.next_distribution_amount = distribution
            .checked_mul(200)
            .and_then(|v| v.checked_div(100))
            .expect("Distribution overflow");

        self.eruption_count += 1;

        env::log_str(&format!(
            "ERUPTION count={} threshold={} distribution={} reserved={} carried_pressure={} next_threshold={} next_distribution={} participants={}",
            self.eruption_count,
            threshold,
            distribution,
            reserved,
            self.volcano_pressure,
            self.next_eruption_threshold,
            self.next_distribution_amount,
            self.participants.len()
        ));
    }

    pub fn get_owner(&self) -> AccountId {
        self.owner_id.clone()
    }

    pub fn get_operations_wallet(&self) -> AccountId {
        self.operations_wallet.clone()
    }

    pub fn get_volcano_pressure(&self) -> u128 {
        self.volcano_pressure
    }

    pub fn get_next_eruption_threshold(&self) -> u128 {
        self.next_eruption_threshold
    }

    pub fn get_next_distribution_amount(&self) -> u128 {
        self.next_distribution_amount
    }

    pub fn get_eruption_count(&self) -> u64 {
        self.eruption_count
    }

    pub fn get_participant_count(&self) -> u64 {
        self.participants.len() as u64
    }

    pub fn get_participants(&self) -> Vec<AccountId> {
        self.participants.clone()
    }
}
