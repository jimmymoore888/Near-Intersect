use near_sdk::{
    env, near, require, AccountId, NearToken, PanicOnDefault, Promise,
};

const BPS_DENOMINATOR: u128 = 10_000;
const SYSTEM_FEE_BPS: u128 = 500; // 5.00%

// These split the 5% fee itself.
// Total must equal 500 bps.
const TREASURY_BPS: u128 = 140;   // 1.40%
const GROWTH_BPS: u128 = 90;      // 0.90%
const VOLCANO_BPS: u128 = 105;    // 1.05%
const RESERVE_BPS: u128 = 55;     // 0.55%
const CORE_OPS_BPS: u128 = 110;   // 1.10% <- your cut

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub operations_wallet: AccountId,
    pub treasury_wallet: AccountId,
    pub growth_wallet: AccountId,
    pub reserve_wallet: AccountId,

    pub volcano_pressure: u128,
    pub next_eruption_threshold: u128,
}

#[near]
impl Contract {
    #[init]
    pub fn new(
        owner_id: AccountId,
        operations_wallet: AccountId,
        treasury_wallet: AccountId,
        growth_wallet: AccountId,
        reserve_wallet: AccountId,
    ) -> Self {
        require!(!env::state_exists(), "Already initialized");

        require!(
            TREASURY_BPS + GROWTH_BPS + VOLCANO_BPS + RESERVE_BPS + CORE_OPS_BPS == SYSTEM_FEE_BPS,
            "Fee split must equal 5%"
        );

        Self {
            owner_id,
            operations_wallet,
            treasury_wallet,
            growth_wallet,
            reserve_wallet,
            volcano_pressure: 0,
            next_eruption_threshold: NearToken::from_near(100_000).as_yoctonear(),
        }
    }

    #[payable]
    pub fn deposit(&mut self) {
        let amount = env::attached_deposit().as_yoctonear();
        require!(amount > 0, "Attach deposit");

        let fee = amount * SYSTEM_FEE_BPS / BPS_DENOMINATOR;

        let treasury = amount * TREASURY_BPS / BPS_DENOMINATOR;
        let growth = amount * GROWTH_BPS / BPS_DENOMINATOR;
        let volcano = amount * VOLCANO_BPS / BPS_DENOMINATOR;
        let reserve = amount * RESERVE_BPS / BPS_DENOMINATOR;
        let core_ops = amount * CORE_OPS_BPS / BPS_DENOMINATOR;

        let total_split = treasury + growth + volcano + reserve + core_ops;
        require!(total_split <= fee, "Fee math overflow");

        Promise::new(self.treasury_wallet.clone()).transfer(NearToken::from_yoctonear(treasury));
        Promise::new(self.growth_wallet.clone()).transfer(NearToken::from_yoctonear(growth));
        Promise::new(self.reserve_wallet.clone()).transfer(NearToken::from_yoctonear(reserve));
        Promise::new(self.operations_wallet.clone()).transfer(NearToken::from_yoctonear(core_ops));

        self.volcano_pressure += volcano;

        if self.volcano_pressure >= self.next_eruption_threshold {
            self.trigger_eruption();
        }

        env::log_str(&format!(
            "DEPOSIT_RECEIVED amount={} fee={} core_ops={} volcano_pressure={}",
            amount, fee, core_ops, self.volcano_pressure
        ));
    }

    fn trigger_eruption(&mut self) {
        let threshold = self.next_eruption_threshold;

        // First doctrine model:
        // 75% distributable, 25% remains as carried pressure/reserve.
        let distributable = threshold * 75 / 100;
        let carried = self.volcano_pressure - distributable;

        self.volcano_pressure = carried;

        // Example threshold progression:
        // 100,000 -> 250,000 -> 625,000 -> keeps scaling
        self.next_eruption_threshold = self.next_eruption_threshold * 250 / 100;

        env::log_str(&format!(
            "ERUPTION_TRIGGERED threshold={} distributable={} carried={} next_threshold={}",
            threshold, distributable, carried, self.next_eruption_threshold
        ));
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
}
use near_sdk::{near, AccountId, PanicOnDefault};

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {
    owner: AccountId,
}

#[near]
impl Contract {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        Self { owner }
    }

    pub fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }
}
