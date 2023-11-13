use std::fmt;
use std::hash::Hash;
use std::path::PathBuf;

use clap::{arg, Parser, ValueEnum};
use enum_map::{Enum, enum_map};

use crate::input::parse_input;
use crate::list_schedulers::{best_fit, first_fit, longest_processing_time, random_fit, round_robin};
use crate::output::{output, Solution};

mod input;
mod output;
mod list_schedulers;

/// Program to solve makespan-minimization problems
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File path of the input data
    #[arg(short, long)]
    path: PathBuf,

    /// Whether the output should be written in a file or not
    #[arg(short = 'w', long, action)]
    write: bool,

    /// File name of the output file
    #[arg(short = 'n', long, requires = "write")]
    write_name: Option<String>,

    /// Algorithm(s) to use
    #[arg(short, long, num_args = 1..,)]
    algos: Vec<Algorithm>,

}

#[derive(Clone, ValueEnum, Debug, Eq, PartialEq, Hash, Enum, )]
pub enum Algorithm {
    /// LPT (Longest Processing Time/Worst Fit)
    LPT,
    /// BF (Best Fit)
    BF,
    /// FF (First Fit)
    FF,
    /// RR (Round Robin)
    RR,
    /// RF (Random Fit)
    RF,
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "algorithm {:?}:", self)
    }
}

fn main() {
    //new algorithms can be added here:
    let algorithm_map = enum_map! {
            Algorithm::LPT => |input| longest_processing_time(input),
            Algorithm::BF=> |input| best_fit(input),
            Algorithm::FF=> |input| first_fit(input),
            Algorithm::RR=> |input| round_robin(input),
            Algorithm::RF=> |input| random_fit(input),
    };

    //start:
    let args = Args::parse();

    let input = match parse_input(&args.path) {
        Ok(input) => input,
        Err(e) => {
            println!("ERROR: {}", e.to_string());
            return;
        }
    };

    let mut solutions: Vec<(Solution, &Algorithm)> = vec![];
    args.algos.iter().for_each(|algo| {
        solutions.push((algorithm_map[algo.clone()](&input), algo))
    });

    output(solutions, args.write, args.write_name, args.path.file_stem().unwrap().to_str().unwrap());
}