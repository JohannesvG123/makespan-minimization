use std::rc::Rc;

use crate::Algorithm::{LPT, RR};
use crate::input::sorted_input::SortedInput;
use crate::output::Solution;
use crate::scheduler::{Scheduler, SchedulingObjects};

/// Schedulers using algorithms from the LS (List Scheduling family) to solve the makespan-minimization problem

//TODO DESIGN-FRAGE

pub struct LPTScheduler {
    scheduling_objects: SchedulingObjects,
}

impl LPTScheduler {
    pub fn new(input: Rc<SortedInput>, upper_bound: Option<u32>) -> Self {
        LPTScheduler { scheduling_objects: SchedulingObjects::new(input, upper_bound, RR) }
    }
}

impl Scheduler for LPTScheduler {
    fn schedule(&mut self) -> Solution {
        Solution::unsatisfiable(LPT)
    }
}

