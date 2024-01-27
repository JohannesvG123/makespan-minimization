use std::fs;
use std::path::PathBuf;

use crate::input::sorted_input::SortedInput;

pub mod input;
pub mod sorted_input;

pub fn get_input(path_buf: &PathBuf) -> SortedInput {
    let input_str = read_input(path_buf);
    parse_input(&input_str)
}

fn read_input(path_buf: &PathBuf) -> String {
    println!("reading input...");

    match fs::read_to_string(path_buf) {
        Ok(str) => str,
        Err(e) => panic!("{}", e),
    }
}

fn parse_input(input_str: &str) -> SortedInput {
    //TODO (low prio) auf Tokenized umstellen...
    println!("parsing input...");

    let mut split = input_str.split_whitespace();

    let p = split.next().unwrap().to_string();
    let p_cmax = split.next().unwrap().to_string();
    let job_count = split.next().unwrap().to_string().parse::<u32>().unwrap();
    let machine_count = split.next().unwrap().to_string().parse::<u32>().unwrap() as usize;
    let mut jobs: Vec<u32> = Vec::new();
    split.by_ref().for_each(|j| jobs.push(j.parse::<u32>().unwrap()));

    //checks:
    if p == "p" && p_cmax == "p_cmax" && *(jobs.last().unwrap()) == 0 && job_count + 1 == jobs.len() as u32 {
        jobs.pop();
        SortedInput::new(machine_count, jobs)
    } else {
        panic!("invalid input! => check the input file") //wenns tokenized ist: evtl aussagekräftiger machen und sagen was falsch war
    }
}