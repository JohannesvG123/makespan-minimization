use std::cmp::max;
use std::fmt::Debug;
use std::str::FromStr;
use std::string::ParseError;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use atoi::atoi;
use clap::ValueEnum;
use permutation::Permutation;
use rand::Rng;
use rand_distr::Distribution;
use rand_distr::Exp;
use rayon::current_num_threads;
use regex::Regex;

use crate::{Algorithm, Args};
use crate::Algorithm::Swap;
use crate::global_bounds::bounds::Bounds;
use crate::good_solutions::good_solutions::GoodSolutions;
use crate::input::input::Input;
use crate::input::MyRng;
use crate::output::log;
use crate::output::machine_jobs::MachineJobs;
use crate::output::solution::Solution;
use crate::schedulers::list_schedulers::rf_scheduler::{RFConfig, RFScheduler};
use crate::schedulers::local_search::swapper::SwapAcceptanceRule::{All, DeclineByChance, Improvement, SimulatedAnnealing};
use crate::schedulers::local_search::swapper::SwapTactic::{TwoJobBruteForce, TwoJobRandomSwap};
use crate::schedulers::scheduler::Scheduler;

pub struct Swapper {
    input: Arc<Input>,
    global_bounds: Arc<Bounds>,
    config: SwapConfig,
    shared_initial_rng: Arc<Mutex<MyRng>>,
}

impl Scheduler for Swapper {
    fn schedule(&mut self, good_solutions: GoodSolutions, args: Arc<Args>, perm: Arc<Permutation>, start_time: Instant) -> Solution {
        self.swap(good_solutions, args, perm, start_time)
    }

    fn get_algorithm(&self) -> Algorithm {
        Swap
    }
}

///Tactic to find jobs to swap
#[derive(Clone, Copy, Debug)]
pub enum SwapTactic {
    TwoJobBruteForce,
    TwoJobRandomSwap(usize), //fails_until_stop
}

///Rule when to accept a swap
#[derive(Clone, Copy, Debug)]
pub enum SwapAcceptanceRule {
    ///accept swap if it improves c_max
    Improvement,
    ///accept improvements & declines with a p-percent chance (0<=p<=100)
    DeclineByChance(u8),
    ///accept improvements & declines with ...todo (-> https://de.wikipedia.org/wiki/Simulated_Annealing)
    SimulatedAnnealing,

    ///accept only declines of c_max (used to get out of local minimum)
    Decline,
    ///accept all swaps independent of c_max
    All,
}

impl Swapper {
    pub fn new(input: Arc<Input>, global_bounds: Arc<Bounds>, config: SwapConfig, shared_initial_rng: Arc<Mutex<MyRng>>) -> Self {
        Self {
            input,
            global_bounds,
            config,
            shared_initial_rng,
        }
    }

    fn accept_improvement(old_c_max: u32, new_c_max: u32, _concrete_swap_config: &mut ConcreteSwapConfig) -> bool {
        new_c_max > old_c_max
    }

    fn accept_decline_by_chance_c(old_c_max: u32, new_c_max: u32, concrete_swap_config: &mut ConcreteSwapConfig) -> bool {
        Self::accept_decline_by_chance(old_c_max, new_c_max, concrete_swap_config)
    }
    fn accept_decline_by_chance(old_c_max: u32, new_c_max: u32, concrete_swap_config: &mut ConcreteSwapConfig) -> bool {
        let percentage = concrete_swap_config.decline_by_chance_percentage.unwrap() as f64 / 100f64;
        debug_assert!(0f64 <= percentage);
        debug_assert!(1f64 >= percentage);

        if new_c_max > old_c_max {
            true
        } else {
            concrete_swap_config.rng.get_mut().gen_bool(percentage)
        }
    }

    fn accept_decline(old_c_max: u32, new_c_max: u32, _concrete_swap_config: &mut ConcreteSwapConfig) -> bool {
        new_c_max < old_c_max
    }

    fn accept_all(_old_c_max: u32, _new_c_max: u32, _concrete_swap_config: &mut ConcreteSwapConfig) -> bool {
        true
    }

