use near_sdk::{
    near_bindgen,
    AccountId,
    PanicOnDefault,
    borsh::{BorshDeserialize, BorshSerialize},
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
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
