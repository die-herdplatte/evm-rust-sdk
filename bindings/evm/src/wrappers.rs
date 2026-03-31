extern crate alloc;

use derive_more::{From, Into};
pub use ekubo_sdk::{alloy_primitives::Address as RustAddress, U256 as RustU256};
use js_sys::{BigUint64Array, Uint8Array};
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

#[wasm_bindgen]
#[derive(From, Into)]
pub struct U256(RustU256);

#[wasm_bindgen]
#[derive(From, Into)]
pub struct Address(RustAddress);

#[wasm_bindgen]
impl U256 {
    #[wasm_bindgen(constructor)]
    pub fn new(array: &BigUint64Array) -> Result<Self, JsError> {
        if array.length() as usize != RustU256::LIMBS {
            return Err(JsError::new(&format!(
                "expected {} u64 limbs for U256, got {}",
                RustU256::LIMBS,
                array.length()
            )));
        }

        let mut limbs = [0_u64; RustU256::LIMBS];
        array.copy_to(&mut limbs);

        Ok(Self(RustU256::from_limbs(limbs)))
    }

    #[wasm_bindgen(js_name = clone)]
    pub fn clone_js(&self) -> Self {
        Self(self.0)
    }
}

impl Address {
    const BYTES: usize = 20;
}

#[wasm_bindgen]
impl Address {
    #[wasm_bindgen(constructor)]
    pub fn new(array: &Uint8Array) -> Result<Self, JsError> {
        if array.length() as usize != Self::BYTES {
            return Err(JsError::new(&format!(
                "expected {} bytes, got {}",
                Self::BYTES,
                array.length()
            )));
        }

        let mut bytes = [0_u8; Self::BYTES];
        array.copy_to(&mut bytes);

        Ok(Self(bytes.into()))
    }
}

impl From<&Address> for RustAddress {
    fn from(value: &Address) -> Self {
        value.0
    }
}

impl From<&U256> for RustU256 {
    fn from(value: &U256) -> Self {
        value.0
    }
}
