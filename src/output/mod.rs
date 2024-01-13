use std::fs::File;
use std::io::Write;
use std::sync::Arc;

use permutation::Permutation;

use crate::Algorithm;
use crate::output::solution::Solution;

pub mod solution;
pub mod data;
pub mod machine_jobs;
pub mod schedule;

pub fn output_solutions(solutions: Vec<(Solution, &Algorithm)>, write: bool, write_name: Option<String>, input_file_name: &str) {
    todo!()
}

pub fn output_solution(solution: &Solution, perm: Arc<Permutation>, write: bool, write_name: Option<String>, input_file_name: &str) {
    if write {
        let write_name = match &write_name {
            None => {
                let mut algorithms_str: String = String::new();
                for algorithm in solution.get_used_algorithms() {
                    algorithms_str.push_str(format!("{:?}_", algorithm).as_str());
                }
                format!("{0}_{1}solution", input_file_name, algorithms_str)
            }
            Some(str) => str.to_string()
        };
        let path = format!("data/{}.txt", write_name);
        println!("writing output in \"{}\" ...", path);
        let mut file = File::create(path).unwrap();
        file.write_all(solution.to_output_string(perm).as_bytes()).unwrap();
    } else {
        println!("{}", solution);
    }
}