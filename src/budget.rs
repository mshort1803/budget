use super::category::Category;
use super::transaction::Transaction;

#[derive(Debug)]
pub struct Budget {
    categories: Vec<Category>,
    transactions: Vec<Transaction>,
    current_balance: f64,
    currently_budgetted: f64,
}

impl Budget {
    pub fn new(starting_balance: f64) -> Self {
	Self {
	    categories: Vec::<Category>::new(),
	    transactions: Vec::<Transaction>::new(),
	    current_balance: starting_balance,
	    currently_budgetted: 0.0,
	}
    }

    pub fn new_category(&mut self, description: String) {
	let new_category = Category::new(description);
	self.categories.push(new_category);
    }
}
