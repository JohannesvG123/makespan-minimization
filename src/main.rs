use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{DerefMut, DivAssign};
use std::path::PathBuf;
use std::str::FromStr;
use std::string::String;
use std::sync::Arc;

use clap::{arg, FromArgMatches, Parser, Subcommand, ValueEnum};
use enum_map::{Enum, enum_map, EnumMap};
use rand::{Rng, SeedableRng};
use rayon::prelude::*;

use crate::Algorithm::{BF, FF, LPT, RF, RR, Swap};
use crate::global_bounds::bounds::Bounds;
use crate::good_solutions::good_solutions::GoodSolutions;
use crate::input::get_input;
use crate::input::input::Input;
use crate::schedulers::list_schedulers::bf_scheduler::BFScheduler;
use crate::schedulers::list_schedulers::ff_scheduler::FFScheduler;
use crate::schedulers::list_schedulers::lpt_scheduler::LPTScheduler;
use crate::schedulers::list_schedulers::rf_scheduler::{RFConfig, RFScheduler};
use crate::schedulers::list_schedulers::rr_scheduler::RRScheduler;
use crate::schedulers::local_search::swapper::{SwapConfig, Swapper};
use crate::schedulers::scheduler::Scheduler;

mod global_bounds;
mod good_solutions;
mod input;
mod output;
mod schedulers;

/// Framework to solve makespan-minimization problems

fn main() {
    //TODO PRIO thread nr
    //new algorithms can be added here:
    let algorithm_map: EnumMap<Algorithm, fn(Arc<Input>, Arc<Bounds>, Arc<Args>, usize) -> Box<dyn Scheduler + Send>, > = enum_map! {
        Algorithm::LPT => |input:Arc<Input>,global_bounds: Arc<Bounds>, args: Arc<Args>, config_id: usize| Box::new(LPTScheduler::new(input,global_bounds)) as Box<dyn Scheduler + Send>,
        Algorithm::BF=> |input:Arc<Input>,global_bounds: Arc<Bounds>, args: Arc<Args>, config_id: usize| Box::new(BFScheduler::new(input,global_bounds))as Box<dyn Scheduler + Send>,
        Algorithm::FF=> |input:Arc<Input>,global_bounds: Arc<Bounds>, args: Arc<Args>, config_id: usize| Box::new(FFScheduler::new(input,global_bounds))as Box<dyn Scheduler + Send>,
        Algorithm::RR=> |input:Arc<Input>,global_bounds: Arc<Bounds>, args: Arc<Args>, config_id: usize| Box::new(RRScheduler::new(input,global_bounds))as Box<dyn Scheduler + Send>,
        Algorithm::RF=> |input:Arc<Input>,global_bounds: Arc<Bounds>, args: Arc<Args>, config_id: usize| Box::new(RFScheduler::new(input,global_bounds,args.rf_configs[config_id].clone()))as Box<dyn Scheduler + Send>, //TODO prio clone wegbekommen mit slice oder soo
        Algorithm::Swap=> |input:Arc<Input>,global_bounds: Arc<Bounds>, args: Arc<Args>, config_id: usize| Box::new(Swapper::new(input,global_bounds,args.swap_configs[config_id].clone()))as Box<dyn Scheduler + Send>,
    };

    //start:
    let args = Arc::new(Args::parse());
    let mut algos = vec![]; //das muss man gerade so machen, da das cmd-arg Vec<Algos> keine subcommands zulässt...
    if args.bf { algos.push(BF); }
    if args.ff { algos.push(FF); }
    if args.lpt { algos.push(LPT); }
    if args.rf { algos.push(RF); }
    if args.rr { algos.push(RR); }
    if args.swap { algos.push(Swap); }

    let mut sorted_input = get_input(&args.path);
    let input = sorted_input.get_input();
    let perm = Arc::new(sorted_input.get_permutation());

    let thread_pool = rayon::ThreadPoolBuilder::new().num_threads(args.num_threads).build().unwrap();
    let global_bounds = Arc::new(Bounds::trivial(Arc::clone(&input)));
    let good_solutions = GoodSolutions::new(args.num_solutions);

    let perm_for_output = perm.clone(); //todo schöner machen
    let args_for_output = args.clone();
    let good_solutions_for_output = good_solutions.clone();

    thread_pool.scope(move |scope| {
        for algorithm in algos.iter() {
            let mut config_count: usize = 1;
            if algorithm == &RF {
                config_count = args.rf_configs.len();
            } else if algorithm == &Swap {
                config_count = args.swap_configs.len();
            }
            for current_config_id in 0..config_count {
                //clone references to use them in spawned threads:
                let (algorithm, good_solutions, input, perm, args, global_bounds) = (algorithm.clone(), good_solutions.clone(), Arc::clone(&input), Arc::clone(&perm), Arc::clone(&args), Arc::clone(&global_bounds), );

                //let tmp = Arc::new(args.rf_configs);
                scope.spawn(move |_| {
                    let mut scheduler = algorithm_map[algorithm](input, global_bounds, args, current_config_id);
                    let solution = scheduler.schedule(good_solutions.clone());
                    //TODO logging hier immer solution loggen
                    //output_solution(&solution, perm, args.write.clone(), args.directory_name.clone(), args.path.file_stem().unwrap().to_str().unwrap()); //this would print the calculated solution directly TODO mit param modifizierbar machen ob hier oder am ende
                    good_solutions.add_solution(solution);
                });
            }
        }
    });

    good_solutions_for_output.write_output(perm_for_output, args_for_output.write, args_for_output.write_directory_name.clone(), args_for_output.path.file_stem().unwrap().to_str().unwrap(), args_for_output.write_separate_files);
}

