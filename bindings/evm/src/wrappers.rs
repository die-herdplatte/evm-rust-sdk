use derive_more::{From, Into};
use ekubo_sdk::quoting::types::{Quote as RustQuote, QuoteParams as RustQuoteParams};
pub use ekubo_sdk::{alloy_primitives::Address as RustAddress, U256 as RustU256};
use js_sys::{BigUint64Array, Uint8Array};
use serde::{de, Deserialize, Serialize};
use tsify::{serde_wasm_bindgen, Tsify};

#[derive(Tsify, From, Into)]
pub struct U256(#[tsify(type = "BigUint64Array")] RustU256);

#[derive(Tsify, From, Into)]
pub struct Address(#[tsify(type = "Uint8Array")] RustAddress);

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

impl Address {
    const BYTES: usize = 20;
}

impl Serialize for U256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let limbs = self.0.into_limbs();
        let array = BigUint64Array::from(limbs.as_slice());
        serde_wasm_bindgen::preserve::serialize(&array, serializer)
    }
}

impl<'de> Deserialize<'de> for U256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let array: BigUint64Array = serde_wasm_bindgen::preserve::deserialize(deserializer)?;

        if array.length() as usize != RustU256::LIMBS {
            return Err(de::Error::custom(format!(
                "expected {} u64 limbs for U256, got {}",
                RustU256::LIMBS,
                array.length()
            )));
        }

        let mut limbs = [0_u64; _];
        array.copy_to(&mut limbs);

        Ok(Self(RustU256::from_limbs(limbs)))
    }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let bytes = self.0.into_array();
        let array = Uint8Array::from(bytes.as_slice());
        serde_wasm_bindgen::preserve::serialize(&array, serializer)
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let array: Uint8Array = serde_wasm_bindgen::preserve::deserialize(deserializer)?;

        if array.length() as usize != Self::BYTES {
            return Err(de::Error::custom(format!(
                "expected {} bytes, got {}",
                Self::BYTES,
                array.length()
            )));
        }

        let mut bytes = [0_u8; _];
        array.copy_to(&mut bytes);

        Ok(Self(bytes.into()))
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
