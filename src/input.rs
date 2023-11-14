use std::fs;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use permutation::Permutation;

#[derive(Debug)]
pub struct Input {
    machine_count: u32,
    jobs: Vec<u32>,
}

impl Input {
    fn new(machine_count: u32, jobs: Vec<u32>) -> Self { //TODO FRAGE müssen hier checks eingebaut werden wie zb. m==jobs.length?
        Input { machine_count, jobs }
    }

    pub fn get_machine_count(&self) -> &u32 {
        &self.machine_count
    }

    pub fn get_jobs(&self) -> &Vec<u32> {
        &self.jobs
    }
}

#[derive(Debug)]
pub struct SortedInput {
    input: Input,
    permutation: Permutation,
}

impl SortedInput {
    pub fn new(machine_count: u32, jobs: Vec<u32>) -> Self {
        let mut input = Input::new(machine_count, jobs);
        let compare_desc = |a: &u32, b: &u32| b.cmp(a);
        let permutation = permutation::sort_by(&(input.jobs), compare_desc);
        input.jobs.sort_by(compare_desc);
        //println!("{:?}", permutation.apply_inv_slice(&(input.jobs))); //this gives us the original input
        SortedInput { input, permutation }
    }

    pub fn get_input(&self) -> &Input {
        &self.input
    }

    pub fn unsort_schedule<T: Clone>(&self, vec: Vec<T>) -> Vec<T> { //TODO FRAGE: in wiefern macht es einen unterschied ob ich hier mit oder ohne & arbeote?
        self.permutation.apply_inv_slice(vec)
    }
}

pub fn parse_input(path_buf: &PathBuf) -> Result<SortedInput, Error> {
    println!("reading input...");
    let data = match fs::read_to_string(path_buf) {
        Ok(str) => str,
        Err(e) => return Err(e),
    };

    //TODO FRAGE soll ich das so lassen oder noch auf Tokenized umstellen?
    //TODO daten tokenized einlesen aber des war mir jetzt zu blöd - desshalb jz erstmal so low^^
    println!("parsing input...");
    let mut split = data.split_whitespace();
    let p = split.next().unwrap().to_string();
    let p_cmax = split.next().unwrap().to_string();
    let job_count = split.next().unwrap().to_string().parse::<u32>().unwrap();
    let machine_count = split.next().unwrap().to_string().parse::<u32>().unwrap();
    let mut jobs: Vec<u32> = Vec::new();
    split.by_ref().for_each(|j| jobs.push(j.parse::<u32>().unwrap()));
    //checks:
    if p == "p" && p_cmax == "p_cmax" && *(jobs.last().unwrap()) == 0 && job_count + 1 == jobs.len() as u32 {
        jobs.pop();
        Ok(SortedInput::new(machine_count, jobs))
    } else {
        Err(Error::new(ErrorKind::InvalidInput, "invalid input!"))//todo aussagekräftiger machen (aber erst wenns tokenized is^^)
    }
}