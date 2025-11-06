use rucol_core::account::Account;
use rucol_core::account::AccountType;
use rucol_core::journal::JournalEntry;
use rucol_core::ledger::Ledger;
use rucol_core::transaction::{Transaction, TransactionLine};

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use rust_decimal::Decimal;

    #[test]
    fn test_ledger_balance() {
        let mut ledger = Ledger::new();

        let cash = Account::new("Cash", AccountType::Asset);
        let revenue = Account::new("Revenue", AccountType::Revenue);

        let cash_id = cash.id;
        let revenue_id = revenue.id;

        ledger.add_account(cash);
        ledger.add_account(revenue);

        let txn = Transaction::new(
            vec![
                TransactionLine {
                    account_id: cash_id,
                    amount: Decimal::new(10000, 2), // $100.00
                },
                TransactionLine {
                    account_id: revenue_id,
                    amount: Decimal::new(-10000, 2), // -$100.00
                },
            ],
            Some("Service revenue".to_string()),
        );

        let entry = JournalEntry::new(txn, NaiveDate::from_ymd_opt(2024, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap());

        let result = ledger.post_entry(entry);

        assert!(result.is_ok());
        assert_eq!(
            ledger.get_account_balance(cash_id).unwrap(),
            Decimal::new(10000, 2)
        );
        assert_eq!(
            ledger.get_account_balance(revenue_id).unwrap(),
            Decimal::new(-10000, 2)
        );
    }
}