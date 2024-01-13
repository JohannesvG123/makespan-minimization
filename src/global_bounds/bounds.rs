use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::input::input::Input;

pub struct Bounds {
    upper_bound: AtomicU32,
    lower_bound: AtomicU32,
}

impl Bounds {
    pub fn new(upper_bound: u32, lower_bound: u32) -> Self {
        Self {
            upper_bound: AtomicU32::new(upper_bound),
            lower_bound: AtomicU32::new(lower_bound),
        }
    }

    pub fn trivial(input: Arc<Input>) -> Self {
        let upper_bound = input.get_jobs().iter().sum::<u32>() / input.get_machine_count() as u32 + input.get_jobs().iter().max().unwrap();
        let lower_bound = 0; //TODO besseren verwenden
        Self::new(upper_bound, lower_bound)
    }

    /// returns (upper_bound, lower_bound)
    pub fn get_bounds(&self) -> (u32, u32) {
        (self.get_upper_bound(), self.get_lower_bound())
    }

    pub fn get_upper_bound(&self) -> u32 {
        self.upper_bound.load(Ordering::Relaxed) //TODO Ordering checken überall!!! (docs)
    }

    pub fn get_lower_bound(&self) -> u32 {
        self.lower_bound.load(Ordering::Relaxed)
    }

    pub fn set_upper_bound(&self, upper_bound: u32) {
        self.upper_bound.store(upper_bound, Ordering::Release)
    }

    pub fn set_lower_bound(&self, lower_bound: u32) {
        self.lower_bound.store(lower_bound, Ordering::Release)
    }

    pub fn update_bounds(&self, new_upper_bound: u32, new_lower_bound: u32) {
        self.update_upper_bound(new_upper_bound);
        self.update_lower_bound(new_lower_bound);
    }

    pub fn update_upper_bound(&self, new_upper_bound: u32) {
        self.upper_bound.fetch_min(new_upper_bound, Ordering::SeqCst);
    }

    pub fn update_lower_bound(&self, new_lower_bound: u32) {
        self.upper_bound.fetch_max(new_lower_bound, Ordering::SeqCst);
    }
}