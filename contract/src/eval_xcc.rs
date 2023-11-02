use near_sdk::{
    env::{self, log_str},
    json_types::U128,
    near_bindgen, AccountId, Gas, Promise, PromiseError,
};

use crate::{
    constants::{NO_ARGS, NO_DEPOSIT, TGAS},
    Contract, ContractExt,
};

#[near_bindgen]
impl Contract {
    pub fn evaluate_xcc(&mut self, contract_account_id: AccountId) -> Promise {
        self.assert_valid_account(&contract_account_id);

        let rand_uint = self.random_u128(0);

        self.temp_u128.insert(&contract_account_id, &rand_uint);

        Promise::new(contract_account_id)
            .function_call(
                "get_current_uint".to_string(),
                NO_ARGS,
                NO_DEPOSIT,
                Gas(40 * TGAS),
            )
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas(5 * TGAS))
                    .evaluate_current_uint(env::predecessor_account_id(), U128(rand_uint)),
            )
    }

    pub fn provide_u128(&self) -> U128 {
        U128(self.temp_u128.get(&env::predecessor_account_id()).unwrap())
    }

    // Evaluate current uint
    #[private]
    pub fn evaluate_current_uint(
        &mut self,
        #[callback_result] call_result: Result<U128, PromiseError>,
        student_id: AccountId,
        expected_uint: U128,
    ) -> bool {
        self.temp_u128.remove(&env::predecessor_account_id());

        let mut passed = false;

        match call_result {
            Ok(current_uint) => {
                log_str(&format!(
                    "Expected current uint to be {}, received {}",
                    expected_uint.0, current_uint.0
                ));
                if current_uint.0 == expected_uint.0 {
                    let mut evaluations = self.evaluations.get(&student_id).unwrap();
                    evaluations[2] = true;
                    self.evaluations.insert(&student_id, &evaluations);
                    passed = true;
                };
            }
            _ => {}
        }

        passed
    }
}
