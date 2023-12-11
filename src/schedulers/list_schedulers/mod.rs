use rand::Rng;

pub mod lpt_scheduler;
pub mod rr_scheduler;
pub mod rf_scheduler;
pub mod ff_scheduler;
pub mod bf_scheduler;

/// Schedulers using algorithms from the List Scheduling family to solve the makespan-minimization problem

fn assign_job(schedule: &mut Vec<(u32, u32)>, machines_workload: &mut [u32], job: u32, index: usize) {
    schedule.push((index as u32, machines_workload[index]));
    machines_workload[index] += job;
}
