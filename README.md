NEAR Evaluator
==============

> This is a work in progress

This project aims to create a tool that automatically evaluates students on NEAR smart contract development. It was intended to be used within a larger educational project, in which students follow a self-paced course, and can automatically evaluate their gain knowledge.

From a technical perspective, `NEAR Evaluator` is a smart contract that automatically evaluates other smart contracts. It works by making a cross contract calls to the contract being evaluated, and checking the data it returns.

On passing all evaluations, students will be able to claim a non-transferable `NEAR Certified Developer` NFT. 

The NFT contract can be found at [evaluator-certificate-issuer](https://github.com/flmel/evaluator-certificate-issuer/)

In addition to this smart contract, we also provide additional challenges in form of integration tests suite that can be used to further explore development on NEAR. 

The additional test suite can be found at [evaluator-integration-checks](https://github.com/flmel/evaluator-integration-checks/) repository.

---

## Evaluation
To be evaluated, the student needs to first call the `register` method, to register the account being evaluated. Afterwards, we will expect the student to call all methods using always the registered account, and to deploy all smart contracts in a sub-account of the registered account.

### [1. Hello NEAR](contract/src/eval_hello.rs)
Here we evaluate that the student knows how to deploy a simple smart contract. 

```rs
evaluate_hello_near(contract_account_id: AccountId)
```

The contract makes a batch call to `contract_account_id`, calling `set_greeting` with a random string and `get_greeting`. The expected result of `get_greeting` is the random string that was set.  


### [2. Guest Book](contract/src/eval_guestbook.rs)
Here we evaluate that the student knows how to store an array of messages on a contract.

```rs
evaluate_guestbook(contract_account_id: AccountId)
```

The contract makes a batch call to `contract_account_id`, calling `add_message` twice, and then `last_messages({last: 2})`. The contract being evaluated is expected to return the two messages that were added.  

### [3. Complex Datatypes](contract/src/eval_complex_input.rs)
Here we evaluate that the student knows how to handle types such as `AccountId`, `U64`, and `Objects`.

```rs
evaluate_datatypes(contract_account_id: AccountId)
```

We make a batch call to `set_data` and `get_data`. We expect `get_data` to return a new object containing all the complex data given to `set_data`.

### [4. Cross Contract Calls](contract/src/eval_xcc.rs)
Here we evaluate that the student knows how to write and call cross contract calls, and handle the returned data.

```rs
evaluate_xcc(contract_account_id: AccountId)
```

### Claiming the NFT

After passing all evaluations, the student can claim the NFT by calling the `claim_certificate` method. This method will check if the student has passed all evaluations, and if so, will mint the NFT on the [NFT contract](https://github.com/flmel/evaluator-integration-checks/) to the student.

```rs
claim_certificate()
``` 
---

#### Progress Checklist
- [x] Evaluate a `hello world` contract
- [x] Evaluate a `guestbook` contract
- [x] Evaluate the input and output of complex types / objects
- [x] Evaluate an explicit init (via integration-checks tests)
- [x] Evaluate private and public methods (via integration-checks tests)
- [x] Handling NEAR transfers (via integration-checks tests)
- [x] Evaluating usage of ENV variables (via integration-checks tests)
- [x] Evaluate storage of complex data types (via integration-checks tests)
- [ ] Evaluate the use of collections
- [ ] Evaluate basic actions
- [ ] Evaluate cross-contract calls
- [ ] Implement a Simple BOS Frontend

Long Term Planning
====
[ ] Evaluate factories
[ ] NEP Evaluator for NFT & FT Contracts
