use std::fmt::Debug;
use std::sync::Arc;
use std::time::Instant;

use permutation::Permutation;

use crate::{Algorithm, Args};
use crate::global_bounds::bounds::Bounds;
use crate::output::data::Data;
use crate::output::machine_jobs::MachineJobs;
use crate::output::schedule::Schedule;

#[derive(Debug, Clone, Eq)]
pub struct Solution {
    satisfiable: bool,
    used_algorithms: Vec<Algorithm>,
    used_config: Option<String>,
    //if available
    data: Option<Data>,
}

impl Solution {
    /// creates a new solution, calculates the Schedule and updates the global upper bound
    pub fn new(used_algorithm: Algorithm, used_config: Option<String>, machine_jobs: MachineJobs, jobs: &[u32], global_bounds: Arc<Bounds>, args: Arc<Args>, perm: Arc<Permutation>, start_time: Instant, machine_count: usize) -> Self {
        let solution = Self { satisfiable: true, used_algorithms: vec![used_algorithm], used_config, data: Some(Data::new(machine_jobs.get_c_max(), machine_jobs)) };
        global_bounds.update_upper_bound(solution.get_data().get_c_max(), &solution, args, perm, start_time, Some(used_algorithm), jobs, machine_count);
        solution
    }

    pub fn unsatisfiable(used_algorithm: Algorithm) -> Self {
        Self {
            satisfiable: false,
            used_algorithms: vec![used_algorithm],
            used_config: None,
            data: None,
        }
    }

    pub fn to_output_string(&self, perm: Arc<Permutation>, jobs: &[u32], machine_count: usize) -> String {
        if self.satisfiable {
            let mut algorithms_str: String = String::new();
            for algorithm in self.used_algorithms.as_slice() {
                algorithms_str.push_str(format!("{:?}_", algorithm).as_str());
            }
            algorithms_str.pop();

            let mut schedule = Schedule::from_machine_jobs(self.get_data().get_machine_jobs(), jobs, machine_count);
            schedule.unsort(perm);
            format!("{2}\nSCHEDULING_SOLUTION {0} {1}0\nconfig:{3:?}\n\n", self.get_data().get_c_max(), schedule, algorithms_str, self.used_config)
        } else {
            format!("{}\nSCHEDULING_SOLUTION UNSATISFIABLE!\n{:?}\n\n", self.used_algorithms[0], self.used_config)
        }
    }

    pub fn is_satisfiable(&self) -> bool {
        self.satisfiable
    }

    /* pub fn get_used_algorithm(&self) -> &Algorithm {
        &self.used_algorithms
    }*/

    pub fn get_used_algorithms(&self) -> &[Algorithm] {
        self.used_algorithms.as_slice()
    }

    pub fn add_algorithm(&mut self, algorithm: Algorithm) {
        self.used_algorithms.push(algorithm);
    }

    pub fn add_config(&mut self, config: String) {
        match &self.used_config {
            None => { self.used_config = Some(config) }
            Some(s) => { self.used_config = Some(format!("{}\n{}", s, config)) }
        }
    }

    pub fn get_data(&self) -> &Data {
        if self.satisfiable {
            match &self.data {
                None => { panic!() }//impossible to reach this
                Some(data) => { &data }
            }
        } else {
            panic!("The solution is unsatisfiable, there is no data!");
        }
    }

    pub fn get_mut_data(&mut self) -> &mut Data {
        if self.satisfiable {
            match &mut self.data {
                None => { panic!() }//impossible to reach this
                Some(data) => { data }
            }
        } else {
            panic!("The solution is unsatisfiable, there is no data!");
        }
    }

    pub fn swap_jobs(&mut self, swap_indices: (usize, usize, usize, usize), jobs: &[u32], machine_count: usize, global_bounds: Arc<Bounds>, args: Arc<Args>, perm: Arc<Permutation>, start_time: Instant, currently_running_algo: Option<Algorithm>) {
        self.get_mut_data().swap_jobs(swap_indices, jobs, machine_count);
    }
}

impl PartialEq for Solution {
    fn eq(&self, other: &Self) -> bool {
        //two solutions are equal even if the used algorithms are different
        self.satisfiable == other.satisfiable && self.data == other.data
    }
}
