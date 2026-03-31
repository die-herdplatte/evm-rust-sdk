use ekubo_sdk::quoting::types::{PoolConfig as RustPoolConfig, PoolKey as RustPoolKey};

use crate::wrappers::RustAddress;

mod concentrated;

type PoolKey<C> = RustPoolKey<RustAddress, u64, C>;
type PoolConfig<T> = RustPoolConfig<RustAddress, u64, T>;
