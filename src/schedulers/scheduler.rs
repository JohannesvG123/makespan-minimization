use crate::Algorithm;
use crate::output::solution::Solution;

pub trait Scheduler {
    fn schedule(&mut self) -> Solution;
    fn get_algorithm(&self) -> Algorithm;
}