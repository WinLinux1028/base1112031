use std::str::FromStr;

use base1112031::{FromBase1112031, ToBase1112031};
use num_bigint::BigUint;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn to_base1112031(input: &str) -> Option<String> {
    let input = BigUint::from_str(input).ok()?;
    input.to_base1112031()
}

#[wasm_bindgen]
pub fn from_base1112031(input: &str) -> Option<String> {
    let result: BigUint = FromBase1112031::from_base1112031(input)?;
    Some(result.to_string())
}
