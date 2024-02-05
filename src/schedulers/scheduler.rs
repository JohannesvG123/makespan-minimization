use std::sync::Arc;
use std::time::Instant;

use permutation::Permutation;

use crate::{Algorithm, Args};
use crate::good_solutions::good_solutions::GoodSolutions;
use crate::output::solution::Solution;

pub trait Scheduler {
    fn schedule(&mut self, good_solutions: GoodSolutions, args: Arc<Args>, perm: Arc<Permutation>, start_time: Instant) -> Solution;
    fn get_algorithm(&self) -> Algorithm;
}