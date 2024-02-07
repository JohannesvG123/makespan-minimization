use std::{fmt, fs};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Deref, DerefMut, DivAssign};
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
use std::string::String;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use atoi::atoi;
use clap::{arg, FromArgMatches, Parser, Subcommand, ValueEnum};
use enum_map::{Enum, enum_map, EnumMap};
use rand::{Rng, SeedableRng};
use rayon::prelude::*;

use crate::Algorithm::{BF, FF, LPT, RF, RR, Swap};
use crate::global_bounds::bounds::Bounds;
use crate::good_solutions::good_solutions::GoodSolutions;
use crate::input::get_input;
use crate::input::input::Input;
use crate::output::log;
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
    //new algorithms can be added here:
    let algorithm_map: EnumMap<Algorithm, fn(Arc<Input>, Arc<Bounds>, Arc<Args>, usize) -> Box<dyn Scheduler + Send>, > = enum_map! {
        LPT => |input:Arc<Input>,global_bounds: Arc<Bounds>, args: Arc<Args>, config_id: usize| Box::new(LPTScheduler::new(input,global_bounds)) as Box<dyn Scheduler + Send>,
        BF=> |input:Arc<Input>,global_bounds: Arc<Bounds>, args: Arc<Args>, config_id: usize| Box::new(BFScheduler::new(input,global_bounds))as Box<dyn Scheduler + Send>,
        FF=> |input:Arc<Input>,global_bounds: Arc<Bounds>, args: Arc<Args>, config_id: usize| Box::new(FFScheduler::new(input,global_bounds))as Box<dyn Scheduler + Send>,
        RR=> |input:Arc<Input>,global_bounds: Arc<Bounds>, args: Arc<Args>, config_id: usize| Box::new(RRScheduler::new(input,global_bounds))as Box<dyn Scheduler + Send>,
        RF=> |input:Arc<Input>,global_bounds: Arc<Bounds>, args: Arc<Args>, config_id: usize| Box::new(RFScheduler::new(input,global_bounds,args.rf_configs[config_id].clone()))as Box<dyn Scheduler + Send>, //TODO prio clone wegbekommen mit slice oder soo
        Swap=> |input:Arc<Input>,global_bounds: Arc<Bounds>, args: Arc<Args>, config_id: usize| Box::new(Swapper::new(input,global_bounds,args.swap_configs[config_id].clone()))as Box<dyn Scheduler + Send>,
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

    let mut sorted_input = get_input(&args.path, args.measurement);
    let input = sorted_input.get_input();
    let perm = sorted_input.get_permutation();

    let thread_pool = rayon::ThreadPoolBuilder::new().num_threads(args.num_threads).build().unwrap();
    let tmp_opt = tmp_get_opt(&args.path);
    let global_bounds = Arc::new(Bounds::trivial(Arc::clone(&input), tmp_opt));
    let good_solutions = GoodSolutions::new(args.num_solutions);

    let (perm_for_output, args_for_output, good_solutions_for_output) = (Arc::clone(&perm), Arc::clone(&args), good_solutions.clone());

    //log(format!("START: {}", Local::now().format("%H:%M:%S%.f")));
    let start_time = std::time::Instant::now();
    let timeout_duration = Duration::from_secs(args.timeout_after);

    thread_pool.spawn(move || {
        let (perm_for_output, args_for_output, good_solutions_for_output) = (Arc::clone(&perm), Arc::clone(&args), good_solutions.clone());
        rayon::scope(move |s| {
            for algorithm in algos.iter() {
                let mut config_count: usize = 1;
                if algorithm == &RF {
                    config_count = args.rf_configs.len();
                } else if algorithm == &Swap {
                    config_count = args.swap_configs.len();
                }

                for current_config_id in 0..config_count {
                    //clone references to use them in spawned threads:

                    let (algorithm, good_solutions, input, args, global_bounds, perm) = (algorithm.clone(), good_solutions.clone(), Arc::clone(&input), Arc::clone(&args), Arc::clone(&global_bounds), Arc::clone(&perm));

                    s.spawn(move |_| {
                        let mut scheduler = algorithm_map[algorithm](input, global_bounds, Arc::clone(&args), current_config_id);
                        let solution = scheduler.schedule(good_solutions.clone(), args, perm, start_time);
                        good_solutions.add_solution(solution);
                    });
                }
            }
        });
        log(format!("END (all algorithms finished) after: {} sec (OPT not necessarily found)", start_time.elapsed().as_secs_f64()), true, args_for_output.measurement);
        good_solutions_for_output.write_output(perm_for_output, args_for_output.write, args_for_output.write_directory_name.clone(), args_for_output.path.file_stem().unwrap().to_str().unwrap(), args_for_output.write_separate_files, args_for_output.measurement);
        exit(0)
    });

    while start_time.elapsed() < timeout_duration {
        sleep(Duration::from_millis(100)); //hier kann die Genauigkeit angepasst werden
    }

    log(format!("END (timeout) after: {} sec (OPT not necessarily found)", start_time.elapsed().as_secs_f64()), true, args_for_output.measurement);
    good_solutions_for_output.write_output(perm_for_output, args_for_output.write, args_for_output.write_directory_name.clone(), args_for_output.path.file_stem().unwrap().to_str().unwrap(), args_for_output.write_separate_files, args_for_output.measurement);
}

fn tmp_get_opt(path_buf: &PathBuf) -> Option<u32> { //tmp
    let input_str = match fs::read_to_string(path_buf) {
        Ok(str) => str,
        Err(e) => panic!("{}", e),
    };
    match input_str.find("OPT:") {
        None => { None }
        Some(i) => {
            atoi::<u32>(&input_str.as_bytes()[i + 4..])
        }
    }
}

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

    /// configurations for running the RF algo
    ///
    /// (structure: "[rng_seed1],[fails_until_check1] ...", rng_seed-default=random seed, fails_until_check-default = machine_count)
    #[arg(long, value_name = "RF_CONFIG", num_args = 1.., requires = "rf", required_if_eq("rf", "true"))]
    rf_configs: Vec<RFConfig>,

    /// use RR (Round Robin) algo
    #[arg(long, action)]
    rr: bool,

    /// use Swap (local search approach) algo
    #[arg(long, action)]
    swap: bool,

    /// configurations for running the Swap algo
    ///
    /// (structure: "[swap_finding_tactic1],[swap_acceptance_rule1],[number_of_solutions1][,[rng_seed1]] ...", swap_finding_tactic-default=two-job-brute-force, swap_acceptance_rule-default = improvement, number_of_solutions-default=1, rng_seed-default=random seed or none depending on number of ;'s)
    #[arg(long, value_name = "SWAP_CONFIG", num_args = 1.., requires = "swap", required_if_eq("swap", "true"))]
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

    /// execution will be stopped after given amount of seconds
    #[arg(long, default_value = "10")]
    timeout_after: u64,

    /// Whether a measurement will be done or not (changes the amount of logs that are written)
    #[arg(long, action)]
    measurement: bool,

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
    Swap,
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "algorithm {:?}:", self)
    }
}