use near_sdk::{
    env, log, near_bindgen, require,
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
        require!(
            self.evaluating_sub_account(&contract_account_id),
            format!(
                "Please deploy contract as sub account. Such as guestbook.{}",
                env::predecessor_account_id()
            ),
        );

        let random_string = self.random_string();
        let args: Vec<u8> = json!({ "text": self.random_string() })
            .to_string()
            .into_bytes();
        let args_call_2: Vec<u8> = json!({ "text": random_string }).to_string().into_bytes();

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
                args_call_2,
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
                    .evaluate_guestbook_callback(random_string),
            )
    }

    #[private]
    pub fn evaluate_guestbook_callback(
        &mut self,
        #[callback_result] call_result: Result<Vec<PostedMessage>, PromiseError>,
        random_string: String,
    ) {
        match call_result {
            Ok(messages_vec) => {
                require!(
                    messages_vec.len() >= 2,
                    "There should be at least 2 messages in the guestbook"
                );

                let last_message = &messages_vec[messages_vec.len() - 1];
                require!(
                    last_message.text == random_string,
                    format!("The last message should be {}", random_string)
                );
                require!(last_message.premium, "The last message should be premium");

                log!(
                    "Guestbook evaluation success! Last message is:  {}",
                    last_message.text
                );
            }
            // Log Error message
            Err(err) => log!("{:#?}", err),
        }
    }
}
