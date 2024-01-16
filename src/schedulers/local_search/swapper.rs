use std::cmp::max;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use rand::Rng;

use crate::Algorithm;
use crate::Algorithm::Swap;
use crate::global_bounds::bounds::Bounds;
use crate::good_solutions::good_solutions::GoodSolutions;
use crate::input::input::Input;
use crate::output::machine_jobs::MachineJobs;
use crate::output::solution::Solution;
use crate::schedulers::local_search::swapper::SwapAcceptanceRule::{All, DeclineByChance, Improvement, SimulatedAnnealing};
use crate::schedulers::local_search::swapper::SwapTactic::{TwoJobBruteForce, TwoJobRandomSwap};
use crate::schedulers::scheduler::Scheduler;

pub struct Swapper {
    input: Arc<Input>,
    global_bounds: Arc<Bounds>,
    swap_finding_tactic: fn(&Swapper, &Solution) -> Option<(usize, usize, usize, usize)>,
    swap_acceptance_rule: fn(u32, u32) -> bool,
    number_of_solutions: usize,
}

impl Scheduler for Swapper {
    fn schedule(&mut self, good_solutions: GoodSolutions) -> Solution {
        self.swap(good_solutions)
    }

    fn get_algorithm(&self) -> Algorithm {
        Swap
    }
}

///Tactic to find jobs to swap
#[derive(Clone, Copy)]
pub enum SwapTactic {
    TwoJobBruteForce,
    TwoJobRandomSwap,
}

///Rule when to accept a swap
#[derive(Clone)]
pub enum SwapAcceptanceRule {
    ///accept swap if it improves c_max
    Improvement,
    ///accept improvements & declines with a p-percent chance
    DeclineByChance(f64),
    ///accept improvements & declines with ...todo (-> https://de.wikipedia.org/wiki/Simulated_Annealing)
    SimulatedAnnealing,

    ///accept all swaps independent of c_max
    All,
}

impl Swapper {
    pub fn new(input: Arc<Input>, global_bounds: Arc<Bounds>, swap_tactic: SwapTactic, swap_acceptance_rule: SwapAcceptanceRule, number_of_solutions: usize) -> Self {
        //new swap tactics can be added here:
        let swap_finding_tactic_fn = match swap_tactic {
            TwoJobBruteForce => { Self::find_brute_force_two_job_swap }
            TwoJobRandomSwap => { Self::find_random_two_job_swap }
        };

        //new swap acceptance rules can be added here:
        let swap_acceptance_rule_fn = match swap_acceptance_rule {
            Improvement => { Self::accept_improvement }
            DeclineByChance(percentage) => { Self::accept_decline_by_chance_tmp } //TODO 1 den parameter mit aufnehmen...
            SimulatedAnnealing => { todo!() }
            All => { Self::accept_all }
        };


        Self { input, global_bounds, swap_finding_tactic: swap_finding_tactic_fn, swap_acceptance_rule: swap_acceptance_rule_fn, number_of_solutions }
    }

    fn accept_improvement(old_c_max: u32, new_c_max: u32) -> bool {
        new_c_max > old_c_max
    }

    fn accept_decline_by_chance_tmp(old_c_max: u32, new_c_max: u32) -> bool {
        Self::accept_decline_by_chance(old_c_max, new_c_max, 0.1)
    }
    fn accept_decline_by_chance(old_c_max: u32, new_c_max: u32, percentage: f64) -> bool {
        debug_assert!(0f64 <= percentage);
        debug_assert!(1f64 >= percentage);

        if new_c_max > old_c_max {
            true
        } else {
            let mut rng = rand::thread_rng();
            rng.gen_bool(percentage)
        }
    }

    fn accept_all(_old_c_max: u32, _new_c_max: u32) -> bool {
        true
    }

