use near_sdk::{
    env, near, require, AccountId, NearToken, PanicOnDefault, Promise,
};

const YOCTO_PER_NEAR: u128 = 1_000_000_000_000_000_000_000_000;

const BPS_DENOMINATOR: u128 = 10_000;
const SYSTEM_FEE_BPS: u128 = 500; // 5.00%

const TREASURY_BPS: u128 = 140;   // 1.40%
const GROWTH_BPS: u128 = 90;      // 0.90%
const VOLCANO_BPS: u128 = 105;    // 1.05%
const RESERVE_BPS: u128 = 55;     // 0.55%
const CORE_OPS_BPS: u128 = 110;   // 1.10%

const OPERATIONS_WALLET: &str = "lawdeploy.near";

const CREATE_RECORD_FEE_YOCTO: u128 = 32_000_000_000_000_000_000_000; // 0.032 NEAR
const CREATE_RECORD_TREASURY_YOCTO: u128 = 16_000_000_000_000_000_000_000; // 0.016 NEAR
const CREATE_RECORD_PRESSURE_YOCTO: u128 = 16_000_000_000_000_000_000_000; // 0.016 NEAR

const MIN_ELIGIBLE_POSITION_YOCTO: u128 = YOCTO_PER_NEAR; // 1 NEAR anti-spam floor

const SECONDS_TO_NANOS: u64 = 1_000_000_000;
const PRODUCTION_LOCK_NS: u64 = 63_072_000 * SECONDS_TO_NANOS; // 2 years using 365-day years
const PRODUCTION_EXIT_WINDOW_NS: u64 = 604_800 * SECONDS_TO_NANOS; // 7 days

const TEST_LOCK_NS: u64 = 7_200 * SECONDS_TO_NANOS; // 2 hours
const TEST_EXIT_WINDOW_NS: u64 = 420 * SECONDS_TO_NANOS; // 7 minutes

const PHASE_MILESTONES_YOCTO: [u128; 16] = [
    100_000 * YOCTO_PER_NEAR,
    250_000 * YOCTO_PER_NEAR,
    500_000 * YOCTO_PER_NEAR,
    1_000_000 * YOCTO_PER_NEAR,
    2_500_000 * YOCTO_PER_NEAR,
    5_000_000 * YOCTO_PER_NEAR,
    10_000_000 * YOCTO_PER_NEAR,
    25_000_000 * YOCTO_PER_NEAR,
    50_000_000 * YOCTO_PER_NEAR,
    100_000_000 * YOCTO_PER_NEAR,
    250_000_000 * YOCTO_PER_NEAR,
    500_000_000 * YOCTO_PER_NEAR,
    1_000_000_000 * YOCTO_PER_NEAR,
    10_000_000_000 * YOCTO_PER_NEAR,
    100_000_000_000 * YOCTO_PER_NEAR,
    1_000_000_000_000 * YOCTO_PER_NEAR,
];

#[near(serializers = [borsh, json])]
#[derive(Clone)]
pub struct Participant {
    pub account_id: AccountId,
    pub active: bool,
    pub position_balance: u128,
    pub total_deposited: u128,
    pub created_at: u64,
    pub lock_started_at: u64,
    pub claimed_eruption_ids: Vec<u64>,
}

#[near(serializers = [borsh, json])]
#[derive(Clone)]
pub struct EruptionSnapshot {
    pub eruption_id: u64,
    pub phase_number: u64,
    pub threshold: u128,
    pub distribution_pool: u128,
    pub retained_amount: u128,
    pub unallocated_remainder: u128,
    pub share_per_wallet: u128,
    pub eligible_count: u64,
    pub claimed_count: u64,
    pub created_at: u64,
    pub claim_deadline: u64,
    pub eligible_accounts: Vec<AccountId>,
}

