use std::fs;
use std::path::PathBuf;

use crate::input::sorted_input::SortedInput;
use crate::output::log;

pub mod input;
pub mod sorted_input;

pub fn get_input(path_buf: &PathBuf,measurement:bool) -> SortedInput {
    let input_str = read_input(path_buf,measurement);
    parse_input(&input_str,measurement)
}

fn read_input(path_buf: &PathBuf,measurement:bool) -> String {
    log(String::from("\nreading input..."),false,measurement);

    match fs::read_to_string(path_buf) {
        Ok(str) => str,
        Err(e) => panic!("{}", e),
    }
}

fn parse_input(input_str: &str,measurement:bool) -> SortedInput {
    log(String::from("parsing input..."),false,measurement);

    let mut split = match input_str.contains(";") {
        true => {
            //tmp_opt case:
            (input_str.split("OPT").collect::<Vec<_>>()[0]).split_whitespace()
        }
        false => { input_str.split_whitespace() }
    };//tmp wieder rauslöschen und drunter auskommentieren (nur für tmp opt benötigt)

    //let mut split = input_str.split_whitespace(); //TODO (low prio) auf Tokenized umstellen...

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

pub fn seed_from_str(part: &str) -> [u8; 32] {
    let seed_part = part.strip_prefix('[').unwrap().strip_suffix(']').unwrap();
    let seed_parts: Vec<&str> = seed_part.split("|").collect();

    let mut seed: [u8; 32] = [0; 32];
    for i in 0..seed_parts.len() {
        seed[i] = seed_parts[i].parse::<u8>().unwrap();
    }
    seed
}