use near_sdk::{
    near,
    near_bindgen,
    AccountId,
    PanicOnDefault,
    borsh::{BorshDeserialize, BorshSerialize},
};

#[near]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[borsh(crate = "near_sdk::borsh")]
pub struct Contract {
    owner: AccountId,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        Self { owner }
    }

    pub fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }
}
