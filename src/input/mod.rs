use std::fmt::{Display, Formatter};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use std::string::ParseError;

use rand::{Rng, SeedableRng, thread_rng};
use rand_chacha::ChaCha8Rng;

use crate::input::sorted_input::SortedInput;
use crate::output::log;

pub mod input;
pub mod sorted_input;

pub fn get_input(path_buf: &PathBuf, measurement: bool) -> SortedInput {
    let input_str = read_input(path_buf, measurement);
    parse_input(&input_str, measurement)
}

fn read_input(path_buf: &PathBuf, measurement: bool) -> String {
    log(String::from("reading input..."), false, measurement, None);

    match fs::read_to_string(path_buf) {
        Ok(str) => str,
        Err(e) => panic!("{}", e),
    }
}

fn parse_input(input_str: &str, measurement: bool) -> SortedInput {
    log(String::from("parsing input..."), false, measurement, None);

    let mut split = match input_str.contains(";") {
        true => {
            //tmp_opt case:
            input_str.split("OPT").collect::<Vec<_>>()[0].split_whitespace()
        }
        false => { input_str.split_whitespace() }
    };//todo tmp wieder rauslöschen und drunter auskommentieren (nur für tmp opt benötigt)

    //let mut split = input_str.split_whitespace(); //(low prio) auf Tokenized umstellen...

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

#[derive(Debug, Clone)]
pub struct MyRng(ChaCha8Rng);

impl Display for MyRng {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_seed())
    }
}

impl MyRng {
    pub fn get_seed(&self) -> RngSeed {
        RngSeed(self.0.get_seed())
    }

    pub fn generate_new_seed(&mut self) -> RngSeed {
        let mut seed: <ChaCha8Rng as SeedableRng>::Seed = Default::default();
        self.0.fill(&mut seed);
        RngSeed(seed)
    }

    pub fn get_mut(&mut self) -> &mut ChaCha8Rng {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
pub struct RngSeed(<ChaCha8Rng as SeedableRng>::Seed);

impl RngSeed {
    pub fn create_rng(&self) -> MyRng {
        MyRng(ChaCha8Rng::from_seed(self.0))
    }
}

impl FromStr for RngSeed {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let seed_part = s.strip_prefix('[').unwrap().strip_suffix(']').unwrap();

        let seed_parts: Vec<&str> = seed_part.split("/").collect();

        let mut seed: <ChaCha8Rng as SeedableRng>::Seed = Default::default();
        for i in 0..seed_parts.len() {
            seed[i] = seed_parts[i].parse::<u8>().unwrap();
        }
        Ok(Self(seed))
    }
}

impl Default for RngSeed {
    fn default() -> Self {
        let mut seed: <ChaCha8Rng as SeedableRng>::Seed = Default::default();
        thread_rng().fill(&mut seed);
        Self(seed)
    }
}

impl Display for RngSeed {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for val in self.0 {
            out = format!("{}{}/", out, val);
        }
        out = out.strip_suffix("/").unwrap().to_string();
        write!(f, "[{}]", out)
    }
}