use std::fmt;
use std::hash::Hash;
use std::path::PathBuf;
use std::rc::Rc;

use clap::{arg, Parser, ValueEnum};
use enum_map::{Enum, enum_map, EnumMap};
use rand::Rng;
use rayon::prelude::*;

use crate::input::get_input;
use crate::input::input::Input;
use crate::output::output;
use crate::schedulers::list_schedulers::bf_scheduler::BFScheduler;
use crate::schedulers::list_schedulers::ff_scheduler::FFScheduler;
use crate::schedulers::list_schedulers::lpt_scheduler::LPTScheduler;
use crate::schedulers::list_schedulers::rf_scheduler::RFScheduler;
use crate::schedulers::list_schedulers::rr_scheduler::RRScheduler;
use crate::schedulers::scheduler::Scheduler;

mod input;
mod output;
mod schedulers;

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
    let algorithm_map: EnumMap<Algorithm, fn(Rc<Input>) -> Box<dyn Scheduler>> = enum_map! {
        Algorithm::LPT => |input:Rc<Input>| Box::new(LPTScheduler::new(input,None,None)) as Box<dyn Scheduler>,
        Algorithm::BF=> |input:Rc<Input>| Box::new(BFScheduler::new(input,None,None))as Box<dyn Scheduler>,
        Algorithm::FF=> |input:Rc<Input>| Box::new(FFScheduler::new(input,None,None))as Box<dyn Scheduler>,
        Algorithm::RR=> |input:Rc<Input>| Box::new(RRScheduler::new(input,None,None))as Box<dyn Scheduler>,
        Algorithm::RF=> |input:Rc<Input>| Box::new(RFScheduler::new(input,None,None))as Box<dyn Scheduler>,
    };

    //start:
    let args = Args::parse();

    let mut sorted_input = get_input(&args.path);
    let input = sorted_input.get_input();

    let mut schedulers: Vec<Box<dyn Scheduler>> = vec![];
    for algorithm in args.algos.iter() {
        schedulers.push(algorithm_map[algorithm.clone()](input.clone()));
    }


    for mut scheduler in schedulers {
        let mut solution = scheduler.schedule();
        solution.get_mut_data().unsort(sorted_input.get_mut_permutation());
        output(vec![(solution, &scheduler.get_algorithm())], args.write.clone(), args.write_name.clone(), args.path.file_stem().unwrap().to_str().unwrap());
    }
}