    /// swaps jobs of specified good solution(s) to create better one(s)
    /// the newly created solutions get stored in good_solutions
    /// the best one gets returned
    fn swap(&self, good_solutions: GoodSolutions, args: Arc<Args>, perm: Arc<Permutation>, start_time: Instant) -> Solution {
        log(format!("running {:?} algorithm...", Swap), false, args.measurement, None);

        rayon::scope(move |s| {
            let number_of_solutions = match self.config.number_of_solutions { //logic when to wait and when not to wait:
                None => {
                    //Case: "max"
                    while good_solutions.get_solution_count() < 1 {
                        log(String::from("waiting for enough good solutions to run Swap algorithm..."), false, args.measurement, Some(Swap));
                    }
                    current_num_threads()
                }
                Some(n) => {
                    while good_solutions.get_solution_count() < n {
                        log(String::from("waiting for enough good solutions to run Swap algorithm..."), false, args.measurement, Some(Swap));
                    }
                    n
                }
            };

            let mut old_solutions = good_solutions.get_best_solutions(number_of_solutions);
            let mut tmp_i = 0;
            while old_solutions.len() < number_of_solutions { //only happens if number_of_solutions==max and best_solutions.count<max
                //solutions doppelt verwenden
                old_solutions.push(old_solutions[tmp_i].clone());
                tmp_i = (tmp_i + 1).rem_euclid(number_of_solutions)
            }
            let old_solutions = Arc::new(old_solutions);

            for i in 0..old_solutions.len() {
                let old_solutions = Arc::clone(&old_solutions);
                let perm = Arc::clone(&perm);
                let args = Arc::clone(&args);
                let good_solutions = good_solutions.clone();

                s.spawn(move |_| {
                    //new swap tactics can be added here:
                    let swap_finding_tactic_fn = match self.config.swap_finding_tactic {
                        TwoJobBruteForce => Self::find_brute_force_two_job_swap,
                        TwoJobRandomSwap(_) => Self::find_random_two_job_swap,
                    };

                    //new swap acceptance rules can be added here:
                    let swap_acceptance_rule_fn: fn(u32, u32, &mut ConcreteSwapConfig) -> bool = match self.config.swap_acceptance_rule {
                        Improvement => Self::accept_improvement,
                        DeclineByChance(percentage) => Self::accept_decline_by_chance_c,
                        SimulatedAnnealing => {
                            todo!()
                        }
                        SwapAcceptanceRule::Decline => Self::accept_decline,
                        All => Self::accept_all,
                    };
                    let random_swap_fails_until_stop = match self.config.swap_finding_tactic {
                        TwoJobRandomSwap(fails_until_stop) => { Some(fails_until_stop) }
                        _ => { None }
                    };
                    let decline_by_chance_percentage = match self.config.swap_acceptance_rule {
                        DeclineByChance(percentage) => { Some(percentage) }
                        _ => { None }
                    };
                    let rng = self.shared_initial_rng.lock().unwrap().generate_new_seed().create_rng();

                    let mut concrete_swap_config = ConcreteSwapConfig {
                        swap_finding_tactic: swap_finding_tactic_fn,
                        swap_acceptance_rule: swap_acceptance_rule_fn,
                        decline_by_chance_percentage,
                        random_swap_fails_until_stop,
                        rng,
                    };

                    let mut solution = old_solutions[i].clone();
                    solution.add_algorithm(Swap);
                    solution.add_config(format!("SWAP_CONFIG: SWAP_FINDING_TACTIC:{:?}; SWAP_ACCEPTANCE_RULE:{:?}; NUMBER_OF_SOLUTIONS:{:?}; RNG:{}", self.config.swap_finding_tactic, self.config.swap_acceptance_rule, self.config.number_of_solutions, concrete_swap_config.rng));

                    let mut restart_after_steps = self.config.restart_after_steps.unwrap();
                    let mut restart_possibility = self.config.restart_possibility.unwrap();

                    let mut rf_scheduler = RFScheduler::new(Arc::clone(&self.input), Arc::clone(&self.global_bounds), &RFConfig::new(), Arc::clone(&self.shared_initial_rng), Some(Swap));

                    loop {
                        let mut restart = false;
                        let mut steps = 0;

                        while let Some(swap_indices) = (concrete_swap_config.swap_finding_tactic)(self, &solution, &mut concrete_swap_config) {
                            solution.swap_jobs(swap_indices, self.input.get_jobs(), self.input.get_machine_count(), Arc::clone(&self.global_bounds), Arc::clone(&args), Arc::clone(&perm), start_time, Some(Swap));
                            //add newly found solution to shared structs
                            self.global_bounds.update_upper_bound(solution.get_data().get_c_max(), &solution, Arc::clone(&args), Arc::clone(&perm), start_time, Some(Swap), self.input.get_jobs(), self.input.get_machine_count());
                            good_solutions.add_solution(solution.clone());
                            steps += 1;
                            //println!("{}", steps);

                            if self.config.do_restart_after_steps {
                                if steps == restart_after_steps {
                                    restart = true;
                                    steps = 0;
                                    restart_after_steps = (restart_after_steps as f64 * self.config.restart_scaling_factor) as usize;
                                }
                            } else {
                                restart = concrete_swap_config.rng.get_mut().gen_bool(restart_possibility);
                                if restart {
                                    restart_possibility *= (1.0 / self.config.restart_scaling_factor);
                                }
                            }

                            if restart { break; }
                        }
                        //println!("DO RESTART");
                        let random_restart = concrete_swap_config.rng.get_mut().gen_bool(self.config.random_restart_possibility);

                        if random_restart {
                            //generate new random solution:
                            solution = rf_scheduler.schedule_without_bounds(Arc::clone(&args), Arc::clone(&perm), start_time);
                        } else {
                            //choose x-th good solution (using exp. distribution):
                            let exp = Exp::new(self.config.lambda).unwrap();
                            let x = exp.sample(concrete_swap_config.rng.get_mut()) as usize;
                            solution = good_solutions.get_x_best_solution(x).unwrap();
                        }

                        solution.add_algorithm(Swap);
                        solution.add_config(format!("SWAP_CONFIG: SWAP_FINDING_TACTIC:{:?}; SWAP_ACCEPTANCE_RULE:{:?}; NUMBER_OF_SOLUTIONS:{:?}; RNG:{}", self.config.swap_finding_tactic, self.config.swap_acceptance_rule, self.config.number_of_solutions, concrete_swap_config.rng));
                    }
                });
            }
        });

        Solution::unsatisfiable(Swap) //not reachable
    }

