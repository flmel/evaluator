use near_sdk::{
    env, near_bindgen, require, serde_json::json, AccountId, Gas, Promise, PromiseError,
};

pub use crate::constants::{NO_ARGS, NO_DEPOSIT, TGAS};
use crate::{Contract, ContractExt};

#[near_bindgen]
impl Contract {
    pub fn evaluate_hello_near(&mut self, contract_account_id: AccountId) -> Promise {
        self.assert_valid_account(&contract_account_id);

        let random_string: String = self.random_string(1);

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
                    .evaluate_hello_near_callback(env::predecessor_account_id(), random_string),
            )
    }
    // Hello Near Evaluation Callback
    #[private]
    pub fn evaluate_hello_near_callback(
        &mut self,
        #[callback_result] call_result: Result<String, PromiseError>,
        student_id: AccountId,
        random_string: String,
    ) {
        match call_result {
            Ok(greeting) => {
                require!(
                    greeting == random_string,
                    format!(
                        "Expected greeting to be {}, not {}",
                        random_string, greeting
                    )
                );
                let mut evaluations = self.evaluations.get(&student_id).unwrap();
                evaluations[0] = true;
                self.evaluations.insert(&student_id, &evaluations);
            }
            Err(err) => {
                require!(false, format!("{:#?}", err));
            }
        }
    }
}
