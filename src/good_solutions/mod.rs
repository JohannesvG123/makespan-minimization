use std::sync::{Arc, Mutex};

use crate::good_solutions::good_solutions::GoodSolutions;

pub mod good_solutions;

pub fn create_good_solutions(max_capacity: usize) -> Arc<Mutex<GoodSolutions>> {
    Arc::new(Mutex::new(GoodSolutions::new(max_capacity)))
}
