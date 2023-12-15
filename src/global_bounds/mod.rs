use std::sync::{Arc, Mutex};

use crate::global_bounds::bounds::Bounds;
use crate::input::input::Input;

pub mod bounds;

pub fn create_global_bounds(input: Arc<Input>) -> Arc<Mutex<Bounds>> {
    Arc::new(Mutex::new(Bounds::trivial(input)))
}

pub fn update_upper_bound(global_bounds: Arc<Mutex<Bounds>>, found_upper_bound: u32) {
    let mut global_bounds = global_bounds.lock().unwrap();

    if global_bounds.get_upper_bound() > found_upper_bound {
        global_bounds.set_upper_bound(found_upper_bound);
    }
}

