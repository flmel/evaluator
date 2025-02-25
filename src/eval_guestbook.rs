use near_sdk::{
    env::{self, log_str},
    near_bindgen,
    serde::{Deserialize, Serialize},
    serde_json::json,
    AccountId, Gas, Promise, PromiseError, ONE_NEAR,
};

pub use crate::constants::{NO_ARGS, NO_DEPOSIT, TGAS};
use crate::{Contract, ContractExt};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PostedMessage {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

#[near_bindgen]
impl Contract {
    pub fn evaluate_guestbook(&mut self, contract_account_id: AccountId) -> Promise {
        self.assert_valid_account(&contract_account_id);

        let random_strings: Vec<String> = vec![self.random_string(0), self.random_string(1)];

        let args_call_1: Vec<u8> = json!({ "text": random_strings[0] })
            .to_string()
            .into_bytes();
        let args_call_2: Vec<u8> = json!({ "text": random_strings[1] })
            .to_string()
            .into_bytes();

        Promise::new(contract_account_id.clone())
            .function_call(
                "add_message".to_string(),
                args_call_1.clone(),
                NO_DEPOSIT,
                Gas(15 * TGAS),
            )
            // Premium Message (attached deposit)
            .function_call(
                "add_message".to_string(),
                args_call_2,
                ONE_NEAR / 10,
                Gas(15 * TGAS),
            )
            .function_call(
                "last_messages".to_string(),
                json!({ "last": 2 }).to_string().into_bytes(),
                NO_DEPOSIT,
                Gas(5 * TGAS),
            )
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas(5 * TGAS))
                    .evaluate_guestbook_callback(env::predecessor_account_id(), random_strings),
            )
    }

    #[private]
    pub fn evaluate_guestbook_callback(
        &mut self,
        #[callback_result] call_result: Result<Vec<PostedMessage>, PromiseError>,
        student_id: AccountId,
        random_string: Vec<String>,
    ) -> bool {
        let mut passed = true;

        match call_result {
            Ok(messages_vec) => {
                for i in 0..1 {
                    passed = passed & (messages_vec[i].text == random_string[i]);

                    log_str(&format!(
                        "The {} message should be {}, received: {}",
                        i, random_string[i], &messages_vec[i].text
                    ));
                }

                passed = passed & messages_vec[1].premium;

                log_str(&format!(
                    "The last was premium: {}",
                    messages_vec[1].premium
                ));

                if passed {
                    let mut evaluations = self.evaluations.get(&student_id).unwrap();
                    evaluations[1] = true;
                    self.evaluations.insert(&student_id, &evaluations);
                }
            }
            Err(err) => {
                passed = false;
                log_str(&format!("{:#?}", err))
            }
        }
        passed
    }
}