    /// 2 job swap brute force (try all possible swaps)
    fn find_brute_force_two_job_swap(&self, solution: &Solution, concrete_swap_config: &mut ConcreteSwapConfig) -> Option<(usize, usize, usize, usize)> {
        let machine_jobs = solution.get_data().get_machine_jobs();
        let mut current_c_max = solution.get_data().get_c_max();
        let current_heaviest_machines = solution.get_data().get_machine_jobs().get_machines_with_workload(current_c_max);
        let mut swap_indices: (usize, usize, usize, usize) = (0, 0, 0, 0); //(machine_1_index, job_1_index, machine_2_index, job_2_index)
        let mut swap_found = false;

        if current_heaviest_machines.len() == 1 { // otherwise a two job swap does not cause improvements
            let m1 = current_heaviest_machines[0];
            for m2 in 0..self.input.get_machine_count() {
                if m2 == m1 { continue; }

                let machine_1_jobs = machine_jobs.get_machine_jobs(m1);
                let machine_2_jobs = machine_jobs.get_machine_jobs(m2);
                for j1 in 0..machine_1_jobs.len() {
                    for j2 in 0..machine_2_jobs.len() {
                        //for all job pairs (j1,j2) on (m1,m2)
                        let new_c_max = self.simulate_two_job_swap(m1, machine_1_jobs[j1], m2, machine_2_jobs[j2], machine_jobs, current_heaviest_machines.as_slice());
                        if (concrete_swap_config.swap_acceptance_rule)(new_c_max, current_c_max, concrete_swap_config) {
                            swap_found = true;
                            current_c_max = new_c_max;
                            swap_indices = (m1, j1, m2, j2);
                        }
                    }
                }
            }
        }

        if swap_found {
            Some(swap_indices)
        } else {
            None
        }
    }

