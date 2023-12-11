use crate::output::solution::Solution;

pub trait Scheduler {
    fn schedule(&mut self) -> Solution;
}