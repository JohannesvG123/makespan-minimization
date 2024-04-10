use crate::output::machine_jobs::MachineJobs;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Data {
    c_max: u32,
    machine_jobs: MachineJobs,
}

impl Data {
    pub fn new(c_max: u32, machines: MachineJobs) -> Self {
        Self { c_max, machine_jobs: machines }
    }

    pub fn get_c_max(&self) -> u32 {
        self.c_max
    }
    pub fn get_machine_jobs(&self) -> &MachineJobs {
        &self.machine_jobs
    }
    pub fn get_mut_machine_jobs(&mut self) -> &mut MachineJobs {
        &mut self.machine_jobs
    }

    ///job_1_index_on_machine means the index of job1 on its current machine (in MachineJobs)
    /// swap_indices=(machine_1_index, job_1_index, machine_2_index, job_2_index)
    pub fn swap_jobs(&mut self, swap_indices: (usize, usize, usize, i32), jobs: &[u32], keep_sorted: bool) {
        self.machine_jobs.swap_jobs(swap_indices, jobs, keep_sorted);
        self.c_max = self.machine_jobs.get_c_max();
    }
}