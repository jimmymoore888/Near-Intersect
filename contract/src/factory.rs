use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::AccountId;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Factory {
    registry: LookupMap<String, AccountId>,
}

impl Factory {
    pub fn new() -> Self {
        Self {
            registry: LookupMap::new(b"r"),
        }
    }

    pub fn create_token(&mut self, symbol: String, account: AccountId) {
        let s = symbol.to_uppercase();

        // Make sure the symbol is not already used
        assert!(
            self.registry.get(&s).is_none(),
            "SYMBOL_USED"
        );

        // Register the token account under that symbol
        self.registry.insert(&s, &account);
        // Logging removed for SDK 3.x compatibility
    }

    pub fn get_token(&self, symbol: String) -> Option<AccountId> {
        let s = symbol.to_uppercase();
        self.registry.get(&s)
    }
}
