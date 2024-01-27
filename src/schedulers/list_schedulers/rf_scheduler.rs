use std::str::FromStr;
use std::string::ParseError;
use std::sync::Arc;

use rand::{Rng, SeedableRng, thread_rng};
use rand_chacha::ChaCha8Rng;
use rayon::current_thread_index;

use crate::{Algorithm};
use crate::Algorithm::RF;
use crate::global_bounds::bounds::Bounds;
use crate::good_solutions::good_solutions::GoodSolutions;
use crate::input::input::Input;
use crate::output::machine_jobs::MachineJobs;
use crate::output::solution::Solution;
use crate::schedulers::scheduler::Scheduler;

pub struct RFScheduler {
    input: Arc<Input>,
    global_bounds: Arc<Bounds>,
    config: RFConfig,
}

impl Scheduler for RFScheduler {
    fn schedule(&mut self, good_solutions: GoodSolutions) -> Solution {
        self.random_fit()
    }

    fn get_algorithm(&self) -> Algorithm {
        RF
    }
}

impl RFScheduler {
    pub fn new(input: Arc<Input>, global_bounds: Arc<Bounds>, config: RFConfig) -> Self {
        Self { input, global_bounds, config }
    }

    /// Assigns the jobs to random machines
    pub fn random_fit(&self) -> Solution {
        println!("running {:?} algorithm...", RF);
        println!("4T_ID:{:?}", current_thread_index());

        let (upper_bound, lower_bound) = self.global_bounds.get_bounds();
        let machine_count = self.input.get_machine_count();
        let jobs = self.input.get_jobs();

        let mut machine_jobs = MachineJobs::empty(machine_count);
        let fails_until_check: usize = self.input.get_machine_count(); // Number of fails until a satisfiability check is done

        let mut seed: <ChaCha8Rng as SeedableRng>::Seed = Default::default();
        thread_rng().fill(&mut seed);
        println!("seed:{:?}", seed);
        let mut rng = ChaCha8Rng::from_seed(seed);

        for job_index in 0..self.input.get_job_count() {
            let mut random_index = rng.gen_range(0..self.input.get_machine_count());

            let mut fails: usize = 0;
            while machine_jobs.get_machine_workload(random_index) + jobs[job_index] > upper_bound {
                fails += 1;
                if fails == fails_until_check {
                    if (0..machine_count).collect::<Vec<_>>().iter().any(|&machine_index| machine_jobs.get_machine_workload(machine_index) + jobs[job_index] <= upper_bound) { //satisfiability check //TODO (low prio) hier kann evtl speedup erreicht werden (volle maschienen halten) / oder lässt man den einf komplett raus?
                        fails = 0;
                    } else {
                        println!("ERROR: upper bound {} is to low for the {:?}-algorithm with this input", upper_bound, RF);
                        return Solution::unsatisfiable(RF);
                    }
                }
                random_index = rng.gen_range(0..self.input.get_machine_count());
            }

            machine_jobs.assign_job(jobs[job_index], random_index, job_index)
        }

        Solution::new(RF, Some(format!("{:?}", self.config)), machine_jobs, self.input.get_jobs(), Arc::clone(&self.global_bounds)) //TODO vllt display implementieren für die config
    }
}

#[derive(Clone, Debug)]
pub struct RFConfig {
    rng_seed: [u8; 32],
    fails_until_check: usize,
}

impl FromStr for RFConfig {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(";").collect();
        Ok(RFConfig {
            rng_seed: {
                if parts[0].len() > 0 {
                    //todo outline und für swap verwenden
                    let seed_part = parts[0].strip_prefix('[').unwrap().strip_suffix(']').unwrap();
                    let seed_parts: Vec<&str> = seed_part.split(",").collect();

                    let mut seed: [u8; 32] = [0; 32];
                    for i in 0..seed_parts.len() {
                        seed[i] = seed_parts[i].parse::<u8>().unwrap();
                    }
                    seed
                } else {
                    //default: random seed
                    let mut seed: <ChaCha8Rng as SeedableRng>::Seed = Default::default();
                    thread_rng().fill(&mut seed);
                    seed
                }
            },
            fails_until_check: {
                if parts.len() > 1 && parts[1].len() > 0 {
                    parts[1].parse::<usize>().unwrap()
                } else {
                    //default:
                    50 //todo global var dafür einbauen
                }
            },
        })
    }
}