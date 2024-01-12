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
use crate::output::output;
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

/// Program to solve makespan-minimization problems
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
    /// Swap (local search approach)
    Swap, //TODO https://github.com/clap-rs/clap/issues/2005 SwapTacticc, Range<usize>, acc_rule als sub-parameter oä einfügen
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "algorithm {:?}:", self)
    }
}

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
    let mut perm = Arc::new(sorted_input.get_permutation().clone()); //todo ihhh clone value -> Aber das sorting muss eh noch angepasst werden und dann ergibt sich das

    let thread_pool = rayon::ThreadPoolBuilder::new().num_threads(args.num_threads).build().unwrap();
    let global_bounds = Arc::new(Bounds::trivial(Arc::clone(&input)));
    let good_solutions = GoodSolutions::new(args.num_solutions);
    let x = good_solutions.clone(); //tmp

    thread_pool.scope(move |scope| {
        for algorithm in args.algos.iter() {
            println!("{}", algorithm);
            let input = Arc::clone(&input); //TODO alles inline am ende und überprüfen ob immer Arc usw nötig ist
            let perm = Arc::clone(&perm);
            let algorithm = algorithm.clone(); //todo noch nötig?
            let args = Arc::clone(&args);
            let global_bounds = Arc::clone(&global_bounds);
            let good_solutions = x.clone();//good_solutions.clone(); tmp

            scope.spawn(move |_| {
                let mut scheduler = algorithm_map[algorithm](input, global_bounds);
                let mut solution = scheduler.schedule(good_solutions.clone());
                //todo ausgabe schöner machen
                let mut s = solution.clone();
                if s.is_satisfiable() {
                    s.get_mut_data().unsort(perm);
                }
                output(vec![(s, &scheduler.get_algorithm())], args.write.clone(), args.write_name.clone(), args.path.file_stem().unwrap().to_str().unwrap());
                //
                good_solutions.add_solution(solution);
            });
        }
    });
    println!("{:?}", good_solutions.get_solution_count()); //tmp
    println!("{:?}", good_solutions); //tmp
}

