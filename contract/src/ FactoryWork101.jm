use near_sdk::{
    near_bindgen,
    env,
    AccountId,
    BorshDeserialize,
    BorshSerialize,
};

use near_sdk::store::UnorderedMap;
use near_sdk::serde::{Serialize, Deserialize};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenProfile {
    pub owner: AccountId,
    pub total_supply: u128,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    tokens: UnorderedMap<String, TokenProfile>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            tokens: UnorderedMap::new(b"t"),
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn create_token(&mut self, token_id: String, total_supply: u128) {
        let owner = env::predecessor_account_id();

        let profile = TokenProfile {
            owner,
            total_supply,
        };

        self.tokens.insert(token_id, profile);
    }

    pub fn get_token(&self, token_id: String) -> Option<TokenProfile> {
        self.tokens.get(&token_id).cloned()
    }
}
