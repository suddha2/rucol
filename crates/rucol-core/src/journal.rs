use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::transaction::Transaction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub id: Uuid,
    pub transaction: Transaction,
    pub timestamp: chrono::NaiveDateTime,
}

impl JournalEntry {
    pub fn new(transaction: Transaction, timestamp: chrono::NaiveDateTime) -> Self {
        Self {
            id: Uuid::new_v4(),
            transaction,
            timestamp,
        }
    }
}
