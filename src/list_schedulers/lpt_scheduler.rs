use std::rc::Rc;

use crate::Algorithm;
use crate::Algorithm::LPT;
use crate::input::input::Input;
use crate::list_schedulers::assign_job;
use crate::output::solution::Solution;
use crate::scheduler::Scheduler;

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

        let mut schedule = Vec::with_capacity(self.input.get_job_count());
        let mut machines_workload = vec![0; self.input.get_machine_count()];
        let mut current_machine: usize = 0;
        let mut foreward: bool = true; // used to fill the machines in this order: (m=3) 0-1-2-2-1-0-0-1-2...
        let mut pause: bool = false;

        for &job in self.input.get_jobs().iter() {
            if machines_workload[current_machine] + job > self.upper_bound { //satisfiability check
                println!("ERROR: upper bound {} is to low for the {:?}-algorithm with this input", self.upper_bound, LPT);
                return Solution::unsatisfiable(LPT);
            }
            assign_job(&mut schedule, machines_workload.as_mut_slice(), job, current_machine);

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

        let c_max: u32 = *machines_workload.iter().max().unwrap(); //TODO cmax smarter berechnen evtl(?)

        //TODO machine_jobs richtig befüllen (bei allen LS schedulern)
        Solution::new(LPT, c_max, schedule, vec![(2, vec![1, 2, 3]), (2, vec![1, 2, 3])])
    }
}
