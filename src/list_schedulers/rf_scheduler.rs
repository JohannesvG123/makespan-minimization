use std::rc::Rc;

use rand::Rng;

use crate::Algorithm;
use crate::Algorithm::RF;
use crate::input::input::Input;
use crate::list_schedulers::assign_job;
use crate::output::solution::Solution;
use crate::scheduler::Scheduler;

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
    pub fn random_fit(&self) -> Solution { //TODO FRAGE hatte mir aufgeschrieben, dass hier kein ub genutzt werden soll... stimmt das?
        println!("running {:?} algorithm...", RF);

        let mut schedule = Vec::with_capacity(self.input.get_job_count());
        let mut machines_workload = vec![0; self.input.get_machine_count()];
        let mut rng = rand::thread_rng();
        let fails_until_check: usize = self.input.get_machine_count();// Number of fails until a satisfiability check is done //TODO FRAGE passt das so oder anderer wert?

        for &job in self.input.get_jobs().iter() {
            let mut random_index = rng.gen_range(0..self.input.get_machine_count());

            let mut fails: usize = 0;
            while machines_workload[random_index] + job > self.upper_bound {
                fails += 1;
                if fails == fails_until_check {
                    if machines_workload.iter().any(|machine_workload| machine_workload + job <= self.upper_bound) { //satisfiability check
                        fails = 0;
                    } else {
                        println!("ERROR: upper bound {} is to low for the {:?}-algorithm with this input", self.upper_bound, RF);
                        return Solution::unsatisfiable(RF);
                    }
                }
                random_index = rng.gen_range(0..self.input.get_machine_count());
            }

            assign_job(&mut schedule, machines_workload.as_mut_slice(), job, random_index);
        }

        let c_max: u32 = *machines_workload.iter().max().unwrap();


        Solution::new(RF, c_max, schedule, vec![(2, vec![1, 2, 3]), (2, vec![1, 2, 3])])
    }
}
