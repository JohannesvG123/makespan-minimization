use std::sync::Arc;

use permutation::Permutation;

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
    pub fn swap_jobs(&mut self, machine_1_index: usize, job_1_index_on_machine: usize, machine_2_index: usize, job_2_index_on_machine: usize, jobs: &[u32], machine_count: usize) {
        self.machine_jobs.swap_jobs(machine_1_index, job_1_index_on_machine, machine_2_index, job_2_index_on_machine, jobs);
        //TODO Schedule.swap statt neuberechnung:
        self.schedule = Schedule::from_machine_jobs(self.get_machine_jobs(), jobs, machine_count);
        self.c_max = self.machine_jobs.get_c_max();
    }

    pub fn unsort_inplace(&mut self, permutation: &mut Permutation) {
        permutation.apply_inv_slice_in_place(self.schedule.as_mut_slice());
        //permutation.apply_inv_slice_in_place(self.machine_jobs.as_mut_slice()); //TODO 2 rein machen und anpassen!
    }

    pub fn unsort(&mut self, permutation: Arc<Permutation>) {
        self.schedule = Schedule::new(permutation.apply_inv_slice(self.schedule.as_slice()));
        //permutation.apply_inv_slice_in_place(self.machine_jobs.as_mut_slice()); //TODO 2 rein machen und anpassen!
    }
}