use std::fs::File;
use std::io::Write;

use crate::Algorithm;
use crate::output::solution::Solution;

pub mod solution;
pub mod data;
pub mod machine_jobs;
pub mod schedule;

pub fn output(solutions: Vec<(Solution, &Algorithm)>, write: bool, write_name: Option<String>, input_file_name: &str) {
    if write { //TODO hier resorting + mehr möglichkeiten
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
        solutions.iter().for_each(|(solution, _)| println!("{}", solution));
    }
}