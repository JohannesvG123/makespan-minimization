use std::rc::Rc;

use crate::Algorithm;
use crate::Algorithm::LPT;
use crate::input::input::Input;
use crate::output::machine_jobs::MachineJobs;
use crate::output::solution::Solution;
use crate::schedulers::scheduler::Scheduler;

pub struct LPTScheduler {
    input: Rc<Input>,
    upper_bound: u32,
    lower_bound: u32,
}

impl Scheduler for LPTScheduler {
    fn schedule(&mut self) -> Solution {
        self.longest_processing_time()
    }

    fn get_algorithm(&self) -> Algorithm {
        LPT
    }
}

impl LPTScheduler {
    pub fn new(input: Rc<Input>, upper_bound_opt: Option<u32>, lower_bound_opt: Option<u32>) -> Self {
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

    /// Assigns the biggest job to the least loaded machine until all jobs are assigned (= worst fit)
    fn longest_processing_time(&self) -> Solution {
        println!("running {:?} algorithm...", LPT);

        let machine_count = self.input.get_machine_count();
        let jobs = self.input.get_jobs();

        let mut machine_jobs = MachineJobs::empty(machine_count);
        let mut current_machine: usize = 0;
        let mut foreward: bool = true; // used to fill the machines in this order: (m=3) 0-1-2-2-1-0-0-1-2...
        let mut pause: bool = false;

        for job_index in 0..self.input.get_job_count() {
            if machine_jobs.get_machine_workload(current_machine) + jobs[job_index] > self.upper_bound { //satisfiability check
                println!("ERROR: upper bound {} is to low for the {:?}-algorithm with this input", self.upper_bound, LPT);
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

        Solution::new(LPT, machine_jobs, self.input.get_jobs())
    }
}
