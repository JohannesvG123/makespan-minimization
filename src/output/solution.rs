use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;

use permutation::Permutation;

use crate::Algorithm;
use crate::global_bounds::bounds::Bounds;
use crate::output::data::Data;
use crate::output::machine_jobs::MachineJobs;

#[derive(Debug, Clone, Eq)]
pub struct Solution {
    satisfiable: bool,
    used_algorithms: Vec<Algorithm>,
    data: Option<Data>,
}

impl Solution {
    /// creates a new solution, calculates the Schedule and updates the global upper bound
    pub fn new(used_algorithm: Algorithm, machine_jobs: MachineJobs, jobs: &[u32], global_bounds: Arc<Bounds>) -> Self {//TODO (low prio) schedule nur on demand ausrechnen
        let solution = Self {
            satisfiable: true,
            used_algorithms: vec![used_algorithm],
            data: Some(Data::new(machine_jobs.get_c_max(), machine_jobs.calculate_schedule(jobs), machine_jobs)),
        };
        global_bounds.update_upper_bound(solution.get_data().get_c_max());
        solution
    }

    pub fn unsatisfiable(used_algorithm: Algorithm) -> Self {
        Self {
            satisfiable: false,
            used_algorithms: vec![used_algorithm],
            data: None,
        }
    }

    pub fn to_output_string(&self, perm: Arc<&Permutation>) -> String {
        if self.satisfiable {
            let mut algorithms_str: String = String::new();
            for algorithm in self.used_algorithms.as_slice() {
                algorithms_str.push_str(format!("{:?}_", algorithm).as_str());
            }
            algorithms_str.pop();
            format!("{2}\nSCHEDULING_SOLUTION {0} {1}0", self.get_data().get_c_max(), self.get_data().get_unsorted_schedule(perm), algorithms_str)
        } else {
            format!("{}\nSCHEDULING_SOLUTION UNSATISFIABLE!", self.used_algorithms[0])
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
}

impl PartialEq for Solution {
    fn eq(&self, other: &Self) -> bool {
        //two solutions are equal even if the used algorithms are different
        self.satisfiable == other.satisfiable && self.data == other.data
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //TODO (low prio) hier coolere ansicht zum debuggen oder soo (+ alle algos ausgeben net nur einen)
        if self.satisfiable {
            write!(f, "{2}\nSCHEDULING_SOLUTION {0} {1}0", self.get_data().get_c_max(), self.get_data().get_schedule(), self.used_algorithms[0])
        } else {
            write!(f, "{}\nSCHEDULING_SOLUTION UNSATISFIABLE!", self.used_algorithms[0])
        }
    }
}