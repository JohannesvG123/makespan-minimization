use std::rc::Rc;

use crate::Algorithm;
use crate::Algorithm::BF;
use crate::input::input::Input;
use crate::output::solution::Solution;
use crate::schedulers::list_schedulers::assign_job;
use crate::schedulers::scheduler::Scheduler;

pub struct BFScheduler {
    input: Rc<Input>,
    upper_bound: u32,
    lower_bound: u32,
}

impl Scheduler for BFScheduler {
    fn schedule(&mut self) -> Solution {
        self.best_fit()
    }

    fn get_algorithm(&self) -> Algorithm {
        BF
    }
}

impl BFScheduler {
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

    /// Assigns the biggest job to the most loaded machine (that can fit the job) until all jobs are assigned
    pub fn best_fit(&self) -> Solution {
        println!("running {:?} algorithm...", BF);

        let mut schedule = Vec::with_capacity(self.input.get_job_count());
        let mut machines_workload = vec![0; self.input.get_machine_count()];

        for &job in self.input.get_jobs().iter() {
            let mut best_machine = 0;
            let mut fitting_machine_found = false;
            for m in 0..self.input.get_machine_count() { //man könnte hier speedup erreichen wenn man ab Eingabegröße x eine BH-PQ nutzt...
                if !fitting_machine_found && machines_workload[m] + job <= self.upper_bound {
                    best_machine = m;
                    fitting_machine_found = true
                } else if fitting_machine_found && machines_workload[m] + job <= self.upper_bound && machines_workload[m] + job > machines_workload[best_machine] + job {
                    best_machine = m;
                }
            }
            if !fitting_machine_found { //satisfiability check
                println!("ERROR: upper bound {} is to low for the {:?}-algorithm with this input", self.upper_bound, BF);
                return Solution::unsatisfiable(BF);
            }

            assign_job(&mut schedule, machines_workload.as_mut_slice(), job, best_machine);
        }


        let c_max: u32 = *machines_workload.iter().max().unwrap();


        Solution::new(BF, c_max, schedule, vec![(2, vec![1, 2, 3]), (2, vec![1, 2, 3])]) //TODO 1 machine_jobs -> eig reichts ja nur machine jobs zu halten und dann daraus schedule zu berechnen
    }
}
