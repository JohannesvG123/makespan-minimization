use rand::Rng;

pub mod lpt_scheduler;
pub mod rr_scheduler;
pub mod rf_scheduler;
pub mod ff_scheduler;
pub mod bf_scheduler;

// Schedulers using algorithms from the List Scheduling family to solve the makespan-minimization problem