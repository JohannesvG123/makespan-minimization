use std::cmp::max;
use std::fmt::Debug;
use std::ops::Range;
use std::str::FromStr;
use std::string::ParseError;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, Instant};

use atoi::atoi;
use clap::ValueEnum;
use permutation::Permutation;
use rand::{Rng, SeedableRng, thread_rng};
use rand_chacha::ChaCha8Rng;
use regex::Regex;

use crate::{Algorithm, Args};
use crate::Algorithm::Swap;
use crate::global_bounds::bounds::Bounds;
use crate::good_solutions::good_solutions::GoodSolutions;
use crate::input::input::Input;
use crate::input::seed_from_str;
use crate::output::log;
use crate::output::machine_jobs::MachineJobs;
use crate::output::solution::Solution;
use crate::schedulers::local_search::swapper::SwapAcceptanceRule::{All, DeclineByChance, Improvement, SimulatedAnnealing};
use crate::schedulers::local_search::swapper::SwapTactic::{TwoJobBruteForce, TwoJobRandomSwap};
use crate::schedulers::scheduler::Scheduler;

pub struct Swapper {
    input: Arc<Input>,
    global_bounds: Arc<Bounds>,
    config: SwapConfig,
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
    pub fn new(input: Arc<Input>, global_bounds: Arc<Bounds>, config: SwapConfig) -> Self {
        Self {
            input,
            global_bounds,
            config,
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
            concrete_swap_config.rng_gen_bool(percentage)
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
        log(format!("running {:?} algorithm...", Swap), false, args.measurement);

        let best_solution_for_output = Arc::new(Mutex::new(Solution::unsatisfiable(Swap)));
        let best_solution_for_threads = Arc::clone(&best_solution_for_output);
        rayon::scope(move |s| {
            //get solutions:
            while good_solutions.get_solution_count() < self.config.number_of_solutions {
                //TODO 1 should terminate methode hier aufrufen (iwan abbruch -> durch cmd arg spezifizieren)
                sleep(Duration::from_millis(100));
                log(String::from("waiting for enough good solutions to run Swap algorithm..."), false, args.measurement);
            }

            let old_solutions = Arc::new(good_solutions.get_best_solutions(self.config.number_of_solutions)); //TODO (low prio) version einbauen mit eingabe von Solution auf der gearbeitet wird (zb RF laufen lassen und iwan dann swap drauf schmeißen) => bei den List schedulern ein bool hinzufügen ob das gemacht werden soll oder nicht
            let best_c_max = old_solutions.last().unwrap().get_data().get_c_max();

            for i in 0..self.config.number_of_solutions {
                let old_solutions = Arc::clone(&old_solutions);
                let mut best_solution = Arc::clone(&best_solution_for_threads);
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
                    let rng = match self.config.rng_seed {
                        None => { None }
                        Some(seed) => { Some(ChaCha8Rng::from_seed(seed)) }
                    };

                    let mut concrete_swap_config = ConcreteSwapConfig {
                        swap_finding_tactic: swap_finding_tactic_fn,
                        swap_acceptance_rule: swap_acceptance_rule_fn,
                        decline_by_chance_percentage,
                        random_swap_fails_until_stop,
                        rng,
                        number_of_solutions: self.config.number_of_solutions,
                    };

                    let mut solution = old_solutions[i].clone();

                    while let Some(swap_indices) = (concrete_swap_config.swap_finding_tactic)(self, &solution, &mut concrete_swap_config) {
                        solution.swap_jobs(swap_indices, self.input.get_jobs(), self.input.get_machine_count(), Arc::clone(&self.global_bounds), Arc::clone(&args), Arc::clone(&perm), start_time);
                        self.global_bounds.update_upper_bound(solution.get_data().get_c_max(), &solution, args.clone(), perm.clone(), start_time); //todo .clone ><
                    }

                    solution.add_algorithm(Swap);
                    solution.add_config(format!("{:?}", self.config)); //TODO (low prio) vllt display implementieren für die config
                    if solution.get_data().get_c_max() <= best_c_max { //this is only used for the output of the method
                        let mut bs = best_solution.lock().unwrap();
                        *bs = solution.clone();
                    }
                    good_solutions.add_solution(solution);
                });
            }
        });

