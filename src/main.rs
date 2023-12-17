use std::fmt;
use std::hash::Hash;
use std::ops::{DerefMut, DivAssign};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use clap::{arg, Parser, ValueEnum};
use enum_map::{Enum, enum_map, EnumMap};
use rand::Rng;
use rayon::prelude::*;

use crate::global_bounds::bounds::Bounds;
use crate::global_bounds::create_global_bounds;
use crate::good_solutions::create_good_solutions;
use crate::good_solutions::good_solutions::GoodSolutions;
use crate::input::get_input;
use crate::input::input::Input;
use crate::output::output;
use crate::schedulers::list_schedulers::bf_scheduler::BFScheduler;
use crate::schedulers::list_schedulers::ff_scheduler::FFScheduler;
use crate::schedulers::list_schedulers::lpt_scheduler::LPTScheduler;
use crate::schedulers::list_schedulers::rf_scheduler::RFScheduler;
use crate::schedulers::list_schedulers::rr_scheduler::RRScheduler;
use crate::schedulers::local_search::swapper::Swapper;
use crate::schedulers::scheduler::Scheduler;

mod input;
mod output;
mod schedulers;
mod global_bounds;
mod good_solutions;

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
    /// Swap (local search approach)
    Swap,
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "algorithm {:?}:", self)
    }
}

fn main() {
    //new algorithms can be added here:
    let algorithm_map: EnumMap<Algorithm, fn(Arc<Input>, Arc<Mutex<Bounds>>, Arc<Mutex<GoodSolutions>>) -> Box<dyn Scheduler + Send>> = enum_map! {
        Algorithm::LPT => |input:Arc<Input>,global_bounds: Arc<Mutex<Bounds>>,_| Box::new(LPTScheduler::new(input,global_bounds)) as Box<dyn Scheduler + Send>,
        Algorithm::BF=> |input:Arc<Input>,global_bounds: Arc<Mutex<Bounds>>,_| Box::new(BFScheduler::new(input,global_bounds))as Box<dyn Scheduler + Send>,
        Algorithm::FF=> |input:Arc<Input>,global_bounds: Arc<Mutex<Bounds>>,_| Box::new(FFScheduler::new(input,global_bounds))as Box<dyn Scheduler + Send>,
        Algorithm::RR=> |input:Arc<Input>,global_bounds: Arc<Mutex<Bounds>>,_| Box::new(RRScheduler::new(input,global_bounds))as Box<dyn Scheduler + Send>,
        Algorithm::RF=> |input:Arc<Input>,global_bounds: Arc<Mutex<Bounds>>,_| Box::new(RFScheduler::new(input,global_bounds))as Box<dyn Scheduler + Send>,
        Algorithm::Swap=> |input:Arc<Input>,global_bounds: Arc<Mutex<Bounds>>,good_solutions: Arc<Mutex<GoodSolutions>>| Box::new(Swapper::new(input,global_bounds,good_solutions))as Box<dyn Scheduler + Send>,
    };

    //start:
    let args = Arc::new(Args::parse());

    let mut sorted_input = get_input(&args.path);
    let input = sorted_input.get_input();
    let mut perm = Arc::new(sorted_input.get_permutation().clone()); //todo ihhh clone value -> Aber das sorting muss eh noch angepasst werden und dann ergibt sich das

    let thread_pool = rayon::ThreadPoolBuilder::new().build().unwrap();
    let global_bounds = create_global_bounds(Arc::clone(&input));
    let good_solutions = create_good_solutions(20);

    for algorithm in args.algos.iter() {
        let input = Arc::clone(&input); //TODO alles inline am ende
        let perm = Arc::clone(&perm);
        let algo = algorithm.clone();
        let args = Arc::clone(&args);
        let global_bounds = Arc::clone(&global_bounds);
        let good_solutions = Arc::clone(&good_solutions);

        thread_pool.spawn(move || {
            let mut scheduler = algorithm_map[algo](input, global_bounds, Arc::clone(&good_solutions));
            let mut solution = scheduler.schedule();
            //ausgabe
            let mut s = solution.clone();
            s.get_mut_data().unsort(perm);
            output(vec![(s, &scheduler.get_algorithm())], args.write.clone(), args.write_name.clone(), args.path.file_stem().unwrap().to_str().unwrap());
            //
            good_solutions.lock().unwrap().add_solution(solution); //Todo diesen call in extra methode schieben damit mutex unlockt? evtl (ohne let definition unlockt der direkt wieder oder)
        });
    }

    sleep(Duration::from_secs(3)); //Todo wie warte ich drauf dass die threads fertig werden? -> handles halten und joinen oder mit synchronisationsmechanismus
}

