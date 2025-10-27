# 🧮 Rust Accounting Core

A modular, high-assurance accounting engine written in **Rust**, designed to demonstrate how a modern ledger system could be built from first principles — inspired by **Xero** and **Manager.io**.

This project showcases architectural thinking, domain-driven design, and financial correctness at the systems level.

---

## ⚙️ Core Features

* 🧾 **Double-Entry Ledger** — enforces debit = credit at journal level
* 💼 **Multi-Company Support** — isolate books per company
* 🧱 **Chart of Accounts** — hierarchical, typed, currency-aware
* 📅 **Fiscal Period Management** — open/close periods safely
* 💰 **Multi-Currency FX Handling** — rate tracking and conversions
* 📊 **Financial Reports** — Trial Balance, P&L, Balance Sheet
* 🧮 **Tax Codes** — configurable tax rates and reporting
* 🕵️ **Audit Trail** — immutable activity logging for compliance

---

## 🏗️ Module Architecture

```
ledger_core/
├── company/       # Multi-company and fiscal configuration
├── account/       # Chart of accounts and account types
├── journal/       # Double-entry journal validation
├── transaction/   # Payments, receipts, transfers
├── period/        # Fiscal period control
├── report/        # Trial Balance, P&L, Balance Sheet
├── tax/           # Tax codes and calculation
├── audit/         # Audit trail and history
├── currency/      # FX management
└── utils/         # Shared decimal/date/error helpers
```

Each module owns its own data structures and business rules, enabling isolated testing and long-term scalability.

---

## 🧩 Example: Adding a Journal Entry

```rust
use ledger_core::journal::{JournalEntry, TransactionLine};
use ledger_core::currency::Currency;
use rust_decimal_macros::dec;

let entry = JournalEntry::new(
    1, "Invoice Payment",
    vec![
        TransactionLine::debit(1001, dec!(500.00), Currency::USD),
        TransactionLine::credit(2001, dec!(500.00), Currency::USD),
    ]
)?;

ledger_core::journal::add_journal_entry(entry)?;
```

✅ Validates debits = credits
✅ Ensures open period
✅ Automatically posts to ledger accounts

---

## 🧠 Design Principles

* **Deterministic Accounting:** Ledger state is derived from journal entries — no mutable balances.
* **Safety via Rust:** Compile-time guarantees and strong typing prevent category errors.
* **Composability:** Each module can be replaced or extended independently.
* **Auditability:** Every mutation produces an audit log record.

---
---

## 🧩 System Architecture Overview

This diagram illustrates how the React UI, Elixir API, and Rust Core interact with PostgreSQL in the accounting engine:

![System Architecture](A_flowchart_diagram_depicts_a_software_architectur.png)

Data flows downward:
- **React UI** — handles user interactions and displays reports  
- **Elixir API** — orchestrates requests, authentication, and concurrency  
- **Rust Core** — performs validated accounting logic (journal, ledger, reports)  
- **PostgreSQL** — stores all persistent financial and configuration data  

---

## 🧭 Tech Stack

| Layer              | Technology                | Purpose                             |
| ------------------ | ------------------------- | ----------------------------------- |
| Core               | **Rust**                  | Ledger logic, validation, reporting |
| API (planned)      | **Elixir / gRPC**         | Multi-user orchestration            |
| Frontend (planned) | **React + TypeScript**    | UI, dashboards                      |
| Database           | **PostgreSQL (via sqlx)** | Persistent storage                  |

---

## 🚀 Future Roadmap

* [ ] CLI demo for journal entry + trial balance
* [ ] REST / gRPC service layer
* [ ] React dashboard with live reports
* [ ] Integration tests with property-based validation

---

## 📄 License

MIT – Use freely, with attribution appreciated.

---

## 🙋 About the Author

**Sudha**
Systems-minded developer focused on reliable, domain-driven software.
Built this project to explore how financial cores can be designed for correctness, performance, and long-term auditability.
