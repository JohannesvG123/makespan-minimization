use std::cmp::max;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use crate::Algorithm;
use crate::Algorithm::{Swap};
use crate::global_bounds::bounds::Bounds;
use crate::good_solutions::good_solutions::GoodSolutions;
use crate::input::input::Input;
use crate::output::machine_jobs::MachineJobs;
use crate::output::solution::Solution;
use crate::schedulers::scheduler::Scheduler;

pub struct Swapper {
    input: Arc<Input>,
    global_bounds: Arc<Bounds>,
    good_solutions: Arc<Mutex<GoodSolutions>>,
}

impl Scheduler for Swapper {
    fn schedule(&mut self) -> Solution {
        self.swap()
    }

    fn get_algorithm(&self) -> Algorithm {
        Algorithm::Swap
    }
}

impl Swapper {
    pub fn new(input: Arc<Input>, global_bounds: Arc<Bounds>, good_solutions: Arc<Mutex<GoodSolutions>>) -> Self {
        Self { input, global_bounds, good_solutions }
    }

    /// swaps 2 jobs of a given schedule to create a better one
    pub fn swap(&self) -> Solution { //TODO alles ausführlich testen (va. die methode hier)
        println!("running {:?} algorithm...", Swap); //todo (low prio) das kann man raus ziehen

        while self.good_solutions.lock().unwrap().get_solution_count() == 0 { //todo active waiting vllt mit thread_pool.yield oder soo(?)
            sleep(Duration::from_millis(10));
        }

        //momentan mit der schlechtesten besten lsg -> todo 1 einstellbar machen
        let i = self.good_solutions.lock().unwrap().get_solution_count() - 1;
        let mut current_solution = self.good_solutions.lock().unwrap().get_solution(i).lock().unwrap().clone();

        loop {
            println!("curr c_max={}", current_solution.get_data().get_c_max());
            let new_solution = self.swap_tactic_1(&current_solution);
            if new_solution.is_satisfiable() {
                current_solution = new_solution;
            } else {
                return current_solution;
            }
        }
    }

    /// brute force (try all possible swaps)
    fn swap_tactic_1(&self, mut solution: &Solution) -> Solution { //TODO solution.algorithm als vec arg machen damit man hier swap hinzufügen kann
        let mut solution = solution.clone();

        let machine_jobs = solution.get_data().get_machine_jobs();
        let mut current_c_max = solution.get_data().get_c_max();
        let current_heaviest_machines = solution.get_data().get_machine_jobs().get_machines_with_workload(current_c_max);
        let mut swap_indices: (usize, usize, usize, usize) = (0, 0, 0, 0);//(machine_1_index, job_1_index, machine_2_index, job_2_index)
        let mut swap_found = false;

        for m1 in 0..self.input.get_machine_count() {
            for m2 in m1..self.input.get_machine_count() { //for all machine pairs {m1,m2}
                let machine_1_jobs = machine_jobs.get_machine_jobs(m1);
                let machine_2_jobs = machine_jobs.get_machine_jobs(m2);
                for j1 in 0..machine_1_jobs.len() {
                    for j2 in 0..machine_2_jobs.len() { //for all job pairs (j1,j2) on (m1,m2)
                        let new_c_max = self.simulate_swap(m1, machine_1_jobs[j1], m2, machine_2_jobs[j2], machine_jobs, current_heaviest_machines.as_slice());
                        if new_c_max < current_c_max {
                            swap_found = true;
                            current_c_max = new_c_max;
                            swap_indices = (m1, j1, m2, j2);
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

    fn swap_tactic_n(&self) -> Solution {
        todo!()
    }

    ///computes the c_max that the current solution would have after a specified swap
    fn simulate_swap(&self, machine_1_index: usize, job_1_index: usize, machine_2_index: usize, job_2_index: usize, machine_jobs: &MachineJobs, current_heaviest_machines: &[usize]) -> u32 {
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