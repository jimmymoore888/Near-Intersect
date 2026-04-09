use near_sdk::near;

#[near(contract_state)]
pub struct Contract {
    owner: String,
}

#[near]
impl Contract {
    #[init]
    pub fn new(owner: String) -> Self {
        Self { owner }
    }

    pub fn get_owner(&self) -> String {
        self.owner.clone()
    }
}
