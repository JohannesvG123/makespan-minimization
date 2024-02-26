use std::cmp::max;
use std::process::exit;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Instant;

use chrono::Local;
use permutation::Permutation;

use crate::{Algorithm, Args};
use crate::input::input::Input;
use crate::output::{get_directory_name, log, output_solution};
use crate::output::solution::Solution;

pub struct Bounds {
    upper_bound: AtomicU32,
    lower_bound: AtomicU32,
    tmp_opt: Option<u32>,
}

impl Bounds {
    pub fn new(upper_bound: u32, lower_bound: u32, tmp_opt: Option<u32>) -> Self {
        Self {
            upper_bound: AtomicU32::new(upper_bound),
            lower_bound: AtomicU32::new(lower_bound),
            tmp_opt,
        }
    }

    pub fn trivial(input: Arc<Input>, tmp_opt: Option<u32>) -> Self {
        let upper_bound = input.get_jobs().iter().sum::<u32>() / input.get_machine_count() as u32 + input.get_jobs().iter().max().unwrap();
        let lower_bound = max(*(input.get_jobs().iter().max().unwrap()), input.get_jobs().iter().sum::<u32>().div_ceil(input.get_machine_count() as u32));
        log(format!("using the trivial bounds: UB:{} LB:{} ", upper_bound, lower_bound), true, true, None);
        Self::new(upper_bound, lower_bound, tmp_opt)
    }

    /// returns (upper_bound, lower_bound)
    pub fn get_bounds(&self) -> (u32, u32) {
        (self.get_upper_bound(), self.get_lower_bound())
    }

    pub fn get_upper_bound(&self) -> u32 {
        self.upper_bound.load(Ordering::Acquire)
    }

    pub fn get_lower_bound(&self) -> u32 {
        self.lower_bound.load(Ordering::Acquire)
    }

    /* pub fn set_upper_bound(&self, upper_bound: u32) {
         self.upper_bound.store(upper_bound, Ordering::Release)
     }

     pub fn set_lower_bound(&self, lower_bound: u32) {
         self.lower_bound.store(lower_bound, Ordering::Release)
     }*/

    pub fn update_bounds(&self, new_upper_bound: u32, new_lower_bound: u32, solution: &Solution, args: Arc<Args>, perm: Arc<Permutation>, start_time: Instant, currently_running_algo: Option<Algorithm>) {
        self.update_upper_bound(new_upper_bound, solution, Arc::clone(&args), Arc::clone(&perm), start_time, currently_running_algo);
        self.update_lower_bound(new_lower_bound, solution, args, perm, start_time, currently_running_algo);
    }

    pub fn update_upper_bound(&self, new_upper_bound: u32, solution: &Solution, args: Arc<Args>, perm: Arc<Permutation>, start_time: Instant, currently_running_algo: Option<Algorithm>) {
        let date = Local::now();
        let prev = self.upper_bound.fetch_min(new_upper_bound, Ordering::AcqRel);
        if new_upper_bound < prev {
            log(format!("NEW upper_bound:{}->{} (after: {:?} sec)", prev, new_upper_bound, start_time.elapsed().as_secs_f64()), true, args.measurement, currently_running_algo);
            match self.tmp_opt { //tmp löschen
                None => {}
                Some(opt_c_max) => {
                    if new_upper_bound == opt_c_max {
                        log(format!("END after: {:?} sec (found OPT solution)", start_time.elapsed().as_secs_f64()), true, args.measurement, currently_running_algo);
                        let input_file_name = args.path.file_stem().unwrap().to_str().unwrap();
                        output_solution(solution, perm, args.write, get_directory_name(args.write_directory_name.clone(), input_file_name), input_file_name, true, args.measurement);
                        exit(0)
                    }
                }
            }
            if new_upper_bound == self.get_lower_bound() {
                log(format!("END after: {:?} sec (found OPT solution)", start_time.elapsed().as_secs_f64()), true, args.measurement, currently_running_algo);
                let input_file_name = args.path.file_stem().unwrap().to_str().unwrap();
                output_solution(solution, perm, args.write, get_directory_name(args.write_directory_name.clone(), input_file_name), input_file_name, false, args.measurement);
                exit(0)
            }
        }
    }

    pub fn update_lower_bound(&self, new_lower_bound: u32, solution: &Solution, args: Arc<Args>, perm: Arc<Permutation>, start_time: Instant, currently_running_algo: Option<Algorithm>) {
        let date = Local::now();
        let prev = self.upper_bound.fetch_max(new_lower_bound, Ordering::AcqRel);
        if new_lower_bound > prev {
            log(format!("NEW lower_bound:{}->{} (after: {:?} sec)", prev, new_lower_bound, start_time.elapsed().as_secs_f64()), true, args.measurement, currently_running_algo);
            if self.get_upper_bound() == new_lower_bound {
                log(format!("END after: {:?} sec (found OPT solution)", start_time.elapsed().as_secs_f64()), true, args.measurement, currently_running_algo);
                let input_file_name = args.path.file_stem().unwrap().to_str().unwrap();
                output_solution(solution, perm, args.write, get_directory_name(args.write_directory_name.clone(), input_file_name), input_file_name, false, args.measurement);
                exit(0)
            }
        }
    }
}