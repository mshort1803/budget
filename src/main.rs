mod budget;
mod category;
mod transaction;
mod currency;

use budget::Budget;
use category::Category;
use transaction::Transaction;
use currency::Currency;

fn main() {
    let mut my_budget = Budget::new(50.00);

    println!("{:#?}", my_budget);

    my_budget.new_category("Essentials".to_owned());
    my_budget.categories[0].new_budget_item(String::from("Electricity"), 50.00);
    
    println!("{:#?}", my_budget);
    

    let cents = Currency::new(5000);
    let amount = cents.convert_to_dollars();
    println!("amount in dollars and cents: {:.2}", amount);
    println!("amount in cents: {:#?}", Currency::from_dollars(5.00));
}


