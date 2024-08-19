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
}

#[derive(Debug)]
struct BudgetItem {
    description: String,
    amount_budgetted: f64,
    amount_spent: f64,
}

impl BudgetItem {
    pub fn new(description: String, amount_to_budget: Option<f64>) -> Self {
	Self {
	    description,
	    amount_budgetted: amount_to_budget.expect("Must enter amount to be budgetted"),
	    amount_spent: 0.0,
	}
    }

    fn get_available_to_spend(&self) -> f64 {
	self.amount_budgetted - self.amount_spent
    }
}

