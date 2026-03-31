use ekubo_sdk::{
    alloy_primitives::Address,
    quoting::types::{PoolConfig as RustPoolConfig, PoolKey as RustPoolKey},
};
use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::wrappers::RustAddress;

mod concentrated;

#[derive(Tsify, Serialize, Deserialize)]
pub struct PoolKey<T> {
    /// The smaller token address.
    pub token0: Address,
    /// The larger token address.
    pub token1: Address,
    /// Pool configuration.
    pub config: PoolConfig<T>,
}

#[derive(Tsify, Serialize, Deserialize)]
pub struct PoolConfig<T> {
    /// Extension address.
    pub extension: Address,
    /// Fee tier of the pool.
    pub fee: u64,
    /// Pool-type specific configuration.
    pub pool_type_config: T,
}

impl<T1, T2> From<PoolKey<T1>> for RustPoolKey<RustAddress, u64, T2>
where
    PoolConfig<T1>: Into<RustPoolConfig<RustAddress, u64, T2>>,
{
    fn from(value: PoolKey<T1>) -> Self {
        Self {
            token0: value.token0,
            token1: value.token1,
            config: value.config.into(),
        }
    }
}

impl<T1, T2> From<PoolConfig<T1>> for RustPoolConfig<RustAddress, u64, T2>
where
    T1: Into<T2>,
{
    fn from(value: PoolConfig<T1>) -> Self {
        Self {
            extension: value.extension,
            fee: value.fee,
            pool_type_config: value.pool_type_config.into(),
        }
    }
}
