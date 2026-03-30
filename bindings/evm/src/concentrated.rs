use ekubo_sdk::{
    alloy_primitives::Address,
    chain::evm::{
        EvmConcentratedPool as RustConcentratedPool,
        EvmConcentratedPoolKey as RustConcentratedPoolKey, EvmConcentratedPoolResources,
        EvmConcentratedPoolState,
    },
    quoting::{
        pools::concentrated::TickSpacing,
        types::{Pool, PoolConfig, PoolKey, Tick as RustTick},
    },
};
use itertools::Itertools as _;
use serde::{Deserialize, Serialize};
use tsify::{Ts, Tsify};
use wasm_bindgen::prelude::*;

use crate::wrappers::{Quote, QuoteParams, U256};

#[wasm_bindgen]
pub struct ConcentratedPool(RustConcentratedPool);

#[derive(Tsify, Serialize, Deserialize)]
pub struct ConcentratedPoolConfig {
    /// Extension address.
    pub extension: Address,
    /// Fee tier of the pool.
    pub fee: u64,
    /// Tick spacing for concentrated liquidity.
    pub tick_spacing: u32,
}

#[derive(Tsify, Serialize, Deserialize)]
pub struct ConcentratedPoolKey {
    /// The smaller token address.
    pub token0: Address,
    /// The larger token address.
    pub token1: Address,
    /// Pool configuration.
    pub config: ConcentratedPoolConfig,
}

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
        key: &Ts<ConcentratedPoolKey>,
        state: &Ts<ConcentratedPoolState>,
        sorted_ticks: Vec<Ts<Tick>>,
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

impl From<ConcentratedPoolConfig> for PoolConfig<Address, u64, TickSpacing> {
    fn from(value: ConcentratedPoolConfig) -> Self {
        Self {
            extension: value.extension,
            fee: value.fee,
            pool_type_config: TickSpacing(value.tick_spacing),
        }
    }
}

impl From<ConcentratedPoolKey> for RustConcentratedPoolKey {
    fn from(value: ConcentratedPoolKey) -> Self {
        PoolKey {
            token0: value.token0,
            token1: value.token1,
            config: value.config.into(),
        }
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