#[near(serializers = [borsh, json])]
#[derive(Clone)]
pub struct SystemStatus {
    pub project_status: String,
    pub website_status: String,
    pub wallet_status: String,
    pub contract_status: String,
    pub treasury_status: String,
    pub eruption_engine_status: String,
    pub oim_status: String,
    pub tpi_status: String,
    pub public_fund_launch_status: String,
}

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,

    pub operations_wallet: AccountId,
    pub treasury_wallet: AccountId,
    pub growth_wallet: AccountId,
    pub reserve_wallet: AccountId,

    pub test_mode: bool,
    pub lock_duration_ns: u64,
    pub exit_window_ns: u64,

    pub participants: Vec<Participant>,
    pub eruptions: Vec<EruptionSnapshot>,

    pub volcano_pressure: u128,
    pub eruption_count: u64,
    pub created_record_count: u64,
}

#[near]
impl Contract {
    #[init]
    pub fn new(
        owner_id: AccountId,
        treasury_wallet: AccountId,
        growth_wallet: AccountId,
        reserve_wallet: AccountId,
        test_mode: bool,
    ) -> Self {
        require!(!env::state_exists(), "Already initialized");

        require!(
            TREASURY_BPS + GROWTH_BPS + VOLCANO_BPS + RESERVE_BPS + CORE_OPS_BPS
                == SYSTEM_FEE_BPS,
            "Fee split must equal 5%"
        );

        require!(
            CREATE_RECORD_TREASURY_YOCTO + CREATE_RECORD_PRESSURE_YOCTO
                == CREATE_RECORD_FEE_YOCTO,
            "Record creation fee split mismatch"
        );

        let operations_wallet: AccountId = OPERATIONS_WALLET
            .parse()
            .expect("Invalid operations wallet");

        let lock_duration_ns = if test_mode {
            TEST_LOCK_NS
        } else {
            PRODUCTION_LOCK_NS
        };

        let exit_window_ns = if test_mode {
            TEST_EXIT_WINDOW_NS
        } else {
            PRODUCTION_EXIT_WINDOW_NS
        };

        Self {
            owner_id,
            operations_wallet,
            treasury_wallet,
            growth_wallet,
            reserve_wallet,
            test_mode,
            lock_duration_ns,
            exit_window_ns,
            participants: Vec::new(),
            eruptions: Vec::new(),
            volcano_pressure: 0,
            eruption_count: 0,
            created_record_count: 0,
        }
    }

    #[payable]
    pub fn deposit(&mut self) {
        let amount = env::attached_deposit().as_yoctonear();
        require!(amount > 0, "Attach deposit");

        let caller = env::predecessor_account_id();

        let treasury = amount * TREASURY_BPS / BPS_DENOMINATOR;
        let growth = amount * GROWTH_BPS / BPS_DENOMINATOR;
        let volcano = amount * VOLCANO_BPS / BPS_DENOMINATOR;
        let reserve = amount * RESERVE_BPS / BPS_DENOMINATOR;
        let core_ops = amount * CORE_OPS_BPS / BPS_DENOMINATOR;

        let total_fee = treasury + growth + volcano + reserve + core_ops;
        let expected_fee = amount * SYSTEM_FEE_BPS / BPS_DENOMINATOR;
        require!(total_fee <= expected_fee, "Fee math error");

        let locked_position = amount
            .checked_sub(total_fee)
            .expect("Position underflow");

        Promise::new(self.treasury_wallet.clone())
            .transfer(NearToken::from_yoctonear(treasury));

        Promise::new(self.growth_wallet.clone())
            .transfer(NearToken::from_yoctonear(growth));

        Promise::new(self.reserve_wallet.clone())
            .transfer(NearToken::from_yoctonear(reserve));

        Promise::new(self.operations_wallet.clone())
            .transfer(NearToken::from_yoctonear(core_ops));

        self.add_or_update_participant(caller.clone(), locked_position);

        self.volcano_pressure = self
            .volcano_pressure
            .checked_add(volcano)
            .expect("Pressure overflow");

        self.try_trigger_eruption();

        env::log_str(&format!(
            "DEPOSIT caller={} amount={} fee={} locked_position={} treasury={} growth={} volcano={} reserve={} core_ops={} pressure={} participants={}",
            caller,
            amount,
            total_fee,
            locked_position,
            treasury,
            growth,
            volcano,
            reserve,
            core_ops,
            self.volcano_pressure,
            self.participants.len()
        ));
    }

