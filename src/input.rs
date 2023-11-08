use std::fs;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use permutation::Permutation;

#[derive(Debug)]
struct Input {
    machine_count: i32,
    jobs: Vec<i32>,
}

impl Input {
    fn new(machine_count: i32, jobs: Vec<i32>) -> Self { //TODO müssen hier checks eingebaut werden wie zb. m==jobs.length?
        Input { machine_count, jobs }
    }
}

#[derive(Debug)]
pub struct SortedInput {
    //TODO coolere hierarchie oder so überlegen (?)
    input: Input,
    permutation: Permutation,
}

impl SortedInput {
    //TODO eig kann diese kapselung mit sorted input und input weg oder? + getter/setter usw hinzufügen
    pub fn new(machine_count: i32, jobs: Vec<i32>) -> Self {
        let mut input = Input::new(machine_count, jobs);
        let permutation = permutation::sort(&(input.jobs));
        input.jobs.sort();
        //println!("{:?}",permutation.apply_slice(&(input.jobs))); //this gives us the original input
        SortedInput { input, permutation }
    }
}

pub fn parse_input(path_buf: PathBuf) -> Result<SortedInput, Error> { //TODO schöner aufteilen in read+parse
    println!("reading input...");
    let data = match fs::read_to_string(path_buf) { //TODO Wann soll ich errors/exceptions schmeißen?
        Ok(str) => str,
        Err(e) => return Err(e),
    };
    //TODO daten tokenized einlesen aber des war mir jetzt zu blöd - desshalb jz erstmal so low^^

    println!("parsing input...");
    let mut split = data.split_whitespace();
    let p = split.next().unwrap().to_string();
    let p_cmax = split.next().unwrap().to_string();
    let job_count = split.next().unwrap().to_string().parse::<i32>().unwrap();
    let machine_count = split.next().unwrap().to_string().parse::<i32>().unwrap();
    let mut jobs: Vec<i32> = Vec::new();
    split.by_ref().for_each(|j| jobs.push(j.parse::<i32>().unwrap()));
    //checks:
    if p == "p" && p_cmax == "p_cmax" && *(jobs.last().unwrap()) == 0 && job_count + 1 == jobs.len() as i32 {
        jobs.pop();
        Ok(SortedInput::new(machine_count, jobs))
    } else {
        Err(Error::from(ErrorKind::Other))//TODO schöner machen
    }
}