use near_sdk::{
    env,
    log, near_bindgen, require,
    serde_json::json,
    AccountId, Gas, Promise, PromiseError,
};

use crate::{Contract, ContractExt};
pub use crate::constants::{NO_ARGS, NO_DEPOSIT, TGAS};

#[near_bindgen]
impl Contract {
    pub fn evaluate_hello_near(&mut self, contract_account_id: AccountId) -> Promise {
        require!(
            self.evaluating_sub_account(&contract_account_id),
            format!(
                "Please deploy contract as sub account. Such as hello_near.{}",
                env::predecessor_account_id()
            ),
        );

        // First let's get a random string from random seed
        let random_string: String = self.random_string();

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
}