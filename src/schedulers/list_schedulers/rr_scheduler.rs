use std::rc::Rc;

use crate::Algorithm;
use crate::Algorithm::RR;
use crate::input::input::Input;
use crate::output::solution::Solution;
use crate::schedulers::list_schedulers::assign_job;
use crate::schedulers::scheduler::Scheduler;

pub struct RRScheduler {
    input: Rc<Input>,
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

    /// Round Robin job assignment
    pub fn round_robin(&self) -> Solution {
        println!("running {:?} algorithm...", RR);

        let mut schedule = Vec::with_capacity(self.input.get_job_count());
        let mut machines_workload = vec![0; self.input.get_machine_count()];

        for j in 0..self.input.get_job_count() {
            let mut machine = j.rem_euclid(self.input.get_machine_count());

            let mut offset = 0;
            while machines_workload[(machine + offset).rem_euclid(self.input.get_machine_count())] + self.input.get_jobs()[j] > self.upper_bound {
                offset += 1;
                if offset == self.input.get_machine_count() { //satisfiability check
                    println!("ERROR: upper bound {} is to low for the {:?}-algorithm with this input", self.upper_bound, RR);
                    return Solution::unsatisfiable(RR);
                }
            }
            machine += offset;

            assign_job(&mut schedule, machines_workload.as_mut_slice(), self.input.get_jobs()[j], machine);
        }

        let c_max: u32 = *machines_workload.iter().max().unwrap();

        Solution::new(RR, c_max, schedule, vec![(2, vec![1, 2, 3]), (2, vec![1, 2, 3])])
    }
}
