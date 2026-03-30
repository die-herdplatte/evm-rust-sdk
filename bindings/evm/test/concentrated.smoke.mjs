import test from "node:test";
import assert from "node:assert/strict";

import { ConcentratedPool, initPanicHook } from "../pkg/ekubo_sdk_evm_wasm.js";

const TOKEN0 = Uint8Array.from(new Array(20).fill(0));
const TOKEN1 = Uint8Array.from([...new Array(19).fill(0), 1]);
const EXTENSION = Uint8Array.from([...new Array(19).fill(0), 2]);
const SQRT_RATIO_ONE = BigUint64Array.from([0n, 0n, 1n, 0n]);
const SQRT_RATIO_LIMIT = BigUint64Array.from([0n, 0n, 4n, 0n]);

test("constructs concentrated pool and returns a quote", () => {
  initPanicHook();

  const pool = new ConcentratedPool(
    {
      token0: TOKEN0,
      token1: TOKEN1,
      config: {
        extension: EXTENSION,
        fee: 0,
        tick_spacing: 1,
      },
    },
    {
      sqrt_ratio: SQRT_RATIO_ONE,
      liquidity: 1_000_000_000n,
      active_tick_index: 0,
    },
    [
      { index: 0, liquidity_delta: 1_000_000_000n },
      { index: 1, liquidity_delta: -1_000_000_000n },
    ],
  );

  try {
    for (let i = 0; i < 1000000; i++) {
      const quote = pool.quote({
        token_amount: {
          token: TOKEN1,
          amount: 1000n,
        },
        meta: null,
        sqrt_ratio_limit: SQRT_RATIO_LIMIT,
      });

      assert.equal(quote.calculated_amount, 499n);
      assert.equal(quote.execution_resources.initialized_ticks_crossed, 1);
      assert.equal(typeof quote.is_price_increasing, "boolean");
    }
  } finally {
    pool.free();
  }
});
