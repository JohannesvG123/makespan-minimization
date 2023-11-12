use std::hash::Hash;
use std::path::PathBuf;

use clap::{arg, Parser, ValueEnum};
use enum_map::{Enum, enum_map};

use crate::input::parse_input;
use crate::list_schedulers::lpt;
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

#[derive(Clone, ValueEnum, Debug, Eq, PartialEq, Hash, Enum)]
enum Algorithm {
    //TODO algos einfügen und iwi auf die jeweilige fn mappen
    /// LPT (Longest Processing Time)
    Lpt,
    AlgoXyz,
}

fn main() {
    ////////////testbereich
    let algorithm_map =
        enum_map! {
        Algorithm::Lpt => |input| lpt(input),
        Algorithm::AlgoXyz=> |input| lpt(input)
    };
    ////////////testbereich
    let args = Args::parse();
    //println!("{:?}", args); //---nur zum debuggen---
    let input = match parse_input(&args.path) {
        Ok(input) => input,
        Err(e) => {
            println!("ERROR: {}", e.to_string());
            return;
        }
    };
    //println!("{:?}", input); //---nur zum debuggen---
    let mut solutions: Vec<Solution> = vec![];
    args.algos.iter().for_each(|algo| {
        println!("{:?} stre algo xyz...", algo);
        solutions.push(algorithm_map[algo.clone()](&input))
    });
    println!("{:?}", solutions);
    let solution = lpt(&input);   //TODO algo auswahl durch cmd arg
    output(solution, args.write, args.write_name, args.path.file_stem().unwrap().to_str().unwrap());
}