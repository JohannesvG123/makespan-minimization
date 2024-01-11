use std::cmp::max;
use std::sync::Arc;
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
use crate::schedulers::local_search::swapper::SwapAcceptanceRule::{All, ChanceDecline, Improvement};
use crate::schedulers::local_search::swapper::SwapTactic::{TwoJobBruteForce, TwoJobRandomSwap};
use crate::schedulers::scheduler::Scheduler;

pub struct Swapper {
    //TODO UB immer aktualisieren
    input: Arc<Input>,
    global_bounds: Arc<Bounds>,
    swap_tactic: fn(&Swapper, Solution) -> Solution,
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
    Todo,
}

///Rule when to accept a swap
#[derive(Clone)]
pub enum SwapAcceptanceRule {
    ///accept swap if it improves c_max
    Improvement,
    ///accept improvements & declines with a p-percent chance
    ChanceDecline,
    //(f64)

    //accept improvements & x-percent declines TODO diese als ideen stehen lassen
    //SmallDecline, //(f64)

    //accept improvements & declines with a p-percent chance //TODO PRIOOOOOO andere erstmal unwichtiger
    //SmallAndChanceDecline, //(f64)

    //accept improvements & x-percent declines with a p-percent chance (smaller decline => higher chance; bigger decline => smaller chance)
    //WeightedDecline, //(f64,f64)
    ///accept all swaps independent of c_max
    All,
}

impl Swapper {
    pub fn new(input: Arc<Input>, global_bounds: Arc<Bounds>, swap_tactic: SwapTactic, swap_acceptance_rule: SwapAcceptanceRule, number_of_solutions: usize) -> Self {
        //new swap tactics can be added here:
        let swap_tactic_fn = match swap_tactic {
            TwoJobBruteForce => { Self::two_job_brute_force }
            TwoJobRandomSwap => { Self::two_job_random_swap }
            SwapTactic::Todo => { todo!() }
        };

        //new swap acceptance rules can be added here:
        let swap_acceptance_rule_fn = match swap_acceptance_rule {
            Improvement => { Self::accept_improvement }
            ChanceDecline => { Self::accept_decline_chance_tmp }
            All => { Self::accept_all }
        };

        Self { input, global_bounds, swap_tactic: swap_tactic_fn, swap_acceptance_rule: swap_acceptance_rule_fn, number_of_solutions }
    }

    fn accept_improvement(old_c_max: u32, new_c_max: u32) -> bool {
        new_c_max > old_c_max
    }

    fn accept_decline_chance_tmp(old_c_max: u32, new_c_max: u32) -> bool {
        Self::accept_decline_chance(old_c_max, new_c_max, 0.1)
    }
    fn accept_decline_chance(old_c_max: u32, new_c_max: u32, percentage: f64) -> bool {//TODO find out how to use this with DeclineChance(percentage) => dafür Unterschied closure/fn/Fn usw anschauen
        debug_assert!(0f64 <= percentage);
        debug_assert!(1f64 >= percentage);

        if new_c_max > old_c_max {
            true
        } else {
            let mut rng = rand::thread_rng();
            rng.gen_bool(percentage)
        }
    }

    fn accept_all(old_c_max: u32, new_c_max: u32) -> bool {
        true
    }

    /// swaps jobs of specified good solution(s) to create better one(s)
    fn swap(&self, good_solutions: GoodSolutions) -> Solution {
        println!("running {:?} algorithm...", Swap); //todo (low prio) das kann man raus ziehen

        //get solutions:
        while good_solutions.get_solution_count() < self.number_of_solutions { //TODO waiting so überhaupt nötig?
            sleep(Duration::from_millis(10));
            //todo (low prio) logging was passiert und iwan abbruch
        }

        let out = rayon::scope(move |s| {
            let old_solutions = Arc::new(good_solutions.get_best_solutions(self.number_of_solutions));
            //let new_solutions = vec![];


            for i in 0..self.number_of_solutions {
                let old_solutions = Arc::clone(&old_solutions);
                let good_solutions = good_solutions.clone();
                s.spawn(move |_| {
                    let mut solution = old_solutions[i].clone();

                    loop { //TODO (low prio) params hinzufügen um zu steuern ob man ne tactic um aus local min zu kommen machen will oder net (2.erst wenn kein guter mehr gefunden wird schlechten erlauben 2.1 den am wenigsten schlechten 2.2 random one 2.3 einen der maximal x% schlechter ist (was wählt man für ein x?))
                        println!("(todo schöner loggen)curr c_max={}", solution.get_data().get_c_max());
                        let mut new_solution = (self.swap_tactic)(self, solution.clone()); //TODO low prio: so umbauen dass nicht so oft gecloned werden muss durch neue rückgabeargs der tactics wenn sie nix finden
                        if !new_solution.is_satisfiable() {// did not find a swap:
                            break;
                        }

                        solution = new_solution; //TODO (wenn good solutions gescheit gemacht) ist hier solution entweder ändern oder neu speichern (eher das zweite)
                    }

                    solution.add_algorithm(Swap);
                    good_solutions.add_solution(solution);
                });
            }

            old_solutions
        });

        out.first().unwrap().clone() //TODO (wenn good solutions gescheit gemacht ist) rückgabe zu vec<solution> umbauen und alle ausgeben
    }

    /// 2 job swap brute force (try all possible swaps)
    /// Attention: solution gets mutated!
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