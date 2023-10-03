use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::LookupMap,
    env::{self, predecessor_account_id, random_seed},
    near_bindgen, require, AccountId,
};

pub use crate::constants::{REGISTRATION_COST, BASIC_EVAL_NUMBER};


pub mod external;
pub use crate::external::*;
mod constants;
mod eval_guestbook;
mod eval_hello;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    evaluations: LookupMap<AccountId, Vec<bool>>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            evaluations: LookupMap::new(b"r".to_vec()),
        }
    }
}

#[near_bindgen]
impl Contract {

    #[payable]
    pub fn register(&mut self) {
        require!(
            env::attached_deposit() >= REGISTRATION_COST,
            format!("Please attach at least {} NEAR to register", REGISTRATION_COST) 
        );

        let account_id = env::predecessor_account_id();

        require!(
            !self.evaluations.contains_key(&account_id),
            "You are already registered"
        );

        let evaluations = vec![false; BASIC_EVAL_NUMBER];

        self.evaluations.insert(&account_id, &evaluations);
    }

    pub fn get_evaluations(&self, account_id: AccountId) -> Vec<bool> {
        self.evaluations.get(&account_id).unwrap()
    }

    pub fn passed_all_exams(&self, account_id: AccountId) -> bool {
        let evaluations = self.evaluations.get(&account_id).unwrap();
        evaluations.iter().all(|&x| x)
    }

    fn assert_valid_account(&self, sub_account_id: &AccountId) {
        let parent_id: AccountId = sub_account_id.to_string().split(".").skip(1).collect::<Vec<&str>>().join(".").parse().unwrap();

        // Only parent accounts can evaluate sub-accounts
        require!(
            parent_id == predecessor_account_id(),
            format!("Only {} can evaluate {}", parent_id, sub_account_id)
        );

        // Check the parent account is registered
        require!(
            self.evaluations.contains_key(&parent_id),
            format!("{} is not registered", parent_id)
        );
    }

    fn random_string(&self, seed: u8) -> String {
        let get_array: Vec<u8> = random_seed();
        String::from_utf8_lossy(&get_array).to_string() + &seed.to_string()
    }
}

// UNIT TESTS
// Note: #[private] macro doesn't expand in unit tests
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    fn get_context(is_view: bool) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .is_view(is_view)
            .current_account_id("contract.testnet".parse().unwrap());
        builder
    }

    #[test]
    fn test_evaluating_sub_account() {
        let mut context = get_context(false);
        let mut contract = Contract::default();

        testing_env!(context
            .predecessor_account_id("someone.testnet".parse().unwrap())
            .build());

        contract.register();
        contract.assert_valid_account(&"hello_near.someone.testnet".parse().unwrap());
    }

    #[test]
    fn test_random_string() {
        let mut context = get_context(false);
        let contract = Contract::default();

        testing_env!(context
            .predecessor_account_id("someone.testnet".parse().unwrap())
            .build());

        assert!(contract.random_string(1) != contract.random_string(2));
    }
}
