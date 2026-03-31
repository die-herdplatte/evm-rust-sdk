# ekubo_sdk EVM TypeScript bindings

This package exposes EVM pool quote functions from `ekubo_sdk` via WASM.

## Build

```bash
wasm-pack build --target bundler --release --out-dir pkg
```

## Exported JS API

- `initPanicHook()`
- `quoteConcentratedPool(pool, params)`
- `quoteFullRangePool(pool, params)`
- `quoteStableswapPool(pool, params)`
- `quoteMevCapturePool(pool, params)`
- `quoteOraclePool(pool, params)`
- `quoteTwammPool(pool, params)`
- `quoteBoostedFeesConcentratedPool(pool, params)`
- `quoteBoostedFeesFullRangePool(pool, params)`
- `quoteBoostedFeesStableswapPool(pool, params)`

All functions take JS objects compatible with the corresponding Rust `serde` shapes and return a JS object for `Quote<Resources, State>`.

For EVM types in these bindings:
- `Address` is encoded as a `Uint8Array` of 20 bytes.
- `U256` is encoded as `BigUint64Array` (or `ArrayBuffer`) with 4 little-endian `u64` limbs.
