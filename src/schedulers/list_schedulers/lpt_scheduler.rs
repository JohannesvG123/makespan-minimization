use std::sync::Arc;

use crate::Algorithm;
use crate::Algorithm::LPT;
use crate::global_bounds::bounds::Bounds;
use crate::good_solutions::good_solutions::GoodSolutions;
use crate::input::input::Input;
use crate::output::machine_jobs::MachineJobs;
use crate::output::solution::Solution;
use crate::schedulers::scheduler::Scheduler;

pub struct LPTScheduler {
    input: Arc<Input>,
    global_bounds: Arc<Bounds>,
}

impl Scheduler for LPTScheduler {
    fn schedule(&mut self, good_solutions: GoodSolutions) -> Solution {
        self.longest_processing_time()
    }

    fn get_algorithm(&self) -> Algorithm {
        LPT
    }
}

impl LPTScheduler {
    pub fn new(input: Arc<Input>, global_bounds: Arc<Bounds>) -> Self {
        Self { input, global_bounds }
    }

    /// Assigns the biggest job to the least loaded machine until all jobs are assigned (= worst fit)
    fn longest_processing_time(&self) -> Solution {
        println!("running {:?} algorithm...", LPT);

        let (upper_bound, lower_bound) = self.global_bounds.get_bounds();
        let machine_count = self.input.get_machine_count();
        let jobs = self.input.get_jobs();

        let mut machine_jobs = MachineJobs::empty(machine_count);
        let mut current_machine: usize = 0;
        let mut foreward: bool = true; // used to fill the machines in this order: (m=3) 0-1-2-2-1-0-0-1-2...
        let mut pause: bool = false;

        for job_index in 0..self.input.get_job_count() {
            if machine_jobs.get_machine_workload(current_machine) + jobs[job_index] > upper_bound { //satisfiability check
                println!("ERROR: upper bound {} is to low for the {:?}-algorithm with this input", upper_bound, LPT);
                return Solution::unsatisfiable(LPT);
            }
            machine_jobs.assign_job(jobs[job_index], current_machine, job_index);

            if foreward {
                if pause { pause = false; } else { current_machine += 1; }
                if current_machine == self.input.get_machine_count() - 1 {
                    foreward = false;
                    pause = true;
                }
            } else {
                if pause { pause = false } else { current_machine -= 1; }
                if current_machine == 0 {
                    foreward = true;
                    pause = true
                }
            }
        }

        Solution::new(LPT, None, machine_jobs, self.input.get_jobs(), Arc::clone(&self.global_bounds))
    }
}
