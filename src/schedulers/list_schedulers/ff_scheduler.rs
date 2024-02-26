use std::sync::Arc;
use std::time::Instant;

use permutation::Permutation;

use crate::{Algorithm, Args};
use crate::Algorithm::FF;
use crate::global_bounds::bounds::Bounds;
use crate::good_solutions::good_solutions::GoodSolutions;
use crate::input::input::Input;
use crate::output::log;
use crate::output::machine_jobs::MachineJobs;
use crate::output::solution::Solution;
use crate::schedulers::scheduler::Scheduler;

pub struct FFScheduler {
    input: Arc<Input>,
    global_bounds: Arc<Bounds>,
}

impl Scheduler for FFScheduler {
    fn schedule(&mut self, good_solutions: GoodSolutions, args: Arc<Args>, perm: Arc<Permutation>, start_time: Instant) -> Solution {
        self.first_fit(args, perm, start_time)
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
    pub fn first_fit(&self, args: Arc<Args>, perm: Arc<Permutation>, start_time: Instant) -> Solution {
        log(format!("running {:?} algorithm...", FF),false,args.measurement,None);

        let (upper_bound, lower_bound) = self.global_bounds.get_bounds();
        let machine_count = self.input.get_machine_count();
        let jobs = self.input.get_jobs();

        let mut machine_jobs = MachineJobs::empty(machine_count);

        for job_index in 0..self.input.get_job_count() {
            let mut current_machine: usize = 0;

            if machine_jobs.get_machine_workload(current_machine) + jobs[job_index] > upper_bound {
                current_machine += 1;
                if current_machine == self.input.get_machine_count() { //satisfiability check
                    log(format!("ERROR: upper bound {} is to low for the {:?}-algorithm with this input", upper_bound, FF),false,args.measurement,Some(FF));
                    return Solution::unsatisfiable(FF);
                }
            }

            machine_jobs.assign_job(jobs[job_index], current_machine, job_index)
        }

        Solution::new(FF, None, machine_jobs, self.input.get_jobs(), Arc::clone(&self.global_bounds), args, perm, start_time)
    }
}
