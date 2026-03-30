use derive_more::{From, Into};
pub use ekubo_sdk::{alloy_primitives::Address as RustAddress, U256 as RustU256};
use ekubo_sdk::quoting::types::{Quote as RustQuote, QuoteParams as RustQuoteParams};
use js_sys::BigInt;
use serde::{de, ser, Deserialize, Serialize};
use tsify::{serde_wasm_bindgen, Tsify};

#[derive(Tsify, From, Into)]
pub struct U256(#[tsify(type = "bigint")] RustU256);

#[derive(Tsify, From, Into, Serialize, Deserialize)]
pub struct Address(#[tsify(type = "string")] RustAddress);

#[derive(Tsify, Deserialize)]
pub struct TokenAmount {
    pub token: Address,
    pub amount: i128,
}

#[derive(Tsify, Deserialize)]
pub struct QuoteParams<S, M> {
    /// Token and amount for the swap.
    pub token_amount: TokenAmount,
    /// Optional price limit.
    #[tsify(optional)]
    pub sqrt_ratio_limit: Option<U256>,
    /// Optional override of current pool state.
    #[tsify(optional)]
    pub override_state: Option<S>,
    /// Pool-specific metadata (e.g., timestamp).
    pub meta: M,
}

#[derive(Tsify, Serialize)]
pub struct Quote<R, S> {
    /// Whether price increased during the quote.
    pub is_price_increasing: bool,
    /// Signed amount of input consumed.
    pub consumed_amount: i128,
    /// Unsigned amount of output calculated.
    pub calculated_amount: u128,
    /// Execution resource usage.
    pub execution_resources: R,
    /// Pool state after the simulated swap.
    pub state_after: S,
    /// Fees paid during the swap.
    pub fees_paid: u128,
}

impl Serialize for U256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let bigint: BigInt = self.0.to_string().parse().map_err(|err| {
            ser::Error::custom(format!("failed to deserialize `bigint`: {err:?}"))
        })?;
        serde_wasm_bindgen::preserve::serialize(&bigint, serializer)
    }
}

// Don't use the default `Deserialize` impl which tries to `deserialize_any` which leads to
// `serde_wasm_bindgen::de::Deserializer` trying to deserialize as an i64 or u64
impl<'de> Deserialize<'de> for U256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bigint: BigInt = serde_wasm_bindgen::preserve::deserialize(deserializer)?;
        Ok(Self(ToString::to_string(&bigint).parse().map_err(
            |err| de::Error::custom(format!("failed to deserialize `bigint`: {err}")),
        )?))
    }
}

impl From<TokenAmount> for ekubo_sdk::quoting::types::TokenAmount<RustAddress> {
    fn from(value: TokenAmount) -> Self {
        Self {
            token: value.token.into(),
            amount: value.amount,
        }
    }
}

impl<S1, S2, M1, M2> From<QuoteParams<S1, M1>> for RustQuoteParams<RustAddress, S2, M2>
where
    S1: Into<S2>,
    M1: Into<M2>,
{
    fn from(value: QuoteParams<S1, M1>) -> Self {
        Self {
            token_amount: value.token_amount.into(),
            sqrt_ratio_limit: value.sqrt_ratio_limit.map(Into::into),
            override_state: value.override_state.map(Into::into),
            meta: value.meta.into(),
        }
    }
}

impl<R1, R2, S1, S2> From<RustQuote<R1, S1>> for Quote<R2, S2>
where
    R1: Into<R2>,
    S1: Into<S2>,
{
    fn from(value: RustQuote<R1, S1>) -> Self {
        Self {
            is_price_increasing: value.is_price_increasing,
            consumed_amount: value.consumed_amount,
            calculated_amount: value.calculated_amount,
            execution_resources: value.execution_resources.into(),
            state_after: value.state_after.into(),
            fees_paid: value.fees_paid,
        }
    }
}
