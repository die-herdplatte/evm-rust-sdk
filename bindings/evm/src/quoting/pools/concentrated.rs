use derive_more::{From, Into};
use ekubo_sdk::{
    chain::evm::{
        EvmConcentratedPool as RustConcentratedPool,
        EvmConcentratedPoolResources as RustConcentratedPoolResources,
        EvmConcentratedPoolState as RustConcentratedPoolState,
    },
    quoting::{
        pools::concentrated::{ConcentratedPoolTypeConfig, TickSpacing as RustTickSpacing},
        types::{
            Pool as _, Quote as RustQuote, QuoteParams as RustQuoteParams, Tick as RustTick,
            TokenAmount,
        },
    },
};
use js_sys::BigUint64Array;
use wasm_bindgen::prelude::*;

use crate::{
    quoting::pools::{PoolConfig, PoolKey},
    wrappers::{Address, U256},
};

#[wasm_bindgen]
pub struct ConcentratedPool(RustConcentratedPool);

#[wasm_bindgen]
#[derive(From)]
pub struct ConcentratedPoolQuote(
    RustQuote<RustConcentratedPoolResources, RustConcentratedPoolState>,
);

#[wasm_bindgen]
pub struct ConcentratedPoolKey(PoolKey<ConcentratedPoolTypeConfig>);

#[wasm_bindgen]
#[derive(Into)]
pub struct ConcentratedPoolState(RustConcentratedPoolState);

#[wasm_bindgen]
pub struct ConcentratedPoolResources(RustConcentratedPoolResources);

#[wasm_bindgen]
pub struct Tick(RustTick);

#[wasm_bindgen]
impl ConcentratedPoolKey {
    #[wasm_bindgen(constructor)]
    pub fn new(
        token0: &Address,
        token1: &Address,
        extension: &Address,
        fee: u64,
        tick_spacing: u32,
    ) -> Result<Self, JsError> {
        Ok(Self(PoolKey {
            token0: token0.into(),
            token1: token1.into(),
            config: PoolConfig {
                extension: extension.into(),
                fee,
                pool_type_config: RustTickSpacing(tick_spacing),
            },
        }))
    }
}

#[wasm_bindgen]
impl ConcentratedPoolResources {
    #[wasm_bindgen(getter)]
    pub fn no_override_price_change(&self) -> u32 {
        self.0.no_override_price_change
    }

    #[wasm_bindgen(getter)]
    pub fn extra_distinct_bitmap_lookups(&self) -> u32 {
        self.0.extra_distinct_bitmap_lookups
    }

    #[wasm_bindgen(getter)]
    pub fn initialized_ticks_crossed(&self) -> u32 {
        self.0.initialized_ticks_crossed
    }
}

#[wasm_bindgen]
impl ConcentratedPool {
    #[wasm_bindgen(constructor)]
    pub fn new(
        key: ConcentratedPoolKey,
        state: ConcentratedPoolState,
        sorted_ticks: Box<[Tick]>,
    ) -> Result<ConcentratedPool, JsError> {
        Ok(Self(RustConcentratedPool::new(
            key.0,
            state.0,
            IntoIterator::into_iter(sorted_ticks).map(|t| t.0).collect(),
        )?))
    }

    pub fn quote(
        &self,
        token: &Address,
        amount: i128,
        sqrt_ratio_limit: Option<U256>, // TODO Should be a reference once https://github.com/wasm-bindgen/wasm-bindgen/issues/2370 is resolved
        override_state: Option<ConcentratedPoolState>,
    ) -> Result<ConcentratedPoolQuote, JsError> {
        let quote = self.0.quote(RustQuoteParams {
            token_amount: TokenAmount {
                token: token.into(),
                amount,
            },
            sqrt_ratio_limit: sqrt_ratio_limit.map(Into::into),
            override_state: override_state.map(Into::into),
            meta: (),
        })?;

        Ok(ConcentratedPoolQuote(quote))
    }
}

#[wasm_bindgen]
impl ConcentratedPoolState {
    #[wasm_bindgen(constructor)]
    pub fn new(
        sqrt_ratio: U256,
        liquidity: u128,
        active_tick_index: Option<u32>,
    ) -> Result<Self, JsError> {
        Ok(Self(RustConcentratedPoolState {
            sqrt_ratio: sqrt_ratio.into(),
            liquidity,
            active_tick_index: active_tick_index.map(|index| index as usize),
        }))
    }

    #[wasm_bindgen(getter)]
    pub fn sqrt_ratio(&self) -> BigUint64Array {
        let limbs = self.0.sqrt_ratio.into_limbs();
        BigUint64Array::from(limbs.as_slice())
    }

    #[wasm_bindgen(getter)]
    pub fn liquidity(&self) -> u128 {
        self.0.liquidity
    }

    #[wasm_bindgen(getter)]
    pub fn active_tick_index(&self) -> Option<u32> {
        self.0.active_tick_index.map(|index| index as u32)
    }

    #[wasm_bindgen(js_name = clone)]
    pub fn clone_js(&self) -> Self {
        Self(self.0)
    }
}

#[wasm_bindgen]
impl ConcentratedPoolQuote {
    #[wasm_bindgen(getter)]
    pub fn is_price_increasing(&self) -> bool {
        self.0.is_price_increasing
    }

    #[wasm_bindgen(getter)]
    pub fn consumed_amount(&self) -> i128 {
        self.0.consumed_amount
    }

    #[wasm_bindgen(getter)]
    pub fn calculated_amount(&self) -> u128 {
        self.0.calculated_amount
    }

    #[wasm_bindgen(getter)]
    pub fn fees_paid(&self) -> u128 {
        self.0.fees_paid
    }

    #[wasm_bindgen(getter)]
    pub fn execution_resources(&self) -> ConcentratedPoolResources {
        ConcentratedPoolResources(self.0.execution_resources)
    }

    #[wasm_bindgen(getter)]
    pub fn state_after(&self) -> ConcentratedPoolState {
        ConcentratedPoolState(self.0.state_after)
    }
}

#[wasm_bindgen]
impl Tick {
    #[wasm_bindgen(constructor)]
    pub fn new(index: i32, liquidity_delta: i128) -> Self {
        Self(RustTick {
            index,
            liquidity_delta,
        })
    }
}
