use std::cmp::max;

use crate::Algorithm;
use crate::Algorithm::RF;
use crate::input::SortedInput;
use crate::list_schedulers::random_fit;
use crate::output::{Schedule, Solution};

/// Schedulers using algorithms from the Local Search family to solve the makespan-minimization problem

/// rebalance jobs from the heaviest and the lightest machines
pub fn rebalance(solution_old: Solution, input: &SortedInput) -> Solution {
    debug_assert!(solution_old.is_satisfiable());

    let machine_count = *input.get_input().get_machine_count() as usize;
    let jobs = input.get_input().get_jobs().as_slice();
    let upper_bound: u32 = solution_old.get_c_max();
    let schedule = solution_old.get_schedule();

    let mut machines_workload = vec![0; machine_count];
    for i in 0..jobs.len() {//TODO machines_workload in solution mit übergeben damit manns net neu berechnen muss?
        machines_workload[schedule[i].0 as usize] += jobs[i];
    }
    println!("{:?}", solution_old);
    println!("{:?}", machines_workload);

    //Maschinen nach füllung sortieren
    let compare_desc = |a: &u32, b: &u32| b.cmp(a);
    let permutation = permutation::sort_by(&machines_workload, compare_desc);
    machines_workload.sort_by(compare_desc);//permutation.apply_inv_slice(&machines_workload)); //this gives us the original machines_workload
    println!("{:?}", machines_workload);

    //Machine mit max füllung index finden
    let max_index = permutation.apply_inv_idx(0) as u32;
    //Machinen mit min füllung index finden
    let min_1_index = permutation.apply_inv_idx(machine_count - 1) as u32;
    let min_2_index = permutation.apply_inv_idx(machine_count - 2) as u32;
    println!("{} {} {}", max_index, min_1_index, min_2_index);

    //neue probleminstanz erstellen
    let mut rebalance_jobs = vec![];
    for i in 0..jobs.len() {
        let machine_index = schedule[i].0;
        if machine_index == max_index || machine_index == min_1_index || machine_index == min_2_index {
            rebalance_jobs.push(jobs[i]);
        }
    }
    println!("{:?}", rebalance_jobs);
    let rebalance_input = SortedInput::new(3, rebalance_jobs); //TODO case with <3 machines

    //lösen
    let rebalance_solution = random_fit(&rebalance_input, Some(upper_bound), false);
    println!("{}", rebalance_solution);

    if rebalance_solution.is_satisfiable() {
        let c_max: u32 = max(rebalance_solution.get_c_max(), machines_workload[1]); //rebalance cmax vs 2.größte maschine von davor
        if c_max < upper_bound { //upper_bound ==old_Cmax
            //zusammenführen
            let schedule_new: Vec<&(u32, u32)> = schedule.clone().iter().filter(|&&(machine_index, _)| machine_index != max_index && machine_index != min_1_index && machine_index != min_2_index).collect(); //die rebalanced rausschmeißen
            println!("ol {:?}", schedule);
            println!("da {:?}", schedule_new);
            //die rebalanced rein hauen
            for &(machine_index, enum_map) in rebalance_solution.get_schedule().iter() {
                //TODO überlegen wie des mit den indices funzt
            }

            println!("old cmax {} -> new {}", upper_bound, c_max);
            Solution::new(c_max, Schedule::new(input.unsort_schedule(schedule)), RF)
        } else {
            solution_old //TODO überlegen was dann
        }
    } else {
        solution_old //TODO überlegen was dann
    }
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
     upper_bound, //TODO FRAGE Wenn man ub auch laufendem algo geben kann wie geht der damit um? + atomic shared lb+ub (das aber im parrallelen design)
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