use std::fmt;

use crate::output::machine_jobs::MachineJobs;

#[derive(Debug,Clone,Eq, PartialEq)]
///<(machine_number_job1,start_time_job1),...>
pub struct Schedule(Vec<(u32, u32)>); //TODO (low prio) auf usize,u32 umstellen

impl Schedule {
    pub fn new(schedule: Vec<(u32, u32)>) -> Self {
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
                schedule[job_index] = (m as u32, machine_workload_tmp);
                machine_workload_tmp += jobs[job_index];
            }
        }

        Self(schedule)
    }

    pub fn as_slice(&self) -> &[(u32, u32)] {
        self.0.as_slice()
    }
    pub fn as_mut_slice(&mut self) -> &mut [(u32, u32)] {
        self.0.as_mut_slice()
    }

    pub fn add_job(&mut self, machine_number: u32, job_start_time: u32) {
        self.0.push((machine_number, job_start_time));
    }
}

impl fmt::Display for Schedule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, solution_i| {
            result.and_then(|_| write!(f, "{} {} ", (*solution_i).0, (*solution_i).1))
        })
    }
}