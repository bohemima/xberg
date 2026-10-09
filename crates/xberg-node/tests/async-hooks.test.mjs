import assert from "node:assert/strict";
import { AsyncLocalStorage, createHook } from "node:async_hooks";
import test from "node:test";

import xberg from "../index.js";

function textInput() {
  return {
    kind: xberg.ExtractInputKind.Bytes,
    bytes: new TextEncoder().encode("hi"),
    filename: "a.txt",
    mimeType: "text/plain",
  };
}

async function assertExtractsText() {
  const result = await xberg.extract(textInput());
  assert.equal(result.results?.length, 1);
  assert.equal(result.results[0].content, "hi");
}

async function assertBatchExtractsText() {
  const result = await xberg.extractBatch([textInput(), textInput()]);
  assert.equal(result.results?.length, 2);
  assert.deepEqual(
    result.results.map(({ content }) => content),
    ["hi", "hi"],
  );
}

test("extract should work inside AsyncLocalStorage", async () => {
  const storage = new AsyncLocalStorage();
  await storage.run({ requestId: "request-1" }, assertExtractsText);
});

test("extractBatch should work inside AsyncLocalStorage", async () => {
  const storage = new AsyncLocalStorage();
  await storage.run({ requestId: "request-1" }, assertBatchExtractsText);
});

test("extract should work while a bare async hook is enabled", async () => {
  const hook = createHook({ init() {} });
  hook.enable();
  try {
    await assertExtractsText();
  } finally {
    hook.disable();
  }
});

test("extractBatch should work while a bare async hook is enabled", async () => {
  const hook = createHook({ init() {} });
  hook.enable();
  try {
    await assertBatchExtractsText();
  } finally {
    hook.disable();
  }
});
