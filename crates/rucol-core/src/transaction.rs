use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::currency::Amount;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionLine {
    pub account_id: Uuid,
    pub amount: Amount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub lines: Vec<TransactionLine>,
    pub description: Option<String>,
}

impl Transaction {
    pub fn new(lines: Vec<TransactionLine>, description: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            lines,
            description,
        }
    }

    pub fn is_balanced(&self) -> bool {
        let sum: Amount = self.lines.iter().map(|l| l.amount).sum();
        sum.is_zero()
    }
}
