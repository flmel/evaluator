import { Worker, NearAccount, NEAR, ONE_NEAR } from "near-workspaces";
import anyTest, { TestFn } from "ava";

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

test.beforeEach(async (t) => {
  // Init the worker and start a Sandbox server
  const worker = await Worker.init();

  // Deploy contract
  const root = worker.rootAccount;
  const evaluator = await root.createSubAccount("evaluator", { initialBalance: NEAR.parse("30 N").toJSON() });

  const student = await root.createSubAccount("student", { initialBalance: NEAR.parse("100 N").toJSON() });
  const helloNear = await student.createSubAccount("hello", { initialBalance: NEAR.parse("10 N").toJSON() });

  // Get wasm file path from package.json test script in folder above
  await evaluator.deploy(process.argv[2]);
  await student.call(evaluator, "register", {}, { attachedDeposit: NEAR.parse("2 N").toJSON() });

  // Save state for test runs, it is unique for each test
  t.context.worker = worker;
  t.context.accounts = { root, evaluator, student, helloNear };
});


test.afterEach(async (t) => {
  // Stop Sandbox server
  await t.context.worker.tearDown().catch((error) => {
    console.log("Failed to stop the Sandbox:", error);
  });
});

test("Test Hello Near", async (t) => {
  const { evaluator, student, helloNear } = t.context.accounts;
  const result = await student.call(evaluator, "evaluate_hello_near", { contract_account_id: helloNear.accountId }, { gas: "30000000000000" });
  t.is(result, true);
});


// test("Test GuestBook", async (t) => {
//   const { evaluator, student, guestBook } = t.context.accounts;
//   const result = await student.call(evaluator, guestBook.accountId, { contract_name: guestBook.accountId }, { gas: "30000000000000" });
//   t.is(result, true);
// });
