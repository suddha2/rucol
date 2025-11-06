use std::collections::HashMap;

use crate::{account::Account, currency::Amount, journal::JournalEntry, LedgerError};

pub struct Ledger {
    accounts: HashMap<uuid::Uuid, Account>,
    entries: Vec<JournalEntry>,
}

impl Ledger {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            entries: Vec::new(),
        }
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.insert(account.id, account);
    }

    pub fn post_entry(&mut self, entry: JournalEntry) -> Result<(), LedgerError> {
        if !entry.transaction.is_balanced() {
            return Err(LedgerError::UnbalancedTransaction);
        }

        self.entries.push(entry);
        Ok(())
    }

    pub fn get_account_balance(&self, account_id: uuid::Uuid) -> Result<Amount, LedgerError> {
        let mut balance = Amount::ZERO;

        for entry in &self.entries {
            for line in &entry.transaction.lines {
                if line.account_id == account_id {
                    balance += line.amount;
                }
            }
        }

        Ok(balance)
    }
}