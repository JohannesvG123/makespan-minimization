use rand::Rng;

use crate::Algorithm;
use crate::Algorithm::{BF, FF, LPT, RF, RR};
use crate::input::SortedInput;
use crate::output::{Schedule, Solution};

/// Schedulers using algorithms from the LS (List Scheduling family) to solve the makespan-minimization problem

/// Assigns the biggest job to the least loaded machine until all jobs are assigned (= worst fit)
pub fn longest_processing_time(input: &SortedInput, upper_bound: Option<u32>) -> Solution {
    let (machine_count, jobs, upper_bound, mut schedule, mut machines_workload) = init(input, upper_bound, LPT);
    let mut current_machine: usize = 0;
    let mut foreward: bool = true; // used to fill the machines in this order: (m=3) 0-1-2-2-1-0-0-1-2...
    let mut pause: bool = false;

    for &job in jobs.iter() {
        if machines_workload[current_machine] + job > upper_bound { //satisfiability check
            println!("ERROR: upper bound {} is to low for the {:?}-algorithm with this input", upper_bound, LPT);
            return Solution::unsatisfiable(LPT);
        }
        assign_job(&mut schedule, machines_workload.as_mut_slice(), job, current_machine);

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

    end(input, &schedule, &mut machines_workload, LPT)
}

/// Assigns the biggest job to the most loaded machine (that can fit the job) until all jobs are assigned
pub fn best_fit(input: &SortedInput, upper_bound: Option<u32>) -> Solution {
    let (machine_count, jobs, upper_bound, mut schedule, mut machines_workload) = init(input, upper_bound, BF);

    for &job in jobs.iter() {
        let mut best_machine = 0;
        let mut fitting_machine_found = false;
        for m in 0..machine_count { //man könnte hier speedup erreichen wenn man ab Eingabegröße x eine BH-PQ nutzt...
            if !fitting_machine_found && machines_workload[m] + job <= upper_bound {
                best_machine = m;
                fitting_machine_found = true
            } else if fitting_machine_found && machines_workload[m] + job <= upper_bound && machines_workload[m] + job > machines_workload[best_machine] + job {
                best_machine = m;
            }
        }
        if !fitting_machine_found { //satisfiability check
            println!("ERROR: upper bound {} is to low for the {:?}-algorithm with this input", upper_bound, BF);
            return Solution::unsatisfiable(BF);
        }

        assign_job(&mut schedule, machines_workload.as_mut_slice(), job, best_machine);
    }

    end(input, &schedule, &mut machines_workload, BF)
}

/// Assigns the biggest job to the machine with the smallest index until all jobs are assigned
pub fn first_fit(input: &SortedInput, upper_bound: Option<u32>) -> Solution {
    let (machine_count, jobs, upper_bound, mut schedule, mut machines_workload) = init(input, upper_bound, FF);

    for &job in jobs.iter() {
        let mut current_machine: usize = 0;

        if machines_workload[current_machine] + job > upper_bound {
            current_machine += 1;
            if current_machine == machine_count { //satisfiability check
                println!("ERROR: upper bound {} is to low for the {:?}-algorithm with this input", upper_bound, FF);
                return Solution::unsatisfiable(FF);
            }
        }

        assign_job(&mut schedule, machines_workload.as_mut_slice(), job, current_machine);
    }

    end(input, &schedule, &mut machines_workload, FF)
}

/// Round Robin job assignment
pub fn round_robin(input: &SortedInput, upper_bound: Option<u32>) -> Solution {
    let (machine_count, jobs, upper_bound, mut schedule, mut machines_workload) = init(input, upper_bound, RR);

    for j in 0..jobs.len() {
        let mut machine = j.rem_euclid(machine_count);

        let mut offset = 0;
        while machines_workload[(machine + offset).rem_euclid(machine_count)] + jobs[j] > upper_bound {
            offset += 1;
            if offset == machine_count { //satisfiability check
                println!("ERROR: upper bound {} is to low for the {:?}-algorithm with this input", upper_bound, RR);
                return Solution::unsatisfiable(RR);
            }
        }
        machine += offset;

        assign_job(&mut schedule, machines_workload.as_mut_slice(), jobs[j], machine);
    }

    let c_max: u32 = *machines_workload.iter().max().unwrap();

    Solution::new(c_max, Schedule::new(input.unsort_schedule(&schedule)), RR)
}

/// Assigns the jobs to random machines
pub fn random_fit(input: &SortedInput, upper_bound: Option<u32>) -> Solution { //TODO FRAGE hatte mir aufgeschrieben, dass hier kein ub genutzt werden soll... stimmt das?
    let (machine_count, jobs, upper_bound, mut schedule, mut machines_workload) = init(input, upper_bound, RF);
    let mut rng = rand::thread_rng();
    let fails_until_check: usize = machine_count;// Number of fails until a satisfiability check is done //TODO FRAGE passt das so oder anderer wert?

    for &job in jobs.iter() {
        let mut random_index = rng.gen_range(0..machine_count);

        let mut fails: usize = 0;
        while machines_workload[random_index] + job > upper_bound {
            fails += 1;
            if fails == fails_until_check {
                if machines_workload.iter().any(|machine_workload| machine_workload + job <= upper_bound) { //satisfiability check
                    fails = 0;
                } else {
                    println!("ERROR: upper bound {} is to low for the {:?}-algorithm with this input", upper_bound, RF);
                    return Solution::unsatisfiable(RF);
                }
            }
            random_index = rng.gen_range(0..machine_count);
        }

        assign_job(&mut schedule, machines_workload.as_mut_slice(), job, random_index);
    }

    end(input, schedule.as_slice(), machines_workload.as_slice(), RF)
}

fn init(input: &SortedInput, upper_bound: Option<u32>, algorithm: Algorithm) -> (usize, &[u32], u32, Vec<(u32, u32)>, Vec<u32>) {
    println!("running {:?} algorithm...", algorithm);
    let machine_count = *input.get_input().get_machine_count() as usize;
    let jobs = input.get_input().get_jobs().as_slice();
    let upper_bound: u32 = match upper_bound {
        None => jobs.iter().sum::<u32>() / machine_count as u32 + jobs.iter().max().unwrap(), //trvial upper bound
        Some(val) => val
    };

    (machine_count,
     jobs,
     upper_bound, //TODO 1 überlegen ob man ub auch laufendem algo geben kann + atomic shared lb+ub
     Vec::with_capacity(jobs.len()), //schedule
     vec![0; machine_count]) //machines_workload
}

fn assign_job(schedule: &mut Vec<(u32, u32)>, machines_workload: &mut [u32], job: u32, index: usize) {
    schedule.push((index as u32, machines_workload[index])); //TODO FRAGE slice hier nicht möglich oder
    machines_workload[index] += job;
}


fn end(input: &SortedInput, schedule: &[(u32, u32)], machines_workload: &[u32], algorithm: Algorithm) -> Solution {
    let c_max: u32 = *machines_workload.iter().max().unwrap();

    Solution::new(c_max, Schedule::new(input.unsort_schedule(schedule)), algorithm)
}