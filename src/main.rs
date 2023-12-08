use std::fmt;
use std::hash::Hash;
use std::path::PathBuf;
use std::rc::Rc;

use clap::{arg, Parser, ValueEnum};
use enum_map::Enum;
use rand::Rng;
use rayon::prelude::*;

use crate::input::get_input;
use crate::list_schedulers::LPTScheduler;
//use crate::list_schedulers::{best_fit, first_fit, longest_processing_time, random_fit, round_robin};
use crate::scheduler::Scheduler;

mod input;
mod output;
mod list_schedulers;
mod scheduler;

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
    /*let algorithm_map = enum_map! {
            Algorithm::LPT => |input| longest_processing_time(input,None),
            Algorithm::BF=> |input| best_fit(input,None),
            Algorithm::FF=> |input| first_fit(input,None),
            Algorithm::RR=> |input| round_robin(input,None),
            Algorithm::RF=> |input| random_fit(input,None),
    };*/

    //start:
    let args = Args::parse();

    let input = Rc::new(get_input(&args.path));

    let mut s: Box<dyn Scheduler> = Box::new(LPTScheduler::new(input, None));
    println!("{}", s.schedule());

    /*args.algos.par_iter().for_each(|algo| { //TODO parallelisierung krasser machen
        output(vec![(algorithm_map[algo.clone()](&input), algo)], args.write.clone(), args.write_name.clone(), args.path.file_stem().unwrap().to_str().unwrap()); //TODO clone entfernen (einf ref übergeben) und output methode umschreiben für single output wieder
    });*/
}