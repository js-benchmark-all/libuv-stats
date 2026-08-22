Get `libuv` stats.

```ts
import { track, untrack, lastLoopIters } from "libuv-stats";
import { setImmediate } from "node:timers/promises";

// Register callbacks to track stats
track();

console.log(lastLoopIters()); // 0

await setImmediate();
console.log(lastLoopIters()); // 1

await setImmediate();
await setImmediate();
console.log(lastLoopIters()); // 2

// Unregister all callbacks to track stats
untrack();
```

To reset all state:

```ts
track();
await setImmediate();

// Reset tracked state
track();
console.log(lastLoopIters()); // 0
```
