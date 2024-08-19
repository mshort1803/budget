use super::category::Category;
use super::transaction::Transaction;
use super::currency::Currency;

#[derive(Debug)]
pub struct Budget {
    pub categories: Vec<Category>,
    pub transactions: Vec<Transaction>,
    current_balance: Currency,
    currently_budgetted: Currency,
}

impl Budget {
    pub fn new(starting_balance: f64) -> Self {
	Self {
	    categories: Vec::<Category>::new(),
	    transactions: Vec::<Transaction>::new(),
	    current_balance: Currency::from_dollars(starting_balance),
	    currently_budgetted: Currency::from_dollars(0.00),
	}
    }

    pub fn new_category(&mut self, description: String) {
	let new_category = Category::new(description);
	self.categories.push(new_category);
    }
}
