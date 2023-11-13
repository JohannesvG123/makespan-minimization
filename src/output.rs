use std::fmt;
use std::fs::File;
use std::io::Write;

use crate::Algorithm;

#[derive(Debug)]
pub struct Solution {
    c_max: u32,
    schedule: Schedule,
    algorithm: Algorithm,
}

impl Solution {
    pub fn new(c_max: u32, schedule: Schedule, algorithm: Algorithm) -> Self {
        Solution { c_max, schedule, algorithm }
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{2}\nSCHEDULING_SOLUTION {0} {1}0", self.c_max, self.schedule, self.algorithm)
    }
}


#[derive(Debug)]
///(machine_number_job1,start_time_job1),...
pub struct Schedule(Vec<(u32, u32)>);

impl Schedule {
    pub fn new(schedule: Vec<(u32, u32)>) -> Self {
        Schedule(schedule)
    }
}

impl fmt::Display for Schedule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, solution_i| {
            result.and_then(|_| write!(f, "{} {} ", (*solution_i).0, (*solution_i).1))
        })
    }
}

pub fn output(solutions: Vec<(Solution, &Algorithm)>, write: bool, write_name: Option<String>, input_file_name: &str) {
    if write {
        solutions.iter().for_each(|(solution, algorithm)| {
            let write_name = match &write_name {
                None => format!("{0}_{1:?}_solution", input_file_name, *algorithm),
                Some(str) => str.to_string()
            };
            let path = format!("data/{}.txt", write_name);
            println!("writing output in \"{}\" ...", path);
            let mut file = File::create(path).unwrap();
            file.write_all(solution.to_string().as_bytes()).unwrap();
        });
    } else {
        solutions.iter().for_each(|(solution, algorithm)| println!("{}", solution));
    }
}