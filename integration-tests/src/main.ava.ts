import { Worker, NearAccount, NEAR } from "near-workspaces";
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
  const evaluator = await root.createSubAccount("evaluator");
  await evaluator.deploy(process.argv[2]);
  
  // Student contracts
  const student = await root.createSubAccount("student");
  const helloNear = await student.createSubAccount("hello", {initialBalance: NEAR.parse("1").toString()});
  const guestBook = await student.createSubAccount("guest", {initialBalance: NEAR.parse("1").toString()});

  await helloNear.deploy("./src/aux_contracts/hello_near.wasm");
  await guestBook.deploy("./src/aux_contracts/guest_book.wasm");

  await student.call(evaluator, 'register', {} , { gas: "30000000000000", attachedDeposit: "1" });

  // Save state for test runs, it is unique for each test
  t.context.worker = worker;
  t.context.accounts = { root, evaluator, student, helloNear, guestBook };
});

test.afterEach.always(async (t) => {
  // Stop Sandbox server
  await t.context.worker.tearDown().catch((error) => {
    console.log("Failed to stop the Sandbox:", error);
  });
});

test("Test Hello Near", async (t) => {
  const { evaluator, student, helloNear } = t.context.accounts;
  const result = await student.call(evaluator, helloNear.accountId , { contract_name: helloNear.accountId }, { gas: "30000000000000" });
  t.is(result, true);
});


test("Test GuestBook", async (t) => {
  const { evaluator, student, guestBook } = t.context.accounts;
  const result = await student.call(evaluator, guestBook.accountId, { contract_name: guestBook.accountId }, { gas: "30000000000000" });
  t.is(result, true);
});
