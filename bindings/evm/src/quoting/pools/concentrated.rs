use derive_more::Into;
use ekubo_sdk::{
    chain::evm::{
        EvmConcentratedPool as RustConcentratedPool, EvmConcentratedPoolResources,
        EvmConcentratedPoolState,
    },
    quoting::{
        pools::concentrated::TickSpacing as RustTickSpacing,
        types::{Pool as _, Tick as RustTick},
    },
};
use itertools::Itertools as _;
use serde::{Deserialize, Serialize};
use tsify::{Ts, Tsify};
use wasm_bindgen::prelude::*;

use crate::{
    quoting::pools::PoolKey,
    wrappers::{Quote, QuoteParams, U256},
};

#[wasm_bindgen]
pub struct ConcentratedPool(RustConcentratedPool);

#[derive(Tsify, Deserialize, Into)]
pub struct ConcentratedPoolTypeConfig(#[tsify(type = "number")] RustTickSpacing);

#[derive(Tsify, Serialize, Deserialize)]
pub struct Tick {
    /// Tick index where liquidity delta applies.
    pub index: i32,
    /// Liquidity change applied at this tick.
    pub liquidity_delta: i128,
}

#[derive(Tsify, Serialize, Deserialize)]
pub struct ConcentratedPoolState {
    /// Current square root price ratio.
    pub sqrt_ratio: U256,
    /// Active liquidity at the current price.
    pub liquidity: u128,
    /// Index of the active initialized tick, if any.
    pub active_tick_index: Option<usize>,
}

#[derive(Tsify, Serialize, Deserialize)]
pub struct ConcentratedPoolResources {
    /// Whether price changed when no override was provided.
    pub no_override_price_change: u32,
    /// Count of initialized ticks crossed during the quote.
    pub initialized_ticks_crossed: u32,
    /// Number of additional distinct tick bitmap lookups (besides the mandatory one).
    pub extra_distinct_bitmap_lookups: u32,
}

#[wasm_bindgen]
impl ConcentratedPool {
    #[wasm_bindgen(constructor)]
    pub fn new(
        #[wasm_bindgen(unchecked_param_type = "PoolKey<ConcentratedPoolTypeConfig>")] key: &Ts<
            PoolKey<ConcentratedPoolTypeConfig>,
        >,
        state: &Ts<ConcentratedPoolState>,
        sorted_ticks: Box<[Ts<Tick>]>,
    ) -> Result<ConcentratedPool, JsError> {
        Ok(Self(RustConcentratedPool::new(
            key.to_rust()?.into(),
            state.to_rust()?.into(),
            sorted_ticks
                .into_iter()
                .map(|t| t.to_rust().map(Into::into))
                .try_collect()?,
        )?))
    }

    #[wasm_bindgen(
        unchecked_return_type = "Quote<ConcentratedPoolResources, ConcentratedPoolState>"
    )]
    pub fn quote(
        &self,
        #[wasm_bindgen(unchecked_param_type = "QuoteParams<ConcentratedPoolState, null>")]
        params: &Ts<QuoteParams<ConcentratedPoolState, ()>>,
    ) -> Result<Ts<Quote<ConcentratedPoolResources, ConcentratedPoolState>>, JsError> {
        let quote = self.0.quote(params.to_rust()?.into())?;
        Ok(Quote::from(quote).into_ts()?)
    }
}

impl From<Tick> for RustTick {
    fn from(value: Tick) -> Self {
        Self {
            index: value.index,
            liquidity_delta: value.liquidity_delta,
        }
    }
}

impl From<RustTick> for Tick {
    fn from(value: RustTick) -> Self {
        Self {
            index: value.index,
            liquidity_delta: value.liquidity_delta,
        }
    }
}

impl From<ConcentratedPoolState> for EvmConcentratedPoolState {
    fn from(value: ConcentratedPoolState) -> Self {
        Self {
            sqrt_ratio: value.sqrt_ratio.into(),
            liquidity: value.liquidity,
            active_tick_index: value.active_tick_index,
        }
    }
}

impl From<EvmConcentratedPoolState> for ConcentratedPoolState {
    fn from(value: EvmConcentratedPoolState) -> Self {
        Self {
            sqrt_ratio: value.sqrt_ratio.into(),
            liquidity: value.liquidity,
            active_tick_index: value.active_tick_index,
        }
    }
}

impl From<EvmConcentratedPoolResources> for ConcentratedPoolResources {
    fn from(value: EvmConcentratedPoolResources) -> Self {
        Self {
            no_override_price_change: value.no_override_price_change,
            initialized_ticks_crossed: value.initialized_ticks_crossed,
            extra_distinct_bitmap_lookups: value.extra_distinct_bitmap_lookups,
        }
    }
}
