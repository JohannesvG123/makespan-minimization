use crate::Algorithm;
use crate::good_solutions::good_solutions::GoodSolutions;
use crate::output::solution::Solution;

pub trait Scheduler {
    fn schedule(&mut self, good_solutions: GoodSolutions) -> Solution;
    fn get_algorithm(&self) -> Algorithm;
}