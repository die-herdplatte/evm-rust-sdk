test:
    cargo test --features serde,evm,starknet

check:
    cargo check --features serde,evm,starknet

check-bindings:
    cargo check --manifest-path bindings/evm/Cargo.toml
    cargo check --manifest-path bindings/starknet/Cargo.toml

build-ts-evm:
    wasm-pack build bindings/evm --target bundler --release --out-dir pkg

build-ts-starknet:
    wasm-pack build bindings/starknet --target bundler --release --out-dir pkg

build-ts:
    just build-ts-evm
    just build-ts-starknet
