use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::string::ParseError;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use permutation::Permutation;
use rand::Rng;

use crate::{Algorithm, Args};
use crate::Algorithm::RF;
use crate::global_bounds::bounds::Bounds;
use crate::good_solutions::good_solutions::GoodSolutions;
use crate::input::input::Input;
use crate::input::MyRng;
use crate::output::log;
use crate::output::machine_jobs::MachineJobs;
use crate::output::solution::Solution;
use crate::schedulers::scheduler::Scheduler;

pub struct RFScheduler {
    input: Arc<Input>,
    global_bounds: Arc<Bounds>,
    config: ConcreteRFConfig,
    higher_level_algo: Option<Algorithm>,
}

impl Scheduler for RFScheduler {
    fn schedule(&mut self, good_solutions: GoodSolutions, args: Arc<Args>, perm: Arc<Permutation>, start_time: Instant) -> Solution {
        self.random_fit(args, perm, start_time, true)
    }

    fn get_algorithm(&self) -> Algorithm {
        RF
    }
}

impl RFScheduler {
    pub fn new(input: Arc<Input>, global_bounds: Arc<Bounds>, config: &RFConfig, shared_initial_rng: Arc<Mutex<MyRng>>, higher_level_algo: Option<Algorithm>) -> Self {
        Self { input: Arc::clone(&input), global_bounds, config: ConcreteRFConfig::new(config, input, shared_initial_rng), higher_level_algo }
    }

    pub fn schedule_without_bounds(&mut self, good_solutions: GoodSolutions, args: Arc<Args>, perm: Arc<Permutation>, start_time: Instant) -> Solution {
        self.random_fit(args, perm, start_time, false)
    }

    /// Assigns the jobs to random machines
    pub fn random_fit(&mut self, args: Arc<Args>, perm: Arc<Permutation>, start_time: Instant, use_bounds: bool) -> Solution { //TODO use bounds effizienter impl!
        log(format!("running {:?} algorithm...", RF), false, args.measurement, self.higher_level_algo);

        let (upper_bound, lower_bound) = self.global_bounds.get_bounds();
        let machine_count = self.input.get_machine_count();
        let jobs = self.input.get_jobs();

        let mut machine_jobs = MachineJobs::empty(machine_count);

        for job_index in 0..self.input.get_job_count() {
            let mut random_index = self.config.rng.get_mut().gen_range(0..self.input.get_machine_count());

            let mut fails: usize = 0;
            if use_bounds {
                while machine_jobs.get_machine_workload(random_index) + jobs[job_index] > upper_bound {
                    fails += 1;
                    if fails == self.config.fails_until_check {
                        if (0..machine_count).collect::<Vec<_>>().iter().any(|&machine_index| machine_jobs.get_machine_workload(machine_index) + jobs[job_index] <= upper_bound) { //satisfiability check //TODO prio den weglassen der is lost
                            log(String::from("performed satisfiability check because fails_until_check was reached"), false, args.measurement, Some(RF));
                            fails = 0;
                        } else {
                            log(format!("ERROR: upper bound {} is to low for the {:?}-algorithm with this input", upper_bound, RF), false, args.measurement, Some(RF)); //todo aussagekräftiger machen
                            return Solution::unsatisfiable(RF);
                        }
                    }
                    random_index = self.config.rng.get_mut().gen_range(0..self.input.get_machine_count());
                }
            }

            machine_jobs.assign_job(jobs[job_index], random_index, job_index)
        }

        Solution::new(RF, Some(self.config.to_string()), machine_jobs, self.input.get_jobs(), Arc::clone(&self.global_bounds), args, perm, start_time)
    }
}

#[derive(Clone, Debug)]
pub struct ConcreteRFConfig {
    rng: MyRng,
    fails_until_check: usize,
}

impl ConcreteRFConfig {
    pub fn new(config: &RFConfig, input: Arc<Input>, shared_initial_rng: Arc<Mutex<MyRng>>) -> Self {
        ConcreteRFConfig {
            rng: {
                let mut guard = shared_initial_rng.lock().unwrap();
                guard.generate_new_seed().create_rng()
            },
            fails_until_check: {
                match config.fails_until_check {
                    None => { input.get_machine_count() }
                    Some(n) => { n }
                }
            },
        }
    }
}

impl Display for ConcreteRFConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RF_CONFIG: RNG:{} ;FAILS_UNTIL_CHECK:{}", self.rng, self.fails_until_check)
    }
}

#[derive(Clone, Debug)]
pub struct RFConfig {
    fails_until_check: Option<usize>,
}

impl RFConfig {
    pub fn new() -> Self {
        Self { fails_until_check: None }
    }
}

impl FromStr for RFConfig {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("-fails-until-check").collect();
        Ok(RFConfig {
            fails_until_check: {
                if parts.len() > 1 && parts[0].len() > 0 {
                    Some(parts[0].parse::<usize>().unwrap())
                } else {
                    None //default
                }
            },
        })
    }
}