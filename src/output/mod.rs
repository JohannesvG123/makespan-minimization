use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::Arc;

use chrono::Local;
use permutation::Permutation;
use rayon::current_thread_index;

use crate::output::solution::Solution;

pub mod data;
pub mod machine_jobs;
pub mod schedule;
pub mod solution;

pub fn output_solution(solution: &Solution, perm: Arc<Permutation>, write: bool, directory_name: String, input_file_name: &str, write_separate_files: bool) {
    if write {
        let output_string = solution.to_output_string(perm);

        if write_separate_files {
            let mut algorithms_str: String = String::new();
            for algorithm in solution.get_used_algorithms() {
                algorithms_str.push_str(format!("{:?}_", algorithm).as_str());
            }
            let original_filename = format!("{0}solution", algorithms_str);
            let mut filename = original_filename.clone();
            let dir = format!("data/{}", directory_name);
            if !Path::new(&dir).exists() {
                fs::create_dir(&dir).unwrap();
            }
            let mut path = format!("data/{0}/{1}.txt", directory_name, original_filename);
            let mut i: usize = 0;
            while Path::new(&path).exists() {
                filename = original_filename.clone();
                i += 1;
                filename.push_str(&i.to_string());
                path = format!("data/{0}/{1}.txt", directory_name, filename);
            }
            let mut file = File::create(path).unwrap();
            file.write_all(output_string.as_bytes()).unwrap();
        } else {
            let dir = format!("data/{}", directory_name);
            let path = format!("{}{}", dir, "/solutions.txt");
            if Path::new(&path).exists() {
                let mut file = OpenOptions::new().write(true).append(true).open(format!("{}{}", dir, "/solutions.txt")).unwrap();
                if let Err(e) = write!(file, "{}", output_string) {
                    eprintln!("Couldn't write to file todo: {}", e);
                }
            } else {
                //create dir+file:
                fs::create_dir(&dir).unwrap();
                let mut file = File::create(path).unwrap();
                file.write_all(output_string.as_bytes()).unwrap();
            }
        }
    } else {
        log(solution.to_string());
    }
}

pub fn get_directory_name(directory_name: Option<String>, input_file_name: &str) -> String {
    match directory_name {
        None => {
            let date = Local::now();
            format!("{0}_solution_{1}", input_file_name, date.format("%Y-%m-%d_%H-%M-%S"))
        }
        Some(str) => str.to_string()
    }
}

///used for logging (also prints the current thread index if available)
pub fn log(message: String) {
    if true { //TODO 1 cmd arg hinzufügen damit in dem fall nix geprinted wird und nur die beste solution ausgegeben wird?-> Fragen
        let thread_opt = current_thread_index();
        let thread = match thread_opt {
            None => { String::new() }
            Some(t) => { format!("thread_{}: ", t) }
        };

        println!("{}{}", thread, message)
    }
}