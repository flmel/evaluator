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
  const evaluator = await root.createSubAccount("evaluator", { initialBalance: NEAR.parse("10 N").toJSON() });

  const student = await root.createSubAccount("student", { initialBalance: NEAR.parse("100 N").toJSON() });

  const helloNear = await student.createSubAccount("hello", { initialBalance: NEAR.parse("2 N").toJSON() });
  await helloNear.deploy('./src/aux_contracts/hello_near.wasm');

  const guestBook = await student.createSubAccount("guest", { initialBalance: NEAR.parse("2 N").toJSON() });
  await guestBook.deploy('./src/aux_contracts/guestbook.wasm');

  const xcc = await student.createSubAccount("xcc", { initialBalance: NEAR.parse("2 N").toJSON() });
  await xcc.deploy('./src/aux_contracts/xcc.wasm');

  const ci = await student.createSubAccount("ci", { initialBalance: NEAR.parse("2 N").toJSON() });
  await ci.deploy('./src/aux_contracts/ci.wasm');

  // Get wasm file path from package.json test script in folder above
  await evaluator.deploy(process.argv[2]);
  await student.call(evaluator, "register", {}, { attachedDeposit: NEAR.parse("1 N").toJSON() });

  // Save state for test runs, it is unique for each test
  t.context.worker = worker;
  t.context.accounts = { root, evaluator, student, helloNear, guestBook, xcc, ci };
});


test.afterEach(async (t) => {
  // Stop Sandbox server
  await t.context.worker.tearDown().catch((error) => {
    console.log("Failed to stop the Sandbox:", error);
  });
});

test("Test Hello Near", async (t) => {
  const { evaluator, student, helloNear } = t.context.accounts;
  await student.call(evaluator, "evaluate_hello_near", { contract_account_id: helloNear.accountId }, { gas: "300000000000000" });
  t.is(true, true);
});

test("Test GuestBook", async (t) => {
  const { evaluator, student, guestBook } = t.context.accounts;
  await student.call(evaluator, "evaluate_guestbook", { contract_account_id: guestBook.accountId }, { gas: "300000000000000" });
  t.is(true, true);
});

test("Test XCC", async (t) => {
  const { evaluator, student, xcc } = t.context.accounts;
  await student.call(evaluator, "evaluate_xcc", { contract_account_id: xcc.accountId }, { gas: "300000000000000" });
  t.is(true, true);
});

test("Test Complex Input", async (t) => {
  const { evaluator, student, ci } = t.context.accounts;
  let result = await student.call(evaluator, "evaluate_complex_input", { contract_account_id: ci.accountId }, { gas: "300000000000000" });
  console.log(result);
  t.is(result, true);
});

test("Passed all tests", async (t) => {
  const { evaluator, student, helloNear, guestBook, xcc, ci } = t.context.accounts;
  await student.call(evaluator, "evaluate_hello_near", { contract_account_id: helloNear.accountId }, { gas: "300000000000000" });
  await student.call(evaluator, "evaluate_guestbook", { contract_account_id: guestBook.accountId }, { gas: "300000000000000" });
  await student.call(evaluator, "evaluate_xcc", { contract_account_id: xcc.accountId }, { gas: "300000000000000" });
  await student.call(evaluator, "evaluate_complex_input", { contract_account_id: ci.accountId }, { gas: "300000000000000" });
  const passed = await evaluator.view('passed_all_exams', { account_id: student.accountId })
  t.is(passed, true);
});
