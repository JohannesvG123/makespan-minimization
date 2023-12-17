use std::cmp::max;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use crate::Algorithm;
use crate::Algorithm::Swap;
use crate::global_bounds::bounds::Bounds;
use crate::good_solutions::good_solutions::GoodSolutions;
use crate::input::input::Input;
use crate::output::machine_jobs::MachineJobs;
use crate::output::solution::Solution;
use crate::schedulers::scheduler::Scheduler;

pub struct Swapper {
    input: Arc<Input>,
    global_bounds: Arc<Mutex<Bounds>>,
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
    pub fn new(input: Arc<Input>, global_bounds: Arc<Mutex<Bounds>>, good_solutions: Arc<Mutex<GoodSolutions>>) -> Self {
        Self { input, global_bounds, good_solutions }
    }

    /// todo erklärung was passiert
    pub fn swap(&self) -> Solution {
        while self.good_solutions.lock().unwrap().get_solution_count() == 0 { //todo active waiting vllt mit thread_pool.yield oder soo(?)
            sleep(Duration::from_millis(10));
        }

        //momentan mit der besten lsg
        self.swap_tactic_1(self.good_solutions.lock().unwrap().get_best_solution().lock().unwrap().clone()) //clone weil wir neue solution erzeugen
    }

    /// brute force (try all possible swaps)
    fn swap_tactic_1(&self, mut solution: Solution) -> Solution { //TODO solution.algorithm als vec arg machen damit man hier swap hinzufügen kann(?)
        let machine_jobs = solution.get_data().get_machine_jobs();
        let mut current_c_max = solution.get_data().get_c_max();
        let mut swap_indices: (usize, usize, usize, usize) = (0, 0, 0, 0);//(machine_1_index, job_1_index, machine_2_index, job_2_index)
        let mut swap_found = false;

        for m1 in 0..self.input.get_machine_count() {
            for m2 in m1..self.input.get_machine_count() { //for all machine pairs {m1,m2}
                let machine_1_jobs = machine_jobs.get_machine_jobs(m1);
                let machine_2_jobs = machine_jobs.get_machine_jobs(m2);
                for j1 in 0..machine_1_jobs.len() {
                    for j2 in 0..machine_2_jobs.len() { //for all job pairs (j1,j2) on (m1,m2)
                        let new_c_max = self.simulate_swap(m1, j1, m2, j2, machine_jobs);
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
            //swap TODO 1 ! ! swap logic in Data implementiren
            println!("new cmax {}", current_c_max);
        }
        Solution::unsatisfiable(Swap)//Todo erklärung dazu?
    }

    fn swap_tactic_n(&self) -> Solution {
        todo!()
    }

    ///computes the c_max that the current solution would have after a specified swap
    fn simulate_swap(&self, machine_1_index: usize, job_1_index: usize, machine_2_index: usize, job_2_index: usize, machine_jobs: &MachineJobs) -> u32 {
        let jobs = self.input.get_jobs();
        let machine_1_swap_workload = machine_jobs.get_machine_workload(machine_1_index) - jobs[job_1_index] + jobs[job_2_index];
        let machine_2_swap_workload = machine_jobs.get_machine_workload(machine_2_index) - jobs[job_2_index] + jobs[job_1_index];
        max(machine_1_swap_workload, machine_2_swap_workload)
    }
}