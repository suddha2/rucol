use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub account_type: AccountType,
}

impl Account {
    pub fn new(name: &str, account_type: AccountType) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            account_type,
        }
    }
}
