use rand::Rng;

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

    for i in 0..jobs.len() {
        schedule.push((current_machine as u32, machines_workload[current_machine]));
        machines_workload[current_machine] += jobs[i];
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

/// Assigns the biggest job to the most loaded machine (that can fit the job) until all jobs are assigned TODO
pub fn best_fit(input: &SortedInput) -> Solution {
    println!("running BF algorithm...");
    Solution::new(0, Schedule::new(vec![]), BF)
}

/// Assigns the biggest job to the machine with the smallest index until all jobs are assigned
pub fn first_fit(input: &SortedInput) -> Solution {
    println!("running FF algorithm...");
    let machine_count = *input.get_input().get_machine_count() as usize;
    let jobs = input.get_input().get_jobs();

    //TODO upper bound als parameter(?) -> hier erstmal ein trivialer
    let upper_bound: u32 = 80;//TODO formel raussuchen
    let mut schedule: Vec<(u32, u32)> = Vec::with_capacity(jobs.len());
    let mut machines_workload: Vec<u32> = vec![0; machine_count];
    let mut current_machine: usize = 0;

    for i in 0..jobs.len() {
        if machines_workload[current_machine] + jobs[i] > upper_bound {
            current_machine += 1;
            if current_machine == machine_count {
                println!("ERROR: upper bound is to low");
                return Solution::new(0, Schedule::new(vec![]), FF); //TODO das ist noch unschön
            }
        }
        schedule.push((current_machine as u32, machines_workload[current_machine]));
        machines_workload[current_machine] += jobs[i];
    }

    let c_max: u32 = *machines_workload.iter().max().unwrap();

    Solution::new(c_max, Schedule::new(input.unsort_schedule(schedule)), FF)
}

/// Round Robin job assignment
pub fn round_robin(input: &SortedInput) -> Solution { // TODO 1 testen mit verschiedenen inputs
    println!("running RR algorithm...");
    let machine_count = *input.get_input().get_machine_count() as usize;
    let jobs = input.get_input().get_jobs();

    let mut schedule: Vec<(u32, u32)> = Vec::with_capacity(jobs.len());
    let mut machines_workload: Vec<u32> = vec![0; machine_count];

    for i in 0..jobs.len() { //TODO man kann sich machines_workload sparen aber dann wirds unverständlicher... trotzdem machen?
        let machine = i.rem_euclid(machine_count);
        schedule.push((machine as u32, machines_workload[machine]));
        machines_workload[machine] += jobs[i];
    }

    let c_max: u32 = *machines_workload.iter().max().unwrap();

    Solution::new(c_max, Schedule::new(input.unsort_schedule(schedule)), RR)
}

/// Assigns the jobs to random machines
pub fn random_fit(input: &SortedInput) -> Solution {
    println!("running RF algorithm...");
    let machine_count = *input.get_input().get_machine_count() as usize;
    let jobs = input.get_input().get_jobs();

    let mut schedule: Vec<(u32, u32)> = Vec::with_capacity(jobs.len());
    let mut machines_workload: Vec<u32> = vec![0; machine_count];
    let mut rng = rand::thread_rng();


    for i in 0..jobs.len() {
        let random_index = rng.gen_range(0..machine_count);
        schedule.push((random_index as u32, machines_workload[random_index]));
        machines_workload[random_index] += jobs[i];
    }

    let c_max: u32 = *machines_workload.iter().max().unwrap();

    Solution::new(c_max, Schedule::new(input.unsort_schedule(schedule)), RF)
}