    #[payable]
    pub fn create_volcano_record(&mut self) {
        let amount = env::attached_deposit().as_yoctonear();

        require!(
            amount == CREATE_RECORD_FEE_YOCTO,
            "Attach exactly 0.032 NEAR"
        );

        let caller = env::predecessor_account_id();

        Promise::new(self.treasury_wallet.clone())
            .transfer(NearToken::from_yoctonear(CREATE_RECORD_TREASURY_YOCTO));

        self.volcano_pressure = self
            .volcano_pressure
            .checked_add(CREATE_RECORD_PRESSURE_YOCTO)
            .expect("Pressure overflow");

        self.created_record_count += 1;

        self.try_trigger_eruption();

        env::log_str(&format!(
            "VOLCANO_RECORD_CREATED creator={} fee={} treasury={} pressure_added={} record_number={} pressure={} note=not_nep141_token_creation",
            caller,
            amount,
            CREATE_RECORD_TREASURY_YOCTO,
            CREATE_RECORD_PRESSURE_YOCTO,
            self.created_record_count,
            self.volcano_pressure
        ));
    }

    #[payable]
    pub fn create_token(&mut self) {
        self.create_volcano_record();

        env::log_str(
            "CREATE_TOKEN_COMPATIBILITY_CALL note=this_method_creates_a_volcano_record_not_a_nep141_token",
        );
    }

    pub fn withdraw_position(&mut self, amount: u128) {
        require!(amount > 0, "Amount must be greater than zero");

        let caller = env::predecessor_account_id();
        let now = env::block_timestamp();

        let index = self
            .participant_index(&caller)
            .expect("Participant not found");

        {
            let participant = &self.participants[index];

            require!(participant.active, "Participant inactive");
            require!(
                participant.position_balance >= amount,
                "Insufficient position balance"
            );
            require!(
                self.is_exit_window_open_for(participant, now),
                "Exit window is not open"
            );
        }

        let treasury = amount * TREASURY_BPS / BPS_DENOMINATOR;
        let growth = amount * GROWTH_BPS / BPS_DENOMINATOR;
        let volcano = amount * VOLCANO_BPS / BPS_DENOMINATOR;
        let reserve = amount * RESERVE_BPS / BPS_DENOMINATOR;
        let core_ops = amount * CORE_OPS_BPS / BPS_DENOMINATOR;

        let total_fee = treasury + growth + volcano + reserve + core_ops;
        let net_to_user = amount
            .checked_sub(total_fee)
            .expect("Withdraw fee underflow");

        self.participants[index].position_balance = self.participants[index]
            .position_balance
            .checked_sub(amount)
            .expect("Position underflow");

        if self.participants[index].position_balance == 0 {
            self.participants[index].active = false;
        }

        Promise::new(self.treasury_wallet.clone())
            .transfer(NearToken::from_yoctonear(treasury));

        Promise::new(self.growth_wallet.clone())
            .transfer(NearToken::from_yoctonear(growth));

        Promise::new(self.reserve_wallet.clone())
            .transfer(NearToken::from_yoctonear(reserve));

        Promise::new(self.operations_wallet.clone())
            .transfer(NearToken::from_yoctonear(core_ops));

        Promise::new(caller.clone())
            .transfer(NearToken::from_yoctonear(net_to_user));

        self.volcano_pressure = self
            .volcano_pressure
            .checked_add(volcano)
            .expect("Pressure overflow");

        self.try_trigger_eruption();

        env::log_str(&format!(
            "WITHDRAW_POSITION caller={} gross={} net_to_user={} fee={} treasury={} growth={} volcano={} reserve={} core_ops={} remaining_position={} pressure={}",
            caller,
            amount,
            net_to_user,
            total_fee,
            treasury,
            growth,
            volcano,
            reserve,
            core_ops,
            self.participants[index].position_balance,
            self.volcano_pressure
        ));
    }