    /// 2 job random swap
    fn find_random_two_job_swap(&self, solution: &Solution, concrete_swap_config: &mut ConcreteSwapConfig) -> Option<(usize, usize, usize, usize)> {
        let fails_until_stop = concrete_swap_config.random_swap_fails_until_stop.unwrap();
        let mut fails: usize = 0;
        let machine_count = self.input.get_machine_count();
        let machine_jobs = solution.get_data().get_machine_jobs();
        let current_c_max = solution.get_data().get_c_max();
        let current_heaviest_machines = solution.get_data().get_machine_jobs().get_machines_with_workload(current_c_max);

        loop {
            //generate random values
            let mut m1 = concrete_swap_config.rng.get_mut().gen_range((0..machine_count));
            let mut machine_1_jobs = machine_jobs.get_machine_jobs(m1);
            while machine_1_jobs.len() == 0 {
                // in case the machine is not used for the schedule
                m1 = concrete_swap_config.rng.get_mut().gen_range(0..machine_count);
                machine_1_jobs = machine_jobs.get_machine_jobs(m1);
            }
            let mut m2 = concrete_swap_config.rng.get_mut().gen_range(0..machine_count);
            let mut machine_2_jobs = machine_jobs.get_machine_jobs(m2);
            while m2 == m1 || machine_2_jobs.len() == 0 {
                //cant swap from the same machine
                m2 = concrete_swap_config.rng.get_mut().gen_range(0..machine_count);
                machine_2_jobs = machine_jobs.get_machine_jobs(m2);
            }
            let j1 = concrete_swap_config.rng.get_mut().gen_range(0..machine_1_jobs.len());
            let j2 = concrete_swap_config.rng.get_mut().gen_range(0..machine_2_jobs.len());

            //check swap
            let new_c_max = self.simulate_two_job_swap(
                m1,
                machine_1_jobs[j1],
                m2,
                machine_2_jobs[j2],
                machine_jobs,
                current_heaviest_machines.as_slice(),
            );
            if (concrete_swap_config.swap_acceptance_rule)(new_c_max, current_c_max, concrete_swap_config) {
                return Some((m1, j1, m2, j2));
            } else {
                fails += 1;
                if fails == fails_until_stop {
                    println!("Error: reached {} fails (2JobRandomSwap)", fails_until_stop);
                    return None;
                }
            }
        }
    }

    ///computes the c_max that the current solution would have after a specified swap
    fn simulate_two_job_swap(&self, machine_1_index: usize, job_1_index: usize, machine_2_index: usize, job_2_index: usize, machine_jobs: &MachineJobs, current_heaviest_machines: &[usize]) -> u32 {
        let jobs = self.input.get_jobs();

        let machine_1_swap_workload = machine_jobs.get_machine_workload(machine_1_index) + jobs[job_2_index] - jobs[job_1_index];
        let machine_2_swap_workload = machine_jobs.get_machine_workload(machine_2_index) + jobs[job_1_index] - jobs[job_2_index];
        let max_workload = max(machine_1_swap_workload, machine_2_swap_workload);

        if current_heaviest_machines.iter().any(|&machine| machine != machine_1_index && machine != machine_2_index) {
            let current_c_max = machine_jobs.get_machine_workload(current_heaviest_machines[0]);
            max(current_c_max, max_workload)
        } else {
            max_workload
        }
    }
}

impl SwapTactic {
    pub fn from_str(input: &str) -> Result<Self, String> {
        match input {
            "two-job-brute-force" => Ok(TwoJobBruteForce),
            "two-job-random-swap" => {
                //default:
                Ok(TwoJobRandomSwap(50))
            }
            _ => {
                //more complex param (probably)

                if Regex::new(r"^two-job-random-swap-([0-9]+)$").unwrap().is_match(input) {
                    let parts: Vec<&str> = input.split('-').collect();
                    let fails_until_stop = atoi::<usize>(&parts[4].as_bytes()).unwrap();
                    Ok(TwoJobRandomSwap(fails_until_stop))
                } else {
                    Err(format!("invalid variant: {input}"))
                }
            }
        }
    }
}

