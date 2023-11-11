use near_sdk::{
    env::{self, log_str},
    json_types::U64,
    near_bindgen, require,
    serde::{Deserialize, Serialize},
    serde_json::json,
    AccountId, Gas, Promise, PromiseError,
};

use crate::{
    constants::{NO_DEPOSIT, TGAS},
    Contract, ContractExt,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Args {
    account: AccountId,
    number_big: U64,
    number_small: u8,
    point: Point,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct MockStructure {
    big: U64,
    small: u8,
    vector: Vec<U64>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct MockReturn {
    account: String,
    x: i32,
    structure: MockStructure,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Point {
    x: i32,
    y: i32,
}

#[near_bindgen]
impl Contract {
    pub fn evaluate_complex_input(&mut self, contract_account_id: AccountId) -> Promise {
        self.assert_valid_account(&contract_account_id);

        let account_id = env::signer_account_id();
        let number_big = U64(env::block_height());
        let number_small = self.random_u8(0);
        let point = Point {
            x: i32::from(self.random_u8(1)),
            y: i32::from(self.random_u8(2)),
        };

        let number = number_big.0 - 1;

        let expected = MockReturn {
            account: account_id.to_string(),
            x: point.x,
            structure: MockStructure {
                big: number_big,
                small: number_small,
                vector: vec![number_big, U64::from(number)],
            },
        };

        let args = json!(Args {
            account: account_id,
            number_big,
            number_small,
            point
        })
        .to_string()
        .into_bytes();

        Promise::new(contract_account_id.clone())
            .function_call(
                "provide_output".to_string(),
                args,
                NO_DEPOSIT,
                Gas(5 * TGAS),
            )
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas(5 * TGAS))
                    .eval_ci_callback(env::predecessor_account_id(), expected),
            )
    }

    #[private]
    pub fn eval_ci_callback(
        &mut self,
        #[callback_result] call_result: Result<MockReturn, PromiseError>,
        student_id: AccountId,
        expected: MockReturn,
    ) -> bool {
        match call_result {
            Ok(result) => {
                log_str(&format!(
                    "Expected structure to be {:#?}, received {:#?}",
                    expected, result
                ));
                require!(result.eq(&expected));

                let mut evaluations = self.evaluations.get(&student_id).unwrap();
                evaluations[3] = true;
                self.evaluations.insert(&student_id, &evaluations);

                true
            }
            Err(err) => {
                log_str(&format!("{:#?}", err));
                false
            }
        }
    }
}