        Arc::into_inner(best_solution_for_output).unwrap().into_inner().unwrap()
    }

    /// 2 job swap brute force (try all possible swaps)
    fn find_brute_force_two_job_swap(&self, solution: &Solution, concrete_swap_config: &mut ConcreteSwapConfig) -> Option<(usize, usize, usize, usize)> {
        let machine_jobs = solution.get_data().get_machine_jobs();
        let mut current_c_max = solution.get_data().get_c_max();
        let current_heaviest_machines = solution.get_data().get_machine_jobs().get_machines_with_workload(current_c_max);
        let mut swap_indices: (usize, usize, usize, usize) = (0, 0, 0, 0); //(machine_1_index, job_1_index, machine_2_index, job_2_index)
        let mut swap_found = false;

        for m1 in 0..self.input.get_machine_count() {
            for m2 in m1..self.input.get_machine_count() {
                //for all machine pairs {m1,m2}
                if (current_heaviest_machines.len() == 1 && (current_heaviest_machines.contains(&m1) || current_heaviest_machines.contains(&m2))) || (current_heaviest_machines.len() == 2 && current_heaviest_machines.contains(&m1) && current_heaviest_machines.contains(&m2)) {
                    //vorherige (leichtere Bdg.): current_heaviest_machines.contains(&m1) || current_heaviest_machines.contains(&m2)
                    //only in this case we can improve our c_max
                    let machine_1_jobs = machine_jobs.get_machine_jobs(m1);
                    let machine_2_jobs = machine_jobs.get_machine_jobs(m2);
                    for j1 in 0..machine_1_jobs.len() {
                        for j2 in 0..machine_2_jobs.len() {
                            //for all job pairs (j1,j2) on (m1,m2)
                            let new_c_max = self.simulate_two_job_swap(
                                m1,
                                machine_1_jobs[j1],
                                m2,
                                machine_2_jobs[j2],
                                machine_jobs,
                                current_heaviest_machines.as_slice(),
                            );
                            if (concrete_swap_config.swap_acceptance_rule)(new_c_max, current_c_max, concrete_swap_config) {
                                swap_found = true;
                                current_c_max = new_c_max;
                                swap_indices = (m1, j1, m2, j2);
                            }
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
            let mut m1 = concrete_swap_config.rng_gen_range((0..machine_count));
            let mut machine_1_jobs = machine_jobs.get_machine_jobs(m1);
            while machine_1_jobs.len() == 0 {
                // in case the machine is not used for the schedule
                m1 = concrete_swap_config.rng_gen_range(0..machine_count);
                machine_1_jobs = machine_jobs.get_machine_jobs(m1);
            }
            let mut m2 = concrete_swap_config.rng_gen_range(0..machine_count);
            let mut machine_2_jobs = machine_jobs.get_machine_jobs(m2);
            while m2 == m1 || machine_2_jobs.len() == 0 {
                //cant swap from the same machine
                m2 = concrete_swap_config.rng_gen_range(0..machine_count);
                machine_2_jobs = machine_jobs.get_machine_jobs(m2);
            }
            let j1 = concrete_swap_config.rng_gen_range(0..machine_1_jobs.len());
            let j2 = concrete_swap_config.rng_gen_range(0..machine_2_jobs.len());

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
                    //todo 1 (logging error )
                    println!("TODO error reached {} fails (2JobRandomSwap)", fails_until_stop);
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
    number_of_solutions: usize,
    rng_seed: Option<[u8; 32]>,
}

#[derive(Clone, Debug)]
pub struct ConcreteSwapConfig {
    swap_finding_tactic: fn(&Swapper, &Solution, &mut ConcreteSwapConfig) -> Option<(usize, usize, usize, usize)>,
    swap_acceptance_rule: fn(u32, u32, &mut ConcreteSwapConfig) -> bool,
    decline_by_chance_percentage: Option<u8>,
    random_swap_fails_until_stop: Option<(usize)>,
    rng: Option<ChaCha8Rng>,
    number_of_solutions: usize,
}

impl ConcreteSwapConfig {
    pub fn rng_gen_range(&mut self, range: Range<usize>) -> usize {
        match &mut self.rng {
            None => {
                todo!("errorrr not reachable1")
            }
            Some(r) => {
                r.gen_range(range)
            }
        }
    }

    pub fn rng_gen_bool(&mut self, p: f64) -> bool {
        match &mut self.rng {
            None => {
                todo!("errorrr not reachable2")
            }
            Some(r) => {
                r.gen_bool(p)
            }
        }
    }
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
                    parts[2].parse::<usize>().unwrap()
                } else {
                    //default:
                    1
                }
            },
            rng_seed: { //TODO ACHTUNG  ,decline-by-32%-chance, wirft fehler und  ,decline-by-32%-chance,, nicht (so lassen oder ändern?)
                if parts.len() > 3 { //nur bei ,,, wird seed generiert (bei ,,nicht!)
                    if parts[3].len() > 0 {
                        Some(seed_from_str(parts[3]))
                    } else {
                        //default: random seed
                        let mut seed: <ChaCha8Rng as SeedableRng>::Seed = Default::default();
                        thread_rng().fill(&mut seed);
                        Some(seed)
                    }
                } else {
                    None
                }
            },
        })
    }
}