impl SwapAcceptanceRule {
    pub fn from_str(input: &str) -> Result<Self, String> {
        match input {
            "improvement" => Ok(Improvement),
            "simulated-annealing" => Ok(SimulatedAnnealing),
            "all" => Ok(All),
            _ => {
                //more complex param (probably)
                if Regex::new(r"^decline-by-([0-9]|[1-9][0-9]|100)%-chance$").unwrap().is_match(input) {
                    let p = atoi::<u8>(&input.as_bytes()[11..]).unwrap();
                    Ok(DeclineByChance(p))
                } else {
                    Err(format!("invalid variant: {input}"))
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct SwapConfig {
    swap_finding_tactic: SwapTactic,
    swap_acceptance_rule: SwapAcceptanceRule,
    number_of_solutions: Option<usize>,
    do_restart_after_steps: bool,
    //true=>restart_after_steps=Some(x);false=>restart_possibility=Some(x)
    restart_after_steps: Option<usize>,
    restart_possibility: Option<f64>,
    restart_scaling_factor: f64,
    random_restart_possibility: f64,
    lambda: f64,
}

#[derive(Clone, Debug)]
pub struct ConcreteSwapConfig {
    swap_finding_tactic: fn(&Swapper, &Solution, &mut ConcreteSwapConfig) -> Option<(usize, usize, usize, usize)>,
    swap_acceptance_rule: fn(u32, u32, &mut ConcreteSwapConfig) -> bool,
    decline_by_chance_percentage: Option<u8>,
    random_swap_fails_until_stop: Option<(usize)>,
    rng: MyRng,
}

impl FromStr for SwapConfig {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(",").collect();
        Ok(SwapConfig {
            swap_finding_tactic: {
                if parts[0].len() > 0 {
                    SwapTactic::from_str(parts[0]).unwrap()
                } else {
                    //default:
                    SwapTactic::TwoJobBruteForce
                }
            },
            swap_acceptance_rule: {
                if parts.len() > 1 && parts[1].len() > 0 {
                    SwapAcceptanceRule::from_str(parts[1]).unwrap()
                } else {
                    //default:
                    SwapAcceptanceRule::Improvement
                }
            },
            number_of_solutions: {
                if parts.len() > 2 && parts[2].len() > 0 {
                    if parts[2] == "max" {
                        None
                    } else {
                        Some(parts[2].parse::<usize>().unwrap())
                    }
                } else {
                    //default:
                    Some(1)
                }
            },
            do_restart_after_steps: {
                if parts.len() > 3 && parts[3].len() > 0 {
                    if parts[3] == "true" {
                        true
                    } else {
                        false
                    }
                } else {
                    //default:
                    true
                }
            },
            restart_after_steps: {
                if parts.len() > 4 && parts[4].len() > 0 {
                    Some(parts[4].parse::<usize>().unwrap())
                } else {
                    //default: TODO coole Formel
                    Some(50)
                }
            },
            restart_possibility: {
                if parts.len() > 5 && parts[5].len() > 0 {
                    Some(parts[5].parse::<f64>().unwrap())
                } else {
                    //default: TODO coole Formel
                    Some(0.05)
                }
            },
            restart_scaling_factor: { //muss >1 sein
                if parts.len() > 6 && parts[6].len() > 0 {
                    parts[6].parse::<f64>().unwrap()
                } else {
                    //default:
                    1.2
                }
            },
            random_restart_possibility: { //prozent 0.0-1.0
                if parts.len() > 7 && parts[7].len() > 0 {
                    parts[7].parse::<f64>().unwrap()
                } else {
                    //default:
                    0.5
                }
            },
            lambda: {
                if parts.len() > 8 && parts[8].len() > 0 { //0.1 - inf
                    parts[8].parse::<f64>().unwrap()
                } else {
                    //default:
                    0.5
                }
            },
        })
    }
}
