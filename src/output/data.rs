use std::sync::Arc;

use permutation::Permutation;

use crate::output::machine_jobs::MachineJobs;
use crate::output::schedule::Schedule;

#[derive(Debug,Clone)]
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

    pub fn unsort_inplace(&mut self, permutation: &mut Permutation) {
        permutation.apply_inv_slice_in_place(self.schedule.as_mut_slice());
        //permutation.apply_inv_slice_in_place(self.machine_jobs.as_mut_slice()); //TODO 2 rein machen und anpassen!
    }

    pub fn unsort(&mut self, permutation: Arc<Permutation>) {
        self.schedule = Schedule::new(permutation.apply_slice(self.schedule.as_slice()));
        //permutation.apply_inv_slice_in_place(self.machine_jobs.as_mut_slice()); //TODO 2 rein machen und anpassen!
    }
}