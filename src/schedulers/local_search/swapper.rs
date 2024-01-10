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
use crate::schedulers::local_search::swapper::SwapAcceptanceRule::{Improvement, Random};
use crate::schedulers::local_search::swapper::SwapTactic::{TwoJobBruteForce, TwoJobRandomSwap};
use crate::schedulers::scheduler::Scheduler;

pub struct Swapper {
    input: Arc<Input>,
    global_bounds: Arc<Bounds>,
    good_solutions: Arc<Mutex<GoodSolutions>>,
    swap_tactic: fn(&Swapper, Solution) -> Solution,
    swap_acceptance_rule: fn(u32, u32) -> bool,
    range: Range<usize>,
}

impl Scheduler for Swapper {
    fn schedule(&mut self) -> Solution {
        self.swap()
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
    Todo,
}

///Rule when to accept a swap
#[derive(Clone)]
pub enum SwapAcceptanceRule {
    Improvement,
    Random,
    Todo,
}

impl Swapper {
    pub fn new(input: Arc<Input>, global_bounds: Arc<Bounds>, good_solutions: Arc<Mutex<GoodSolutions>>, swap_tactic: SwapTactic, swap_acceptance_rule: SwapAcceptanceRule, range: Range<usize>) -> Self {
        //new swap tactics can be added here:
        let swap_tactic_fn = match swap_tactic {
            TwoJobBruteForce => { Self::two_job_brute_force }
            TwoJobRandomSwap => { Self::two_job_random_swap }
            SwapTactic::Todo => { todo!() }
        };

        //new swap acceptance rules can be added here:
        let swap_acceptance_rule_fn = match swap_acceptance_rule { //todo (low prio) in methoden auslagern
            Improvement => { |old_c_max: u32, new_c_max: u32| new_c_max > old_c_max }
            Random => {
                |old_c_max: u32, new_c_max: u32| {
                    let mut rng = rand::thread_rng();
                    rng.gen_bool(0.5)
                }
            }
            SwapAcceptanceRule::Todo => { todo!() }
        };

        Self { input, global_bounds, good_solutions, swap_tactic: swap_tactic_fn, swap_acceptance_rule: swap_acceptance_rule_fn, range }
    }

    /// swaps jobs of on given schedule(s) to create better one(s)
    /// range = which schedules to pick from the currently best ones
    pub fn swap(&self) -> Solution {
        println!("running {:?} algorithm...", Swap); //todo (low prio) das kann man raus ziehen

        let range = self.range.clone();//TODO (low prio) unten direkt self. verwenden

        //get solutions:
        while self.good_solutions.lock().unwrap().get_solution_count() < range.end {
            sleep(Duration::from_millis(10));
            //todo (low prio) logging was passiert und iwan abbruch
        }
        //let mut solutions = self.good_solutions.lock().unwrap().get_cloned_solutions(range);
        let tmp_todo = Arc::new(Mutex::new(Solution::unsatisfiable(Swap)));
        rayon::scope(|s| {
            for i in range {
                let tmp_todo = Arc::clone(&tmp_todo);
                s.spawn(move |_| {
                    let binding = self.good_solutions.lock().unwrap().get_solution(i);
                    let mut solution = binding.lock().unwrap();
                    loop { //TODO (low prio) params hinzufügen um zu steuern ob man ne tactic um aus local min zu kommen machen will oder net (2.erst wenn kein guter mehr gefunden wird schlechten erlauben 2.1 den am wenigsten schlechten 2.2 random one 2.3 einen der maximal x% schlechter ist (was wählt man für ein x?))
                        println!("(todo schöner loggen)curr c_max={}", solution.get_data().get_c_max());
                        let mut new_solution = (self.swap_tactic)(self, solution.clone());
                        if !new_solution.is_satisfiable() {// did not find a swap:
                            break;
                        }
                        new_solution.add_algorithm(Swap);

                        *solution = new_solution; //TODO (wenn good solutions gescheit gemacht) ist hier solution entweder ändern oder neu speichern (eher das zweite)
                    }
                    let mut b = tmp_todo.lock().unwrap(); //todo (low prio) todo dings weg wenn good solutions steht
                    *b = solution.clone();
                });
            }
        });

        Arc::into_inner(tmp_todo).unwrap().into_inner().unwrap() //TODO (wenn good solutions gescheit gemacht ist) rückgabe zu vec<solution> umbauen und alle ausgeben
    }

    /// 2 job swap brute force (try all possible swaps)
    fn two_job_brute_force(&self, mut solution: Solution) -> Solution {
        let machine_jobs = solution.get_data().get_machine_jobs();
        let mut current_c_max = solution.get_data().get_c_max();
        let current_heaviest_machines = solution.get_data().get_machine_jobs().get_machines_with_workload(current_c_max);
        let mut swap_indices: (usize, usize, usize, usize) = (0, 0, 0, 0);//(machine_1_index, job_1_index, machine_2_index, job_2_index)
        let mut swap_found = false;

        for m1 in 0..self.input.get_machine_count() {
            for m2 in m1..self.input.get_machine_count() { //for all machine pairs {m1,m2}
                if current_heaviest_machines.contains(&m1) || current_heaviest_machines.contains(&m2) { //todo (low prio) weitere einschränkungen wie zb current_heaviest_machines.len() = 1/2 oder so(?)
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
            solution.get_mut_data().swap_jobs(swap_indices.0, swap_indices.1, swap_indices.2, swap_indices.3, self.input.get_jobs(), self.input.get_machine_count());
            solution
        } else {
            Solution::unsatisfiable(Swap)
        }
    }

    /// 2 job random swap
    fn two_job_random_swap(&self, mut solution: Solution) -> Solution {
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
            if (self.swap_acceptance_rule)(new_c_max, current_c_max) {
                solution.get_mut_data().swap_jobs(m1, j1, m2, j2, self.input.get_jobs(), self.input.get_machine_count());
                return solution;
            } else {
                fails += 1;
                if fails == 50 {//todo (low prio) logging
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