    pub fn claim_eruption(&mut self, eruption_id: u64) {
        let caller = env::predecessor_account_id();
        let now = env::block_timestamp();

        let participant_index = self
            .participant_index(&caller)
            .expect("Participant not found");

        let eruption_index = self
            .eruption_index(eruption_id)
            .expect("Eruption not found");

        require!(
            now <= self.eruptions[eruption_index].claim_deadline,
            "Claim window closed"
        );

        require!(
            self.eruptions[eruption_index]
                .eligible_accounts
                .contains(&caller),
            "Wallet not eligible for this eruption"
        );

        require!(
            !self.participants[participant_index]
                .claimed_eruption_ids
                .contains(&eruption_id),
            "Already claimed"
        );

        let share = self.eruptions[eruption_index].share_per_wallet;
        require!(share > 0, "No claimable share");

        self.participants[participant_index]
            .claimed_eruption_ids
            .push(eruption_id);

        self.eruptions[eruption_index].claimed_count += 1;

        Promise::new(caller.clone())
            .transfer(NearToken::from_yoctonear(share));

        env::log_str(&format!(
            "ERUPTION_CLAIMED caller={} eruption_id={} share={} claimed_count={} eligible_count={}",
            caller,
            eruption_id,
            share,
            self.eruptions[eruption_index].claimed_count,
            self.eruptions[eruption_index].eligible_count
        ));
    }

    fn try_trigger_eruption(&mut self) {
        loop {
            let phase_index = self.eruption_count as usize;

            let threshold = match self.phase_threshold_by_index(phase_index) {
                Some(value) => value,
                None => {
                    env::log_str("ERUPTION_CHECK_STOPPED reason=no_more_supported_phases");
                    return;
                }
            };

            if self.volcano_pressure < threshold {
                return;
            }

            self.trigger_eruption_for_phase(phase_index, threshold);
        }
    }

    fn trigger_eruption_for_phase(&mut self, phase_index: usize, threshold: u128) {
        require!(self.volcano_pressure >= threshold, "Threshold not reached");

        let phase_number = self.eruption_count + 1;
        let now = env::block_timestamp();

        let distribution_pool = if phase_number == 1 {
            threshold * 75 / 100
        } else {
            threshold * 60 / 100
        };

        let retained_amount = threshold
            .checked_sub(distribution_pool)
            .expect("Retained underflow");

        let eligible_accounts = self.eligible_accounts();
        let eligible_count = eligible_accounts.len() as u64;

        let share_per_wallet = if eligible_count > 0 {
            distribution_pool / eligible_count as u128
        } else {
            0
        };

        let allocated_claim_pool = share_per_wallet * eligible_count as u128;
        let unallocated_remainder = distribution_pool
            .checked_sub(allocated_claim_pool)
            .expect("Remainder underflow");

        let extra_pressure = self
            .volcano_pressure
            .checked_sub(threshold)
            .expect("Extra pressure underflow");

        self.volcano_pressure = retained_amount
            .checked_add(unallocated_remainder)
            .and_then(|v| v.checked_add(extra_pressure))
            .expect("Pressure carry overflow");

        let eruption_id = self.eruption_count + 1;

        let snapshot = EruptionSnapshot {
            eruption_id,
            phase_number,
            threshold,
            distribution_pool: allocated_claim_pool,
            retained_amount,
            unallocated_remainder,
            share_per_wallet,
            eligible_count,
            claimed_count: 0,
            created_at: now,
            claim_deadline: now
                .checked_add(self.exit_window_ns)
                .expect("Claim deadline overflow"),
            eligible_accounts,
        };

        self.eruptions.push(snapshot);
        self.eruption_count += 1;

        env::log_str(&format!(
            "ERUPTION_SNAPSHOT eruption_id={} phase={} threshold={} allocated_distribution={} retained={} unallocated_remainder={} share_per_wallet={} eligible_count={} carried_pressure={} next_threshold={}",
            eruption_id,
            phase_number,
            threshold,
            allocated_claim_pool,
            retained_amount,
            unallocated_remainder,
            share_per_wallet,
            eligible_count,
            self.volcano_pressure,
            self.get_next_eruption_threshold()
        ));

        let next_index = phase_index + 1;
        if next_index >= PHASE_MILESTONES_YOCTO.len() {
            env::log_str("ERUPTION_PHASE_TABLE_END reached=phase_16 no_future_phase_promised");
        }
    }

