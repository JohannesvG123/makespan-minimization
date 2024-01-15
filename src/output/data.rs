use std::sync::Arc;

use permutation::Permutation;

use crate::global_bounds::bounds::Bounds;
use crate::output::machine_jobs::MachineJobs;
use crate::output::schedule::Schedule;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Data {
    c_max: u32,
    schedule: Schedule,
    machine_jobs: MachineJobs,
}

impl Data {
    pub fn new(c_max: u32, schedule: Schedule, machines: MachineJobs) -> Self {
        Self { c_max, schedule, machine_jobs: machines }
    }

    pub fn get_c_max(&self) -> u32 {
        self.c_max
    }
    pub fn get_schedule(&self) -> &Schedule {
        &self.schedule
    }
    pub fn get_machine_jobs(&self) -> &MachineJobs {
        &self.machine_jobs
    }
    pub fn get_mut_machine_jobs(&mut self) -> &mut MachineJobs {
        &mut self.machine_jobs
    }

    ///job_1_index_on_machine means the index of job1 on its current machine (in MachineJobs)
    /// swap_indices=(machine_1_index, job_1_index, machine_2_index, job_2_index)
    pub fn swap_jobs(&mut self, swap_indices: (usize, usize, usize, usize), jobs: &[u32], machine_count: usize, global_bounds: Arc<Bounds>) {
        self.machine_jobs.swap_jobs(swap_indices, jobs);
        self.schedule = Schedule::from_machine_jobs(self.get_machine_jobs(), jobs, machine_count);//TODO (low prio) Schedule.swap smart implementiert statt neuberechnung kann speedup bringen => wobeiman dann auch direkt so umbauen kann: solution hat nur machines workload und schedule wird nur beim output berechnet
        self.update_c_max(global_bounds);
    }

    fn update_c_max(&mut self, global_bounds: Arc<Bounds>) {
        self.c_max = self.machine_jobs.get_c_max();
        global_bounds.update_upper_bound(self.c_max);
    }

    pub fn get_unsorted_schedule(&self, permutation: Arc<&Permutation>) -> Schedule {
        Schedule::new(permutation.apply_inv_slice(self.schedule.as_slice()))
    }
}