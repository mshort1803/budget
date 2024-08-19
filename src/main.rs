mod budget;
mod category;
mod transaction;

use budget::Budget;
use category::Category;
use transaction::Transaction;

fn main() {
    let mut my_budget = Budget::new(50.0);

    println!("{:#?}", my_budget);

    my_budget.new_category("Essentials".to_owned());
    println!("{:#?}", my_budget);
}

