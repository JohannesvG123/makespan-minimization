use std::sync::Arc;

use crate::Algorithm;
use crate::Algorithm::FF;
use crate::global_bounds::bounds::Bounds;
use crate::good_solutions::good_solutions::GoodSolutions;
use crate::input::input::Input;
use crate::output::machine_jobs::MachineJobs;
use crate::output::solution::Solution;
use crate::schedulers::scheduler::Scheduler;

pub struct FFScheduler {
    input: Arc<Input>,
    global_bounds: Arc<Bounds>,
}

impl Scheduler for FFScheduler {
    fn schedule(&mut self, good_solutions: GoodSolutions) -> Solution {
        self.first_fit()
    }

    fn get_algorithm(&self) -> Algorithm {
        FF
    }
}

impl FFScheduler {
    pub fn new(input: Arc<Input>, global_bounds: Arc<Bounds>) -> Self {
        Self { input, global_bounds }
    }

    /// Assigns the biggest job to the machine with the smallest index until all jobs are assigned
    pub fn first_fit(&self) -> Solution {
        println!("running {:?} algorithm...", FF);

        let (upper_bound, lower_bound) = self.global_bounds.get_bounds();
        let machine_count = self.input.get_machine_count();
        let jobs = self.input.get_jobs();

        let mut machine_jobs = MachineJobs::empty(machine_count);

        for job_index in 0..self.input.get_job_count() {
            let mut current_machine: usize = 0;

            if machine_jobs.get_machine_workload(current_machine) + jobs[job_index] > upper_bound {
                current_machine += 1;
                if current_machine == self.input.get_machine_count() { //satisfiability check
                    println!("ERROR: upper bound {} is to low for the {:?}-algorithm with this input", upper_bound, FF);
                    return Solution::unsatisfiable(FF);
                }
            }

            machine_jobs.assign_job(jobs[job_index], current_machine, job_index)
        }

        Solution::new(FF, machine_jobs, self.input.get_jobs(), Arc::clone(&self.global_bounds))
    }
}
