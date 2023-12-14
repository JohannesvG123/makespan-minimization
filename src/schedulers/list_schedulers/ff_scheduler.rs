use std::rc::Rc;
use std::sync::Arc;

use crate::Algorithm;
use crate::Algorithm::FF;
use crate::input::input::Input;
use crate::output::machine_jobs::MachineJobs;
use crate::output::solution::Solution;
use crate::schedulers::scheduler::Scheduler;

pub struct FFScheduler {
    input: Arc<Input>,
    upper_bound: u32,
    lower_bound: u32,
}

impl Scheduler for FFScheduler {
    fn schedule(&mut self) -> Solution {
        self.first_fit()
    }

    fn get_algorithm(&self) -> Algorithm {
        FF
    }
}

impl FFScheduler {
    pub fn new(input: Arc<Input>, upper_bound_opt: Option<u32>, lower_bound_opt: Option<u32>) -> Self {
        let upper_bound: u32 = match upper_bound_opt {
            None => input.get_jobs().iter().sum::<u32>() / input.get_machine_count() as u32 + input.get_jobs().iter().max().unwrap(), //trvial upper bound
            Some(val) => val
        };
        let lower_bound = match lower_bound_opt {
            None => 0,
            Some(val) => val
        };

        Self { input, upper_bound, lower_bound }
    }

    /// Assigns the biggest job to the machine with the smallest index until all jobs are assigned
    pub fn first_fit(&self) -> Solution {
        println!("running {:?} algorithm...", FF);

        let machine_count = self.input.get_machine_count();
        let jobs = self.input.get_jobs();

        let mut machine_jobs = MachineJobs::empty(machine_count);

        for job_index in 0..self.input.get_job_count() {
            let mut current_machine: usize = 0;

            if machine_jobs.get_machine_workload(current_machine) + jobs[job_index] > self.upper_bound {
                current_machine += 1;
                if current_machine == self.input.get_machine_count() { //satisfiability check
                    println!("ERROR: upper bound {} is to low for the {:?}-algorithm with this input", self.upper_bound, FF);
                    return Solution::unsatisfiable(FF);
                }
            }

            machine_jobs.assign_job(jobs[job_index], current_machine, job_index)
        }

        Solution::new(FF, machine_jobs, self.input.get_jobs())
    }
}
