use std::fmt;

use crate::Algorithm;
use crate::output::data::Data;
use crate::output::machine_jobs::MachineJobs;
use crate::output::schedule::Schedule;

#[derive(Debug)]
pub struct Solution {
    satisfiable: bool,
    used_algorithm: Algorithm,
    data: Option<Data>,
}

impl Solution {
    pub fn new(used_algorithm: Algorithm, c_max: u32, schedule: Vec<(u32, u32)>, machine_jobs: Vec<(u32, Vec<u32>)>) -> Self {
        Self {
            satisfiable: true,
            used_algorithm,
            data: Some(Data::new(c_max, Schedule::new(schedule), MachineJobs::new(machine_jobs))),
        }
    }

    pub fn unsatisfiable(used_algorithm: Algorithm) -> Self {
        Self {
            satisfiable: false,
            used_algorithm,
            data: None,
        }
    }

    pub fn is_satisfiable(&self) -> bool {
        self.satisfiable
    }

    pub fn get_used_algorithm(&self) -> &Algorithm {
        &self.used_algorithm
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

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.satisfiable {
            write!(f, "{2}\nSCHEDULING_SOLUTION {0} {1}0", self.get_data().get_c_max(), self.get_data().get_schedule(), self.used_algorithm)
        } else {
            write!(f, "{}\nSCHEDULING_SOLUTION UNSATISFIABLE!", self.used_algorithm)
        }
    }
}