    fn add_or_update_participant(&mut self, account_id: AccountId, locked_position: u128) {
        let now = env::block_timestamp();

        match self.participant_index(&account_id) {
            Some(index) => {
                self.participants[index].active = true;
                self.participants[index].position_balance = self.participants[index]
                    .position_balance
                    .checked_add(locked_position)
                    .expect("Position overflow");

                self.participants[index].total_deposited = self.participants[index]
                    .total_deposited
                    .checked_add(locked_position)
                    .expect("Deposit total overflow");
            }
            None => {
                self.participants.push(Participant {
                    account_id,
                    active: true,
                    position_balance: locked_position,
                    total_deposited: locked_position,
                    created_at: now,
                    lock_started_at: now,
                    claimed_eruption_ids: Vec::new(),
                });
            }
        }
    }

    fn participant_index(&self, account_id: &AccountId) -> Option<usize> {
        self.participants
            .iter()
            .position(|p| &p.account_id == account_id)
    }

    fn eruption_index(&self, eruption_id: u64) -> Option<usize> {
        self.eruptions
            .iter()
            .position(|e| e.eruption_id == eruption_id)
    }

    fn eligible_accounts(&self) -> Vec<AccountId> {
        self.participants
            .iter()
            .filter(|p| {
                p.active && p.position_balance >= MIN_ELIGIBLE_POSITION_YOCTO
            })
            .map(|p| p.account_id.clone())
            .collect()
    }

    fn is_exit_window_open_for(&self, participant: &Participant, now: u64) -> bool {
        if !participant.active {
            return false;
        }

        if now < participant.lock_started_at {
            return false;
        }

        let elapsed = now - participant.lock_started_at;

        if elapsed < self.lock_duration_ns {
            return false;
        }

        let cycle = self
            .lock_duration_ns
            .checked_add(self.exit_window_ns)
            .expect("Cycle overflow");

        let offset = (elapsed - self.lock_duration_ns) % cycle;

        offset < self.exit_window_ns
    }

    fn phase_threshold_by_index(&self, phase_index: usize) -> Option<u128> {
        if phase_index < PHASE_MILESTONES_YOCTO.len() {
            return Some(PHASE_MILESTONES_YOCTO[phase_index]);
        }

        None
    }

    pub fn get_system_status(&self) -> SystemStatus {
        SystemStatus {
            project_status: "DEVELOPMENT_STAGE".to_string(),
            website_status: "PUBLIC".to_string(),
            wallet_status: "PENDING_UNTIL_VERIFIED".to_string(),
            contract_status: "SOURCE_PRESENT_NOT_PUBLIC_FUND_LAUNCH_VERIFIED".to_string(),
            treasury_status: "PENDING_PUBLIC_VERIFICATION".to_string(),
            eruption_engine_status: "IMPLEMENTED_AS_SNAPSHOT_CLAIM_MODEL_PENDING_AUDIT".to_string(),
            oim_status: "SPECIFIED_PENDING_RUNTIME_VERIFICATION".to_string(),
            tpi_status: "SPECIFIED_PENDING_RUNTIME_VERIFICATION".to_string(),
            public_fund_launch_status: "NOT_APPROVED_BEFORE_AUDIT_SOURCE_WASM_CONTRACT_AND_LEGAL_VERIFICATION".to_string(),
        }
    }

    pub fn get_owner(&self) -> AccountId {
        self.owner_id.clone()
    }

    pub fn get_operations_wallet(&self) -> AccountId {
        self.operations_wallet.clone()
    }

    pub fn get_treasury_wallet(&self) -> AccountId {
        self.treasury_wallet.clone()
    }

    pub fn get_growth_wallet(&self) -> AccountId {
        self.growth_wallet.clone()
    }

    pub fn get_reserve_wallet(&self) -> AccountId {
        self.reserve_wallet.clone()
    }

    pub fn get_test_mode(&self) -> bool {
        self.test_mode
    }

