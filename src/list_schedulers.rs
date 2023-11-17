use rand::Rng;

use crate::Algorithm;
use crate::Algorithm::{BF, FF, LPT, RF, RR};
use crate::input::SortedInput;
use crate::output::{Schedule, Solution};

/// Schedulers using algorithms from the LS (List Scheduling family) to solve the makespan-minimization problem

/// Assigns the biggest job to the least loaded machine until all jobs are assigned (= worst fit)
pub fn longest_processing_time(input: &SortedInput) -> Solution {
    println!("running LPT algorithm...");
    let machine_count = *input.get_input().get_machine_count() as usize;
    let jobs = input.get_input().get_jobs();

    let mut schedule: Vec<(u32, u32)> = Vec::with_capacity(jobs.len());
    let mut machines_workload: Vec<u32> = vec![0; machine_count];
    let mut current_machine: usize = 0;
    let mut foreward: bool = true; // used to fill the machines in this order: (m=3) 0-1-2-2-1-0-0-1-2...
    let mut pause: bool = false;

    for &job in jobs.iter() {
        schedule.push((current_machine as u32, machines_workload[current_machine]));
        machines_workload[current_machine] += job;
        if foreward {
            if pause { pause = false; } else { current_machine += 1; }
            if current_machine == machine_count - 1 {
                foreward = false;
                pause = true;
            }
        } else {
            if pause { pause = false } else { current_machine -= 1; }
            if current_machine == 0 {
                foreward = true;
                pause = true
            }
        }
    }

    let c_max: u32 = *machines_workload.iter().max().unwrap();

    Solution::new(c_max, Schedule::new(input.unsort_schedule(schedule)), LPT)
}

/// Assigns the biggest job to the most loaded machine (that can fit the job) until all jobs are assigned
pub fn best_fit(input: &SortedInput) -> Solution {
    println!("running BF algorithm...");
    let machine_count = *input.get_input().get_machine_count() as usize;
    let jobs = input.get_input().get_jobs();

    //TODO upper bound als parameter(?) -> hier erstmal ein trivialer
    let upper_bound: u32 = jobs.iter().sum::<u32>() / machine_count as u32 + jobs.iter().max().unwrap();
    let mut schedule: Vec<(u32, u32)> = Vec::with_capacity(jobs.len());
    let mut machines_workload: Vec<u32> = vec![0; machine_count];


    for &job in jobs.iter() {
        let mut best_machine = machine_count;
        let mut fitting_machine_found = false;
        for m in 0..machine_count { //man könnte hier speedup erreichen wenn man ab Eingabegröße x eine BH-PQ nutzt...
            if !fitting_machine_found && machines_workload[m] + job <= upper_bound {
                best_machine = m;
                fitting_machine_found = true
            } else if fitting_machine_found && machines_workload[m] + job <= upper_bound && machines_workload[m] + job > machines_workload[best_machine] + job {
                best_machine = m;
            }
        }
        if best_machine == machine_count {
            println!("ERROR: upper bound is to low");//TODO Rückgabetyp anpassen, damit auch Error Rückgegeben werden kann oä
            return Solution::new(0, Schedule::new(vec![]), BF);
        }
        schedule.push((best_machine as u32, machines_workload[best_machine]));
        machines_workload[best_machine] += job;
    }

    let c_max: u32 = *machines_workload.iter().max().unwrap();

    Solution::new(c_max, Schedule::new(input.unsort_schedule(schedule)), BF)
}

/// Assigns the biggest job to the machine with the smallest index until all jobs are assigned
pub fn first_fit(input: &SortedInput, upper_bound: Option<u32>) -> Solution {
    println!("running FF algorithm...");
    let machine_count = *input.get_input().get_machine_count() as usize;
    let jobs = input.get_input().get_jobs();

    let upper_bound: u32 = match upper_bound {
        None => jobs.iter().sum::<u32>() / machine_count as u32 + jobs.iter().max().unwrap(), //trvial upper bound
        Some(val) => val
    };
    let mut schedule: Vec<(u32, u32)> = Vec::with_capacity(jobs.len());
    let mut machines_workload: Vec<u32> = vec![0; machine_count];
    let mut current_machine: usize = 0;

    for &job in jobs.iter() {
        if machines_workload[current_machine] + job > upper_bound {
            current_machine += 1;
            if current_machine == machine_count {
                println!("ERROR: upper bound is to low");
                return Solution::new(0, Schedule::new(vec![]), FF); //TODO Rückgabetyp anpassen, damit auch Error Rückgegeben werden kann oä
            }
        }
        schedule.push((current_machine as u32, machines_workload[current_machine]));
        machines_workload[current_machine] += job;
    }

    let c_max: u32 = *machines_workload.iter().max().unwrap();

    Solution::new(c_max, Schedule::new(input.unsort_schedule(schedule)), FF)
}

/// Round Robin job assignment
pub fn round_robin(input: &SortedInput) -> Solution {
    println!("running RR algorithm...");
    let machine_count = *input.get_input().get_machine_count() as usize;
    let jobs = input.get_input().get_jobs();

    let mut schedule: Vec<(u32, u32)> = Vec::with_capacity(jobs.len());
    let mut machines_workload: Vec<u32> = vec![0; machine_count];

    for j in 0..jobs.len() {
        let machine = j.rem_euclid(machine_count);
        schedule.push((machine as u32, machines_workload[machine]));
        machines_workload[machine] += jobs[j];
    }

    let c_max: u32 = *machines_workload.iter().max().unwrap();

    Solution::new(c_max, Schedule::new(input.unsort_schedule(schedule)), RR)
}

/// Assigns the jobs to random machines
pub fn random_fit(input: &SortedInput, upper_bound: Option<u32>) -> Solution {
    let (machine_count, jobs, upper_bound, mut schedule, mut machines_workload) = init(input, upper_bound, RF);
    let mut rng = rand::thread_rng();

    for &job in jobs.iter() {
        let random_index = rng.gen_range(0..machine_count);
        assign_job(&mut schedule, &mut machines_workload, job, random_index);
    }

    let c_max: u32 = *machines_workload.iter().max().unwrap();

    Solution::new(c_max, Schedule::new(input.unsort_schedule(schedule)), RF)
}

fn init(input: &SortedInput, upper_bound: Option<u32>, algorithm: Algorithm) -> (usize, &Vec<u32>, u32, Vec<(u32, u32)>, Vec<u32>) {
    println!("running {:?} algorithm...", algorithm);
    let machine_count = *input.get_input().get_machine_count() as usize;
    let jobs = input.get_input().get_jobs();
    let upper_bound: u32 = match upper_bound {
        None => jobs.iter().sum::<u32>() / machine_count as u32 + jobs.iter().max().unwrap(), //trvial upper bound
        Some(val) => val
    };

    (machine_count,
     jobs,
     upper_bound,
     Vec::with_capacity(jobs.len()), //schedule
     vec![0; machine_count]) //machines_workload
}

fn assign_job(schedule: &mut Vec<(u32, u32)>, mut machines_workload: &mut Vec<u32>, job: u32, index: usize) {
    schedule.push((index as u32, machines_workload[index]));
    machines_workload[index] += job;
}