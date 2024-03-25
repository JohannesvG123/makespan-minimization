use std::fmt;
use std::sync::Arc;

use permutation::Permutation;

use crate::output::machine_jobs::MachineJobs;

#[derive(Debug, Clone, Eq, PartialEq)]
///<(machine_number_job1,start_time_job1),...>
pub struct Schedule(Vec<(usize, u32)>);

impl Schedule {
    pub fn new(schedule: Vec<(usize, u32)>) -> Self {
        Self(schedule)
    }

    pub fn empty(job_count: usize) -> Self {
        Self(Vec::with_capacity(job_count))
    }

    pub fn from_machine_jobs(machine_jobs: &MachineJobs, jobs: &[u32], machine_count: usize) -> Self {
        let mut schedule = vec![(0, 0); jobs.len()]; //(1,1) wird jeweils eh alles überschrieben

        for m in 0..machine_count {
            let mut machine_workload_tmp: u32 = 0;
            for &job_index in machine_jobs.get_machine_jobs(m) {
                schedule[job_index] = (m, machine_workload_tmp);
                machine_workload_tmp += jobs[job_index];
            }
        }

        Self(schedule)
    }

    pub fn as_slice(&self) -> &[(usize, u32)] {
        self.0.as_slice()
    }
    pub fn as_mut_slice(&mut self) -> &mut [(usize, u32)] {
        self.0.as_mut_slice()
    }

    pub fn add_job(&mut self, machine_number: usize, job_start_time: u32) {
        self.0.push((machine_number, job_start_time));
    }

    pub fn unsort(&mut self, permutation: Arc<Permutation>) {
        self.0 = permutation.apply_inv_slice(self.0.as_slice())
    }
}

impl fmt::Display for Schedule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, solution_i| {
            result.and_then(|_| write!(f, "{} {} ", (*solution_i).0, (*solution_i).1))
        })
    }
}