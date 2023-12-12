use std::rc::Rc;

use rand::Rng;

use crate::Algorithm;
use crate::Algorithm::RF;
use crate::input::input::Input;
use crate::output::machine_jobs::MachineJobs;
use crate::output::solution::Solution;
use crate::schedulers::scheduler::Scheduler;

pub struct RFScheduler {
    input: Rc<Input>,
    upper_bound: u32,
    lower_bound: u32,
}

impl Scheduler for RFScheduler {
    fn schedule(&mut self) -> Solution {
        self.random_fit()
    }

    fn get_algorithm(&self) -> Algorithm {
        RF
    }
}

impl RFScheduler {
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

    /// Assigns the jobs to random machines
    pub fn random_fit(&self) -> Solution {
        println!("running {:?} algorithm...", RF);

        let machine_count = self.input.get_machine_count();
        let jobs = self.input.get_jobs();

        let mut machine_jobs = MachineJobs::empty(machine_count);
        let mut rng = rand::thread_rng();
        let fails_until_check: usize = self.input.get_machine_count();// Number of fails until a satisfiability check is done //TODO FRAGE passt das so oder anderer wert?

        for job_index in 0..self.input.get_job_count() {
            let mut random_index = rng.gen_range(0..self.input.get_machine_count());

            let mut fails: usize = 0;
            while machine_jobs.get_machine_workload(random_index) + jobs[job_index] > self.upper_bound {
                fails += 1;
                if fails == fails_until_check {
                    if (0..machine_count).collect::<Vec<_>>().iter().any(|&machine_index| machine_jobs.get_machine_workload(machine_index) + jobs[job_index] <= self.upper_bound) { //satisfiability check //TODO (low prio) hier kann evtl speedup erreicht werden (volle maschienen halten)
                        fails = 0;
                    } else {
                        println!("ERROR: upper bound {} is to low for the {:?}-algorithm with this input", self.upper_bound, RF);
                        return Solution::unsatisfiable(RF);
                    }
                }
                random_index = rng.gen_range(0..self.input.get_machine_count());
            }

            machine_jobs.assign_job(jobs[job_index], random_index, job_index)
        }

        Solution::new(RF, machine_jobs, self.input.get_jobs())
    }
}