//TODO PRIO Thread nr ausgeben wenn möglich   println!("3T_ID:{:?}", current_thread_index());
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File path of the input data
    #[arg(long, required = true)]
    path: PathBuf,

    /*/// Algorithm(s) to use
    #[arg(short, long, num_args = 1.., required = true)]
    algos: Vec<Algorithm>,*/

    /// use BF (Best Fit) algo
    #[arg(long, action)]
    bf: bool,

    /// use FF (First Fit) algo
    #[arg(long, action)]
    ff: bool,

    /// use LPT (Longest Processing Time/Worst Fit) algo
    #[arg(long, action)]
    lpt: bool,

    /// use RF (Random Fit) algo
    #[arg(long, action)]
    rf: bool,
    //TODO PRIO zeitstamps + messungen + logging
    /// configurations for running the RF algo (structure: "[rng_seed1];[fails_until_check1] ...", rng_seed default=todo, fails_until_check default = 50)
    #[arg(long, value_name = "RF_CONFIG", num_args = 1.., requires = "rf", required_if_eq("rf", "true"))]
    rf_configs: Vec<RFConfig>,

    /// use RR (Round Robin) algo
    #[arg(long, action)]
    rr: bool,

    /// use Swap (local search approach) algo
    #[arg(long, action)]
    swap: bool,

    /// configurations for running the Swap algo (structure: "todo")
    #[arg(long, value_name = "SWAP_CONFIG", num_args = 1.., requires = "swap", help = "todo", required_if_eq("swap", "true"))]
    swap_configs: Vec<SwapConfig>,

    /// Whether the output should be written in a directory or not
    #[arg(long, action)]
    write: bool,

    /// Name of the output directory
    #[arg(long, requires = "write")]
    write_directory_name: Option<String>,

    /// Whether the output should be written in a single file or in separate ones
    #[arg(long, action, requires = "write")]
    write_separate_files: bool,

    /// How many threads to start
    #[arg(long, default_value = "8")]
    num_threads: usize,

    /// How many good solutions to store
    #[arg(long, default_value = "50")]
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

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "algorithm {:?}:", self)
    }
}