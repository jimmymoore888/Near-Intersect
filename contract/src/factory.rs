use near_sdk::{
    near_bindgen, env, AccountId, PanicOnDefault,
    collections::UnorderedMap,
};
use serde::{Serialize, Deserialize};

/// ==============================
/// TOKEN PROFILE (SYSTEM ANCHOR)
/// ==============================
#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenProfile {
    pub owner: AccountId,
    pub total_supply: u128,
    pub oim_enabled: bool,
    pub modules: Vec<u8>,
}

/// ==============================
/// CONTRACT STATE (REGISTRY)
/// ==============================
#[near_bindgen]
#[derive(PanicOnDefault)]
pub struct Contract {
    pub tokens: UnorderedMap<AccountId, TokenProfile>,
}

/// ==============================
/// INITIALIZATION
/// ==============================
#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            tokens: UnorderedMap::new(b"t"),
        }
    }
}

/// ==============================
/// MODULE ENGINE (DETERMINISTIC)
/// ==============================
fn generate_module_profile(seed: u64) -> Vec<u8> {
    let mut modules: Vec<u8> = vec![];

    // Base deterministic seed (bounded to modules 1–7)
    let base = seed % 7 + 1;

    for i in 0..3 {
        let module_id = ((base + i as u64) % 7 + 1) as u8;

        // Enforce max 2 duplicates
        let count = modules.iter().filter(|&&m| m == module_id).count();
        if count < 2 {
            modules.push(module_id);
        }
    }

    modules
}

/// ==============================
/// FACTORY CORE (COHESION LAYER)
/// ==============================
#[near_bindgen]
impl Contract {

    /// Create a new RagTuff-compliant token profile
    pub fn create_token(&mut self, owner: AccountId) {

        // --------------------------
        // VALIDATION
        // --------------------------
        assert!(
            self.tokens.get(&owner).is_none(),
            "Token already exists for this owner"
        );

        // --------------------------
        // SEED (CHAIN-BASED)
        // --------------------------
        let seed = env::block_timestamp();

        // --------------------------
        // MODULE GENERATION
        // --------------------------
        let modules = generate_module_profile(seed);

        // --------------------------
        // CONSTRUCT TOKEN PROFILE
        // --------------------------
        let profile = TokenProfile {
            owner: owner.clone(),
            total_supply: 10_000_000_000, // 10B fixed supply
            oim_enabled: true,            // LAW: mandatory
            modules,
        };

        // --------------------------
        // PERSIST (REGISTRY)
        // --------------------------
        self.tokens.insert(&owner, &profile);

        // --------------------------
        // EVENT LOG (OBSERVABILITY)
        // --------------------------
        env::log_str(
            &format!(
                "TOKEN_CREATED: owner={}, supply={}, oim={}, modules={:?}",
                owner,
                profile.total_supply,
                profile.oim_enabled,
                profile.modules
            )
        );
    }

    /// View a token profile
    pub fn get_token(&self, owner: AccountId) -> Option<TokenProfile> {
        self.tokens.get(&owner)
    }
}
