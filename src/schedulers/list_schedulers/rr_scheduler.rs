use std::sync::Arc;
use std::time::Instant;

use permutation::Permutation;

use crate::{Algorithm, Args};
use crate::Algorithm::RR;
use crate::global_bounds::bounds::Bounds;
use crate::good_solutions::good_solutions::GoodSolutions;
use crate::input::input::Input;
use crate::output::log;
use crate::output::machine_jobs::MachineJobs;
use crate::output::solution::Solution;
use crate::schedulers::scheduler::Scheduler;

pub struct RRScheduler {
    input: Arc<Input>,
    global_bounds: Arc<Bounds>,
}

impl Scheduler for RRScheduler {
    fn schedule(&mut self, good_solutions: GoodSolutions, args: Arc<Args>, perm: Arc<Permutation>, start_time: Instant) -> Solution {
        self.round_robin(args, perm, start_time)
    }

    fn get_algorithm(&self) -> Algorithm {
        RR
    }
}

impl RRScheduler {
    pub fn new(input: Arc<Input>, global_bounds: Arc<Bounds>) -> Self {
        Self { input, global_bounds }
    }

    /// Round Robin job assignment
    pub fn round_robin(&self, args: Arc<Args>, perm: Arc<Permutation>, start_time: Instant) -> Solution {
        log(format!("running {:?} algorithm...", RR), false, args.measurement, None);

        let (upper_bound, lower_bound) = self.global_bounds.get_bounds();
        let machine_count = self.input.get_machine_count();
        let jobs = self.input.get_jobs();

        let mut machine_jobs = MachineJobs::empty(machine_count);

        for job_index in 0..self.input.get_job_count() {
            let mut machine = job_index.rem_euclid(self.input.get_machine_count());

            let mut offset = 0;
            while machine_jobs.get_machine_workload((machine + offset).rem_euclid(self.input.get_machine_count())) + self.input.get_jobs()[job_index] > upper_bound {
                offset += 1;
                if offset == self.input.get_machine_count() { //satisfiability check
                    log(format!("ERROR: upper bound {} is to low for the {:?}-algorithm with this input", upper_bound, RR), false, args.measurement, Some(RR));
                    return Solution::unsatisfiable(RR);
                }
            }
            machine = (machine + offset).rem_euclid(self.input.get_machine_count());

            machine_jobs.assign_job(jobs[job_index], machine, job_index);
        }

        Solution::new(RR, None, machine_jobs, self.input.get_jobs(), Arc::clone(&self.global_bounds), args, perm, start_time, machine_count)
    }
}
