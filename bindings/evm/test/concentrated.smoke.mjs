import test from "node:test";
import assert from "node:assert/strict";

import { ConcentratedPool, initPanicHook } from "../pkg/ekubo_sdk_evm_wasm.js";

const TOKEN0 = "0x0000000000000000000000000000000000000000";
const TOKEN1 = "0x0000000000000000000000000000000000000001";
const EXTENSION = "0x0000000000000000000000000000000000000002";
const SQRT_RATIO_ONE = 1n << 128n;

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
        sqrt_ratio_limit: 1n << 130n,
      });

      assert.equal(quote.calculated_amount, 499n);
      assert.equal(quote.execution_resources.initialized_ticks_crossed, 1);
      assert.equal(typeof quote.is_price_increasing, "boolean");
    }
  } finally {
    pool.free();
  }
});
