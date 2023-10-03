use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::LookupMap,
    env::{self, predecessor_account_id, random_seed},
    near_bindgen, require, AccountId,
};

pub mod external;
pub use crate::external::*;
mod constants;
mod eval_guestbook;
mod eval_hello;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    records: LookupMap<AccountId, bool>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            records: LookupMap::new(b"r".to_vec()),
        }
    }
}

#[near_bindgen]
impl Contract {
    // Account ID that's being checked is Sub-Account of the caller
    #[private]
    pub fn evaluating_sub_account(&self, account_id: &AccountId) -> bool {
        require!(
            account_id != &env::predecessor_account_id(),
            "You cannot evaluate top level account"
        );

        account_id
            .as_str()
            .contains(predecessor_account_id().as_str())
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
        let contract = Contract::default();

        testing_env!(context
            .predecessor_account_id("someone.testnet".parse().unwrap())
            .build());

        assert!(contract.evaluating_sub_account(&"hello_near.someone.testnet".parse().unwrap()));
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
