use std::fmt;
use std::hash::Hash;
use std::ops::{DerefMut, DivAssign};
use std::path::PathBuf;
use std::sync::Arc;

use clap::{arg, Parser, Subcommand, ValueEnum};
use enum_map::{Enum, enum_map, EnumMap};
use rand::Rng;
use rayon::prelude::*;

use crate::global_bounds::bounds::Bounds;
use crate::good_solutions::good_solutions::GoodSolutions;
use crate::input::get_input;
use crate::input::input::Input;
use crate::output::output_solution;
use crate::schedulers::list_schedulers::bf_scheduler::BFScheduler;
use crate::schedulers::list_schedulers::ff_scheduler::FFScheduler;
use crate::schedulers::list_schedulers::lpt_scheduler::LPTScheduler;
use crate::schedulers::list_schedulers::rf_scheduler::RFScheduler;
use crate::schedulers::list_schedulers::rr_scheduler::RRScheduler;
use crate::schedulers::local_search::swapper::{SwapAcceptanceRule, Swapper};
use crate::schedulers::local_search::swapper::SwapTactic::TwoJobBruteForce;
use crate::schedulers::scheduler::Scheduler;

mod input;
mod output;
mod schedulers;
mod global_bounds;
mod good_solutions;

/// Framework to solve makespan-minimization problems

fn main() {
    //new algorithms can be added here:
    let algorithm_map: EnumMap<Algorithm, fn(Arc<Input>, Arc<Bounds>) -> Box<dyn Scheduler + Send>> = enum_map! {
        Algorithm::LPT => |input:Arc<Input>,global_bounds: Arc<Bounds>| Box::new(LPTScheduler::new(input,global_bounds)) as Box<dyn Scheduler + Send>,
        Algorithm::BF=> |input:Arc<Input>,global_bounds: Arc<Bounds>| Box::new(BFScheduler::new(input,global_bounds))as Box<dyn Scheduler + Send>,
        Algorithm::FF=> |input:Arc<Input>,global_bounds: Arc<Bounds>| Box::new(FFScheduler::new(input,global_bounds))as Box<dyn Scheduler + Send>,
        Algorithm::RR=> |input:Arc<Input>,global_bounds: Arc<Bounds>| Box::new(RRScheduler::new(input,global_bounds))as Box<dyn Scheduler + Send>,
        Algorithm::RF=> |input:Arc<Input>,global_bounds: Arc<Bounds>| Box::new(RFScheduler::new(input,global_bounds))as Box<dyn Scheduler + Send>,
        Algorithm::Swap=> |input:Arc<Input>,global_bounds: Arc<Bounds>| Box::new(Swapper::new(input,global_bounds, TwoJobBruteForce, SwapAcceptanceRule::DeclineByChance(0.1),3))as Box<dyn Scheduler + Send>,
    };

    //start:
    let args = Arc::new(Args::parse());

    let mut sorted_input = get_input(&args.path);
    let input = sorted_input.get_input();
    let perm = Arc::new(sorted_input.get_permutation());

    let thread_pool = rayon::ThreadPoolBuilder::new().num_threads(args.num_threads).build().unwrap();
    let global_bounds = Arc::new(Bounds::trivial(Arc::clone(&input)));
    let good_solutions = GoodSolutions::new(args.num_solutions);

    thread_pool.scope(move |scope| {
        for algorithm in args.algos.iter() {
            //clone references to use them in spawned threads:
            let (algorithm, good_solutions, input, perm, args, global_bounds) = (algorithm.clone(), good_solutions.clone(), Arc::clone(&input), Arc::clone(&perm), Arc::clone(&args), Arc::clone(&global_bounds));

            scope.spawn(move |_| {
                let mut scheduler = algorithm_map[algorithm](input, global_bounds);
                let solution = scheduler.schedule(good_solutions.clone());
                output_solution(&solution, perm, args.write.clone(), args.write_name.clone(), args.path.file_stem().unwrap().to_str().unwrap());
                good_solutions.add_solution(solution);
            });
        }
    });
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File path of the input data
    #[arg(short, long, required = true)]
    path: PathBuf,

    /// Whether the output should be written in a file or not
    #[arg(short = 'w', long, action)]
    write: bool,

    /// File name of the output file
    #[arg(short = 'n', long, requires = "write")]
    write_name: Option<String>,

    /// Algorithm(s) to use
    #[arg(short, long, num_args = 1.., required = true)]
    algos: Vec<Algorithm>,

    /// How many threads to start
    #[arg(short = 't', long, default_value = "8")]
    num_threads: usize,

    /// How many good solutions to store
    #[arg(short = 's', long, default_value = "50")]
    num_solutions: usize,
}

#[derive(Clone, ValueEnum, Debug, Eq, PartialEq, Hash, Enum, Copy)]
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
    /// Swap (local search approach)
    Swap, //TODO 1 https://github.com/clap-rs/clap/issues/2005 SwapTacticc, Range<usize>, acc_rule als sub-parameter oä einfügen / https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_0/index.html einlesen!
}

#[derive(Subcommand)]
pub enum SubEnum {
    Bli,
    Bla,
    Blub,
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "algorithm {:?}:", self)
    }
}