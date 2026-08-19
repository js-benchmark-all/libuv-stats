import { it, describe } from "node:test";

import assert from "node:assert";
import { once } from "node:events";
import { Worker } from "node:worker_threads";

const tests = {
  "lastLoopIters()": "./tests/last-loop-iters.ts",
};

for (const key in tests) {
  const path = tests[key as keyof typeof tests];

  describe(key, () => {
    it("main thread", async () => {
      await import(path);
    });

    it("worker thread", async () => {
      const worker = new Worker(new URL(path, import.meta.url));

      try {
        const [code] = await once(worker, "exit");
        assert.strictEqual(code, 0);
      } finally {
        await worker.terminate();
      }
    });
  });
}
