use crate::input::SortedInput;
use crate::output::{Schedule, Solution};

/// Schedulers using algorithms from the LS (List Scheduling family) to solve the makespan-minimization problem

/// Assigns the biggest job to the least loaded machine until all jobs are assigned
pub fn lpt(input: &SortedInput) -> Solution { // TODO 1 testen mit verschiedenen inputs
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

    Solution::new(c_max, Schedule::new(input.unsort_schedule(schedule)))
}
