use std::cmp::max;
use std::ops::Range;
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
use crate::schedulers::local_search::swapper::SwapAcceptanceRule::Improvement;
use crate::schedulers::local_search::swapper::SwapTactic::{TwoJobBruteForce, TwoJobRandomSwap};
use crate::schedulers::scheduler::Scheduler;

pub struct Swapper {
    input: Arc<Input>,
    global_bounds: Arc<Bounds>,
    good_solutions: Arc<Mutex<GoodSolutions>>,
}

impl Scheduler for Swapper {
    fn schedule(&mut self) -> Solution {
        self.swap(4..5, TwoJobRandomSwap, Improvement) //TODO PRIO als args in main iwi aufnehmen wsh am smartesten über Swap::new() übergeben ig
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
    Todo, //TODO weitere tactics... "XJobTacticY" zb random swap
}

///Rule when to accept a swap
#[derive(Clone)]
pub enum SwapAcceptanceRule {
    Improvement,
    Todo, //TODO weitere rules zb rand possibility bad swap / slightly bad swap +mischungen
}

impl Swapper {
    pub fn new(input: Arc<Input>, global_bounds: Arc<Bounds>, good_solutions: Arc<Mutex<GoodSolutions>>) -> Self {
        Self { input, global_bounds, good_solutions }
    }

    /// swaps jobs of on given schedule(s) to create better one(s)
    /// volume = amount of jobs to be swapped
    /// range = which schedules to pick from the currently best ones
    /// todo
    pub fn swap(&self, range: Range<usize>, swap_tactic: SwapTactic, swap_acceptance_rule: SwapAcceptanceRule) -> Solution { //TODO alles ausführlich testen (va. die methode hier)
        println!("running {:?} algorithm...", Swap); //todo (low prio) das kann man raus ziehen

        //New SwapTactics can be added here
        let swap: Box<dyn Fn(Solution, fn(u32, u32) -> bool) -> Solution> = match swap_tactic {
            TwoJobBruteForce => Box::new(|solution, swap_accepted| self.two_job_brute_force(solution, swap_accepted)),
            TwoJobRandomSwap => Box::new(|solution, swap_accepted| self.two_job_random_swap(solution, swap_accepted)),
            SwapTactic::Todo => todo!(),
        };
        //New SwapAcceptanceRules can be added here
        let swap_accepted = match swap_acceptance_rule { //todo (low prio) in extra methoden auslagern
            Improvement => { |old_c_max: u32, new_c_max: u32| new_c_max > old_c_max }
            SwapAcceptanceRule::Todo => { todo!() }
        };

        //get solutions:
        while self.good_solutions.lock().unwrap().get_solution_count() < range.end { //todo active waiting vllt mit thread_pool.yield oder soo(?)
            sleep(Duration::from_millis(10));
        }
        //let mut solutions = self.good_solutions.lock().unwrap().get_cloned_solutions(range);

        rayon::scope(|s| {
            for i in range {
                s.spawn(move |_| {
                    let binding = self.good_solutions.lock().unwrap().get_solution(i);
                    let mut solution = binding.lock().unwrap();
                    loop { //TODO params hinzufügen um zu steuern ob man ne tactic um aus local min zu kommen machen will oder net
                        println!("(todo schöner loggen)curr c_max={}", solution.get_data().get_c_max());
                        //todo noch bissel unschön mit match hier aber das funzt leider net (let new_solution = swap(solution.clone(), swap_accepted);)
                        let new_solution = match swap_tactic {
                            TwoJobBruteForce => { self.two_job_brute_force(solution.clone(), swap_accepted) }
                            TwoJobRandomSwap => { self.two_job_random_swap(solution.clone(), swap_accepted) }
                            SwapTactic::Todo => { todo!() }
                        };
                        if !new_solution.is_satisfiable() { break; } // did not find a swap
                        *solution = new_solution;
                    }
                });
            }
        });

        Solution::unsatisfiable(Swap) //TODO rückgabe zu vec<solution> umbauen und alle ausgeben
    }

    /// 2 job swap brute force (try all possible swaps)
    fn two_job_brute_force(&self, mut solution: Solution, swap_accepted: fn(u32, u32) -> bool) -> Solution { //TODO solution.algorithm als vec arg machen damit man hier swap hinzufügen kann
        let machine_jobs = solution.get_data().get_machine_jobs();
        let mut current_c_max = solution.get_data().get_c_max();
        let current_heaviest_machines = solution.get_data().get_machine_jobs().get_machines_with_workload(current_c_max);
        let mut swap_indices: (usize, usize, usize, usize) = (0, 0, 0, 0);//(machine_1_index, job_1_index, machine_2_index, job_2_index)
        let mut swap_found = false;

        for m1 in 0..self.input.get_machine_count() {
            for m2 in m1..self.input.get_machine_count() { //for all machine pairs {m1,m2}
                if current_heaviest_machines.contains(&m1) || current_heaviest_machines.contains(&m2) { //todo low prio weitere einschränkungen wie zb current_heaviest_machines.len() = 1/2 oder so(?)
                    //only in this case we can improve our c_max
                    let machine_1_jobs = machine_jobs.get_machine_jobs(m1);
                    let machine_2_jobs = machine_jobs.get_machine_jobs(m2);
                    for j1 in 0..machine_1_jobs.len() {
                        for j2 in 0..machine_2_jobs.len() { //for all job pairs (j1,j2) on (m1,m2)
                            let new_c_max = self.simulate_two_job_swap(m1, machine_1_jobs[j1], m2, machine_2_jobs[j2], machine_jobs, current_heaviest_machines.as_slice());
                            if swap_accepted(new_c_max, current_c_max) {
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
            solution.get_mut_data().swap_jobs(swap_indices.0, swap_indices.1, swap_indices.2, swap_indices.3, self.input.get_jobs(), self.input.get_machine_count());
            solution
        } else {
            Solution::unsatisfiable(Swap)
        }
    }

    /// 2 job random swap
    fn two_job_random_swap(&self, mut solution: Solution, swap_accepted: fn(u32, u32) -> bool) -> Solution {
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

            //actual swap
            let new_c_max = self.simulate_two_job_swap(m1, machine_1_jobs[j1], m2, machine_2_jobs[j2], machine_jobs, current_heaviest_machines.as_slice());
            if swap_accepted(new_c_max, current_c_max) {
                solution.get_mut_data().swap_jobs(m1, j1, m2, j2, self.input.get_jobs(), self.input.get_machine_count());
                return solution;
            } else {
                fails += 1;
                if fails == 50 {
                    return Solution::unsatisfiable(Swap);
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