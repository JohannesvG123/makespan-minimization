use std::hash::Hash;
use std::path::PathBuf;

use clap::{arg, Parser, ValueEnum};

use crate::input::parse_input;
use crate::output::{output, Schedule, Solution};

mod input;
mod output;

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

#[derive(Clone, ValueEnum, Debug, Eq, PartialEq, Hash)]
enum Algorithm {
    //TODO algos einfügen und iwi auf die jeweilige fn mappen
    Algo1,
    Algo2,
    Algo3,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args); //---nur zum debuggen---
    let input = match parse_input(args.path) {
        Ok(input) => input,
        Err(e) => {
            println!("ERROR: {}", e.to_string());
            return;
        }
    };
    println!("{:?}", input); //---nur zum debuggen---
    //algo starten und logging nicht vergessen bidde dange...
    let s = Solution::new(51, Schedule::new(vec![(3, 0), (2, 44), (1, 0)]));

    output(s, args.write, args.write_name);
}