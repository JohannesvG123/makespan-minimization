use std::rc::Rc;
use std::sync::Arc;

use crate::Algorithm;
use crate::Algorithm::RR;
use crate::input::input::Input;
use crate::output::machine_jobs::MachineJobs;
use crate::output::solution::Solution;
use crate::schedulers::scheduler::Scheduler;

pub struct RRScheduler {
    input: Arc<Input>,
    upper_bound: u32,
    lower_bound: u32,
}
impl Scheduler for RRScheduler {
    fn schedule(&mut self) -> Solution {
        self.round_robin()
    }

    fn get_algorithm(&self) -> Algorithm {
        RR
    }
}

impl RRScheduler {
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

    /// Round Robin job assignment
    pub fn round_robin(&self) -> Solution {
        println!("running {:?} algorithm...", RR);

        let machine_count = self.input.get_machine_count();
        let jobs = self.input.get_jobs();

        let mut machine_jobs = MachineJobs::empty(machine_count);

        for job_index in 0..self.input.get_job_count() {
            let mut machine = job_index.rem_euclid(self.input.get_machine_count());

            let mut offset = 0;
            while machine_jobs.get_machine_workload((machine + offset).rem_euclid(self.input.get_machine_count())) + self.input.get_jobs()[job_index] > self.upper_bound {
                offset += 1;
                if offset == self.input.get_machine_count() { //satisfiability check
                    println!("ERROR: upper bound {} is to low for the {:?}-algorithm with this input", self.upper_bound, RR);
                    return Solution::unsatisfiable(RR);
                }
            }
            machine += offset;

            machine_jobs.assign_job(jobs[job_index], machine, job_index);
        }

        Solution::new(RR, machine_jobs, self.input.get_jobs())
    }
}
