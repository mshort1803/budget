use super::category::{Category, BudgetItem};

#[derive(Debug)]
pub struct Transaction {
    description: String,
    budget_item: BudgetItem,
    transaction_type: TransactionType,
    amount: f64,
}

#[derive(Debug)]
enum TransactionType {
    Income { from: String },
    Expense { to: String }
}
