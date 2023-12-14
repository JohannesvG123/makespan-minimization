use std::fmt;
use std::hash::Hash;
use std::ops::DerefMut;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

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
    let algorithm_map: EnumMap<Algorithm, fn(Arc<Input>) -> Box<dyn Scheduler + Send>> = enum_map! {
        Algorithm::LPT => |input:Arc<Input>| Box::new(LPTScheduler::new(input,None,None)) as Box<dyn Scheduler + Send>,
        Algorithm::BF=> |input:Arc<Input>| Box::new(BFScheduler::new(input,None,None))as Box<dyn Scheduler + Send>,
        Algorithm::FF=> |input:Arc<Input>| Box::new(FFScheduler::new(input,None,None))as Box<dyn Scheduler + Send>,
        Algorithm::RR=> |input:Arc<Input>| Box::new(RRScheduler::new(input,None,None))as Box<dyn Scheduler + Send>,
        Algorithm::RF=> |input:Arc<Input>| Box::new(RFScheduler::new(input,None,None))as Box<dyn Scheduler + Send>,
    };

    //start:
    let args = Arc::new(Args::parse());

    let mut sorted_input = get_input(&args.path);
    let input = sorted_input.get_input();

    let mut perm = Arc::new(sorted_input.get_permutation().clone()); //todo ihhh clone value -> Aber das sorting muss eh noch angepasst werden und dann ergibt sich das

    let thread_pool = rayon::ThreadPoolBuilder::new().build().unwrap();

    for algorithm in args.algos.iter() {
        let input_tmp = input.clone();
        let perm_tmp = perm.clone();
        let algo = algorithm.clone();
        let args_tmp = args.clone();

        thread_pool.spawn(move || {
            let mut scheduler = algorithm_map[algo](input_tmp);
            let mut solution = scheduler.schedule();

            solution.get_mut_data().unsort(perm_tmp);
            output(vec![(solution, &scheduler.get_algorithm())], args_tmp.write.clone(), args_tmp.write_name.clone(), args_tmp.path.file_stem().unwrap().to_str().unwrap());
        });
    }

    sleep(Duration::from_secs(3)); //Todo wie warte ich drauf dass die threads fertig werden?
}