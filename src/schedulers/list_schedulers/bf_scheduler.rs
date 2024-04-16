use std::sync::Arc;
use std::time::Instant;

use permutation::Permutation;

use crate::{Algorithm, Args};
use crate::Algorithm::BF;
use crate::global_bounds::bounds::Bounds;
use crate::good_solutions::good_solutions::GoodSolutions;
use crate::input::input::Input;
use crate::output::log;
use crate::output::machine_jobs::MachineJobs;
use crate::output::solution::Solution;
use crate::schedulers::scheduler::Scheduler;

pub struct BFScheduler {
    input: Arc<Input>,
    global_bounds: Arc<Bounds>,
}

impl Scheduler for BFScheduler {
    fn schedule(&mut self, _good_solutions: GoodSolutions, args: Arc<Args>, perm: Arc<Permutation>, start_time: Instant) -> Solution {
        self.best_fit(args, perm, start_time)
    }

    fn get_algorithm(&self) -> Algorithm {
        BF
    }
}

impl BFScheduler {
    pub fn new(input: Arc<Input>, global_bounds: Arc<Bounds>) -> Self {
        Self { input, global_bounds }
    }

    /// Assigns the biggest job to the most loaded machine (that can fit the job) until all jobs are assigned
    pub fn best_fit(&self, args: Arc<Args>, perm: Arc<Permutation>, start_time: Instant) -> Solution {
        log(format!("running {:?} algorithm...", BF), false, args.measurement, None);

        let (upper_bound, _lower_bound) = self.global_bounds.get_bounds();
        let machine_count = self.input.get_machine_count();
        let jobs = self.input.get_jobs();

        let mut machine_jobs = MachineJobs::empty(machine_count);

        for job_index in 0..self.input.get_job_count() {
            let mut best_machine = 0;
            let mut fitting_machine_found = false;
            for m in 0..machine_count { //man könnte hier speedup erreichen wenn man ab Eingabegröße x eine BH-PQ nutzt...
                if !fitting_machine_found && machine_jobs.get_machine_workload(m) + jobs[job_index] <= upper_bound {
                    best_machine = m;
                    fitting_machine_found = true
                } else if fitting_machine_found && machine_jobs.get_machine_workload(m) + jobs[job_index] <= upper_bound && machine_jobs.get_machine_workload(m) + jobs[job_index] > machine_jobs.get_machine_workload(best_machine) + jobs[job_index] {
                    best_machine = m;
                }
            }
            if !fitting_machine_found { //satisfiability check
                log(format!("ERROR: upper bound {} is to low for the {:?}-algorithm with this input", upper_bound, BF), false, args.measurement, Some(BF));
                return Solution::unsatisfiable(BF);
            }

            machine_jobs.assign_job(jobs[job_index], best_machine, job_index);
        }

        Solution::new(BF, None, machine_jobs, self.input.get_jobs(), Arc::clone(&self.global_bounds), args, perm, start_time, machine_count)
    }
}