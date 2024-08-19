#[derive(Debug)]
pub struct Transaction {
    description: String,
    transaction_type: TransactionType,
    amount: f64,
}

#[derive(Debug)]
enum TransactionType {
    Income { from: String },
    Expense { to: String }
}
