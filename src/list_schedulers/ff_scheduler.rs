use std::rc::Rc;

use crate::Algorithm;
use crate::Algorithm::FF;
use crate::input::input::Input;
use crate::list_schedulers::assign_job;
use crate::output::solution::Solution;
use crate::scheduler::Scheduler;

pub struct FFScheduler {
    input: Rc<Input>,
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

    /// Assigns the biggest job to the machine with the smallest index until all jobs are assigned
    pub fn first_fit(&self) -> Solution {
        println!("running {:?} algorithm...", FF);

        let mut schedule = Vec::with_capacity(self.input.get_job_count());
        let mut machines_workload = vec![0; self.input.get_machine_count()];

        for &job in self.input.get_jobs().iter() {
            let mut current_machine: usize = 0;

            if machines_workload[current_machine] + job > self.upper_bound {
                current_machine += 1;
                if current_machine == self.input.get_machine_count() { //satisfiability check
                    println!("ERROR: upper bound {} is to low for the {:?}-algorithm with this input", self.upper_bound, FF);
                    return Solution::unsatisfiable(FF);
                }
            }

            assign_job(&mut schedule, machines_workload.as_mut_slice(), job, current_machine);
        }


        let c_max: u32 = *machines_workload.iter().max().unwrap();


        Solution::new(FF, c_max, schedule, vec![(2, vec![1, 2, 3]), (2, vec![1, 2, 3])])
    }
}