    pub fn get_lock_duration_ns(&self) -> u64 {
        self.lock_duration_ns
    }

    pub fn get_exit_window_ns(&self) -> u64 {
        self.exit_window_ns
    }

    pub fn get_system_fee_bps(&self) -> u128 {
        SYSTEM_FEE_BPS
    }

    pub fn get_fee_split_bps(&self) -> (u128, u128, u128, u128, u128) {
        (
            TREASURY_BPS,
            GROWTH_BPS,
            VOLCANO_BPS,
            RESERVE_BPS,
            CORE_OPS_BPS,
        )
    }

    pub fn get_volcano_pressure(&self) -> u128 {
        self.volcano_pressure
    }

    pub fn get_next_eruption_threshold(&self) -> u128 {
        match self.phase_threshold_by_index(self.eruption_count as usize) {
            Some(value) => value,
            None => 0,
        }
    }

    pub fn get_current_phase_number(&self) -> u64 {
        self.eruption_count + 1
    }

    pub fn get_phase_milestones(&self) -> Vec<u128> {
        PHASE_MILESTONES_YOCTO.to_vec()
    }

    pub fn get_eruption_count(&self) -> u64 {
        self.eruption_count
    }

    pub fn get_created_record_count(&self) -> u64 {
        self.created_record_count
    }

    pub fn get_participant_count(&self) -> u64 {
        self.participants.len() as u64
    }

    pub fn get_eligible_participant_count(&self) -> u64 {
        self.eligible_accounts().len() as u64
    }

    pub fn get_participants(&self) -> Vec<Participant> {
        self.participants.clone()
    }

    pub fn get_participant(&self, account_id: AccountId) -> Option<Participant> {
        self.participant_index(&account_id)
            .map(|index| self.participants[index].clone())
    }

    pub fn get_position_balance(&self, account_id: AccountId) -> u128 {
        self.participant_index(&account_id)
            .map(|index| self.participants[index].position_balance)
            .unwrap_or(0)
    }

    pub fn is_exit_window_open(&self, account_id: AccountId) -> bool {
        match self.participant_index(&account_id) {
            Some(index) => self.is_exit_window_open_for(
                &self.participants[index],
                env::block_timestamp(),
            ),
            None => false,
        }
    }

    pub fn get_eruption_snapshots(&self) -> Vec<EruptionSnapshot> {
        self.eruptions.clone()
    }

    pub fn get_eruption_snapshot(&self, eruption_id: u64) -> Option<EruptionSnapshot> {
        self.eruption_index(eruption_id)
            .map(|index| self.eruptions[index].clone())
    }

    pub fn has_claimed_eruption(&self, account_id: AccountId, eruption_id: u64) -> bool {
        match self.participant_index(&account_id) {
            Some(index) => self.participants[index]
                .claimed_eruption_ids
                .contains(&eruption_id),
            None => false,
        }
    }

    pub fn get_create_record_fee_yocto(&self) -> u128 {
        CREATE_RECORD_FEE_YOCTO
    }

    pub fn get_min_eligible_position_yocto(&self) -> u128 {
        MIN_ELIGIBLE_POSITION_YOCTO
    }

    pub fn get_oim_status(&self) -> String {
        "SPECIFIED_PENDING_RUNTIME_VERIFICATION".to_string()
    }

    pub fn get_tpi_status(&self) -> String {
        "SPECIFIED_PENDING_RUNTIME_VERIFICATION".to_string()
    }

    pub fn canonical_law_summary(&self) -> String {
        "NEAR_INTERSECT_VOLCANO_CANONICAL_PRE_LAUNCH_LAW: development_stage; 5_percent_fee; treasury_1_40_growth_0_90_volcano_1_05_reserve_0_55_core_ops_1_10; production_lock_2_years; exit_window_7_days; test_mode_2_hours_7_minutes; no_manual_eruption; explicit_phase_table; phase_1_75_25; phase_2_plus_60_40; eligible_active_wallet_snapshot_claims; local_profiles_no_financial_rights; OIM_TPI_pending_runtime_verification; no_guaranteed_returns".to_string()
    }
}
