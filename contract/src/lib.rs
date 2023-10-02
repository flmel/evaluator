use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::LookupMap,
    env::{self, log},
    env::{predecessor_account_id, random_seed},
    log, near_bindgen, require,
    serde::{Deserialize, Serialize},
    serde_json::json,
    AccountId, Gas, Promise, PromiseError, ONE_NEAR,
};

pub mod external;
pub use crate::external::*;

pub const TGAS: u64 = 1_000_000_000_000;
pub const NO_DEPOSIT: u128 = 0;
pub const NO_ARGS: Vec<u8> = vec![];

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PostedMessage {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

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
    // ========================================
    // Hello NEAR contract evaluation
    pub fn evaluate_hello_near(&mut self, contract_account_id: AccountId) -> Promise {
        require!(
            self.evaluating_sub_account(&contract_account_id),
            format!(
                "Please deploy contract as sub account. Such as hello_near.{}",
                env::predecessor_account_id()
            ),
        );

        // First let's get a random string from random seed
        let get_array: Vec<u8> = random_seed();
        let random_string: String = String::from_utf8_lossy(&get_array).to_string();
        println!("the random string is {:?}", random_string);

        let args = json!({ "greeting": random_string })
            .to_string()
            .into_bytes();

        Promise::new(contract_account_id.clone())
            .function_call("set_greeting".to_string(), args, NO_DEPOSIT, Gas(15 * TGAS))
            .function_call(
                "get_greeting".to_string(),
                NO_ARGS,
                NO_DEPOSIT,
                Gas(5 * TGAS),
            )
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas(5 * TGAS))
                    .evaluate_hello_near_callback(random_string, contract_account_id.clone()),
            )
    }
    // Hello Near Evaluation Callback
    #[private]
    pub fn evaluate_hello_near_callback(
        &mut self,
        #[callback_result] last_result: Result<String, PromiseError>,
        random_string: String,
        contract_name: AccountId,
    ) -> bool {
        // The callback only has access to the last action's result
        if let Ok(result) = last_result {
            log!(format!("The last result is {result}"));
            let output = result == random_string;
            self.records.insert(&contract_name, &output);
            output
        } else {
            log!("The batch call failed and all calls got reverted");
            false
        }
    }

    // ========================================
    // Guest Book Contract Evaluation
    pub fn evaluate_guestbook(&mut self, contract_account_id: AccountId) -> Promise {
        require!(
            self.evaluating_sub_account(&contract_account_id),
            format!(
                "Please deploy contract as sub account. Such as guestbook.{}",
                env::predecessor_account_id()
            ),
        );

        let args = json!({ "text": "Hello from Evaluator" })
            .to_string()
            .into_bytes();

        Promise::new(contract_account_id.clone())
            .function_call(
                "add_message".to_string(),
                args.clone(),
                NO_DEPOSIT,
                Gas(15 * TGAS),
            )
            // Premium Message (attached deposit)
            .function_call(
                "add_message".to_string(),
                args,
                ONE_NEAR / 10,
                Gas(15 * TGAS),
            )
            .function_call(
                "get_messages".to_string(),
                // TODO: Using NO_ARGS here causes a "Failed to deserialize the input: EOF while parsing a value at line 1 column 0"
                json!({}).to_string().into_bytes(),
                NO_DEPOSIT,
                Gas(5 * TGAS),
            )
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas(5 * TGAS))
                    .evaluate_guestbook_callback(contract_account_id.clone()),
            )
    }

    // Guest Book Evaluation Callback
    #[private]
    pub fn evaluate_guestbook_callback(
        &mut self,
        #[callback_result] call_result: Result<Vec<PostedMessage>, PromiseError>,
        contract_name: AccountId,
    ) {
        // The callback only has access to the last action's result
        match call_result {
            Ok(messages_vec) => {
                require!(
                    messages_vec.len() >= 2,
                    "There should be at least 2 messages in the guestbook"
                );

                let last_message = &messages_vec[messages_vec.len() - 1];
                require!(
                    last_message.text == "Hello from Evaluator",
                    "The last message should be from the evaluator"
                );
                require!(last_message.premium, "The last message should be premium");

                log!("It works! The last message is {}", last_message.text);
            }
            // log Error message
            Err(err) => log!("{:#?}", err),
        }
    }

    // ========================================
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
}
