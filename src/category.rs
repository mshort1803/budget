use super::currency::Currency;

#[derive(Debug)]
pub struct Category {
    description: String,
    budget_items: Vec<BudgetItem>
}

impl Category {
    pub fn new(description: String ) -> Self {
	Self {
	    description,
	    budget_items: Vec::<BudgetItem>::new(),
	}
    }

    pub fn new_budget_item(&mut self, description: String, amount_budgetted: f64) {
	let new_budget_item = BudgetItem::new(description, amount_budgetted);
	self.budget_items.push(new_budget_item);
    }
}

#[derive(Debug)]
pub struct BudgetItem {
    description: String,
    amount_budgetted: Currency,
    amount_spent: Currency,
}

impl BudgetItem {
    pub fn new(description: String, amount_to_budget: f64) -> Self {
	Self {
	    description,
	    amount_budgetted: Currency::from_dollars(amount_to_budget),
	    amount_spent: Currency::from_dollars(0.00),
	}
    }

    pub fn get_available_to_spend(&self) -> Currency {
	self.amount_budgetted - self.amount_spent
    }
}

