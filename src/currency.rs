use std::ops::Sub;

#[derive(Debug, Copy, Clone)]
pub struct Currency {
    amount_in_cents: i32,
}

impl Sub for Currency {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
	Self {
	    amount_in_cents: &self.amount_in_cents - &other.amount_in_cents,
	}
    }
}

impl Currency {
    pub fn new(cents: i32) -> Self {
	Self {
	    amount_in_cents: cents,
	}
    }

    pub fn from_dollars(dollars: f64) -> Self {
	Self {
	    amount_in_cents: (dollars * 100.0) as i32,
	} 
    }
    
    pub fn convert_to_dollars(&self) -> f64 {
	let cents_64:f64 = self.amount_in_cents as f64;
	let amount = cents_64 / 100.00;
	amount
    }

}
