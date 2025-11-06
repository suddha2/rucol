use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Currency {
    pub code: String,
    pub precision: u32,
}

impl Currency {
    pub fn new(code: &str, precision: u32) -> Self {
        Self {
            code: code.to_string(),
            precision,
        }
    }
}

pub type Amount = Decimal;