    /// swaps jobs of specified good solution(s) to create better one(s)
    /// the newly created solutions get stored in good_solutions
    /// the best one gets returned
    fn swap(&self, good_solutions: GoodSolutions) -> Solution {
        println!("running {:?} algorithm...", Swap);

        let best_solution_for_output = Arc::new(Mutex::new(Solution::unsatisfiable(Swap)));
        let best_solution_for_threads = Arc::clone(&best_solution_for_output);


        rayon::scope(move |s| {
            //get solutions:
            while good_solutions.get_solution_count() < self.number_of_solutions { //TODO 1 should terminate methode hier aufrufen (iwan abbruch)
                sleep(Duration::from_millis(10));
                println!("zzzZzzZzzZzz")
                //todo 1 (logging)
            }

            let old_solutions = Arc::new(good_solutions.get_best_solutions(self.number_of_solutions)); //TODO (low prio) version einbauen mit eingabe von Solution auf der gearbeitet wird (zb RF laufen lassen und iwan dann swap drauf schmeißen) => bei den List schedulern ein bool hinzufügen ob das gemacht werden soll oder nicht
            let best_c_max = old_solutions.last().unwrap().get_data().get_c_max();

            for i in 0..self.number_of_solutions {
                let old_solutions = Arc::clone(&old_solutions);
                let mut best_solution = Arc::clone(&best_solution_for_threads);
                let good_solutions = good_solutions.clone();
                s.spawn(move |_| {
                    let mut solution = old_solutions[i].clone();

                    //todo 1 (low prio) params hinzufügen um zu steuern ob man ne tactic um aus local min zu kommen machen will oder net (2.erst wenn kein guter mehr gefunden wird schlechten erlauben 2.1 den am wenigsten schlechten 2.2 random one 2.3 einen der maximal x% schlechter ist (was wählt man für ein x?))
                    while let Some(swap_indices) = (self.swap_finding_tactic)(self, &solution) {
                        solution.get_mut_data().swap_jobs(swap_indices, self.input.get_jobs(), self.input.get_machine_count(), Arc::clone(&self.global_bounds));
                    }

                    solution.add_algorithm(Swap);
                    if solution.get_data().get_c_max() <= best_c_max {
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
    fn find_brute_force_two_job_swap(&self, solution: &Solution) -> Option<(usize, usize, usize, usize)> {
        let machine_jobs = solution.get_data().get_machine_jobs();
        let mut current_c_max = solution.get_data().get_c_max();
        let current_heaviest_machines = solution.get_data().get_machine_jobs().get_machines_with_workload(current_c_max);
        let mut swap_indices: (usize, usize, usize, usize) = (0, 0, 0, 0);//(machine_1_index, job_1_index, machine_2_index, job_2_index)
        let mut swap_found = false;

        for m1 in 0..self.input.get_machine_count() {
            for m2 in m1..self.input.get_machine_count() { //for all machine pairs {m1,m2}
                if (current_heaviest_machines.len() == 1 && (current_heaviest_machines.contains(&m1) || current_heaviest_machines.contains(&m2))) || (current_heaviest_machines.len() == 2 && current_heaviest_machines.contains(&m1) && current_heaviest_machines.contains(&m2)) { //vorherige (leichtere Bdg.): current_heaviest_machines.contains(&m1) || current_heaviest_machines.contains(&m2)
                    //only in this case we can improve our c_max
                    let machine_1_jobs = machine_jobs.get_machine_jobs(m1);
                    let machine_2_jobs = machine_jobs.get_machine_jobs(m2);
                    for j1 in 0..machine_1_jobs.len() {
                        for j2 in 0..machine_2_jobs.len() { //for all job pairs (j1,j2) on (m1,m2)
                            let new_c_max = self.simulate_two_job_swap(m1, machine_1_jobs[j1], m2, machine_2_jobs[j2], machine_jobs, current_heaviest_machines.as_slice());
                            if (self.swap_acceptance_rule)(new_c_max, current_c_max) {
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
    fn find_random_two_job_swap(&self, solution: &Solution) -> Option<(usize, usize, usize, usize)> {//todo 1 , fails_until_stop:u8 als param mit aufnehmen -> dann auch bei RF
        let mut rng = rand::thread_rng();
        let mut fails: u8 = 0;

        let machine_count = self.input.get_machine_count();
        let machine_jobs = solution.get_data().get_machine_jobs();
        let current_c_max = solution.get_data().get_c_max();
        let current_heaviest_machines = solution.get_data().get_machine_jobs().get_machines_with_workload(current_c_max);

        loop {
            //generate random values
            let mut m1 = rng.gen_range(0..machine_count);
            let mut machine_1_jobs = machine_jobs.get_machine_jobs(m1);
            while machine_1_jobs.len() == 0 { // in case the machine is not used for the schedule
                m1 = rng.gen_range(0..machine_count);
                machine_1_jobs = machine_jobs.get_machine_jobs(m1);
            }
            let mut m2 = rng.gen_range(0..machine_count);
            let mut machine_2_jobs = machine_jobs.get_machine_jobs(m2);
            while m2 == m1 || machine_2_jobs.len() == 0 { //cant swap from the same machine
                m2 = rng.gen_range(0..machine_count);
                machine_2_jobs = machine_jobs.get_machine_jobs(m2);
            }
            let j1 = rng.gen_range(0..machine_1_jobs.len());
            let j2 = rng.gen_range(0..machine_2_jobs.len());

            //check swap
            let new_c_max = self.simulate_two_job_swap(m1, machine_1_jobs[j1], m2, machine_2_jobs[j2], machine_jobs, current_heaviest_machines.as_slice());
            if (self.swap_acceptance_rule)(new_c_max, current_c_max) {
                return Some((m1, j1, m2, j2));
            } else {
                fails += 1;
                if fails == 50 {//todo 1 (logging)
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