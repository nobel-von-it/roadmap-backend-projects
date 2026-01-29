# Expense Tracker CLI

A command-line tool to manage and track personal expenses. Built with Rust, this tool allows you to add, update, delete, and summarize expenses in a simple and efficient way.

---

## Features

- **Add Expenses**: Add a new expense with an amount and optional description.
- **Update Expenses**: Modify the amount or description of an existing expense.
- **Delete Expenses**: Remove an expense by its ID.
- **Summary**: View the total amount of expenses, optionally filtered by month.
- **List Expenses**: Display all expenses in a tabular format.

---

## Installation

1. Ensure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/).
2. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/expense-tracker.git
   cd expense-tracker
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```

---

## Usage

Run the CLI with one of the following commands:

### Add an Expense
```bash
cargo run -- expense-tracker add --amount <amount> --description <description>
```
Example:
```bash
cargo run -- expense-tracker add --amount 20.5 --description "Lunch"
```

### Update an Expense
```bash
cargo run -- expense-tracker update --id <id> --amount <amount> --description <description>
```
Example:
```bash
cargo run -- expense-tracker update --id 1 --amount 25.0 --description "Dinner"
```

### Delete an Expense
```bash
cargo run -- expense-tracker delete --id <id>
```
Example:
```bash
cargo run -- expense-tracker delete --id 1
```

### Get Summary
```bash
cargo run -- expense-tracker summary --month <month>
```
Example:
```bash
cargo run -- expense-tracker summary --month 8
```

### List All Expenses
```bash
cargo run -- expense-tracker list
```

---

## Example Output

### List Expenses
```
ID  Date         Description  Amount
1   2024-08-06   Lunch        20.5
2   2024-08-06   Dinner       25.0
```

### Summary
```
Total amount of August: 45.5 rub
```

---

## Acknowledgments

- [Rust](https://www.rust-lang.org/) for providing a fast and safe programming language.
- [clap](https://crates.io/crates/clap) for simplifying command-line argument parsing.
- [serde](https://serde.rs/) for JSON serialization and deserialization.
- This project was inspired by the [roadmap.sh](https://roadmap.sh/projects/expense-tracker) project. 
