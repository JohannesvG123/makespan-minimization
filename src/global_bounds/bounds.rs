use std::sync::Arc;

use crate::input::input::Input;

pub struct Bounds {
    upper_bound: u32,
    lower_bound: u32,
}

impl Bounds {
    pub fn new(upper_bound: u32, lower_bound: u32) -> Self {
        Self { upper_bound, lower_bound }
    }

    pub fn trivial(input: Arc<Input>) -> Self {
        let upper_bound = input.get_jobs().iter().sum::<u32>() / input.get_machine_count() as u32 + input.get_jobs().iter().max().unwrap();
        let lower_bound = 0; //TODO besseren verwenden(?)
        Self { upper_bound, lower_bound }
    }

    /// returns (upper_bound, lower_bound)
    pub fn get_bounds(&self) -> (u32, u32) {
        (self.upper_bound, self.lower_bound)
    }

    pub fn get_upper_bound(&self) -> u32 {
        self.upper_bound
    }

    pub fn get_lower_bound(&self) -> u32 {
        self.lower_bound
    }

    pub fn set_upper_bound(&mut self, upper_bound: u32) {
        self.upper_bound = upper_bound;
    }

    pub fn set_lower_bound(&mut self, lower_bound: u32) {
        self.lower_bound = lower_bound;
    }
}