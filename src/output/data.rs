use crate::output::machine_jobs::MachineJobs;
use crate::output::schedule::Schedule;

#[derive(Debug)]
pub struct Data {
    c_max: u32,
    schedule: Schedule,
    machines: MachineJobs,
}

impl Data {
    pub fn new(c_max: u32, schedule: Schedule, machines: MachineJobs) -> Self {
        Self { c_max, schedule, machines }
    }

    pub fn get_c_max(&self) -> u32 {
        self.c_max
    }
    pub fn get_schedule(&self) -> &Schedule {
        &self.schedule
    }
    pub fn get_machines(&self) -> &MachineJobs {
        &self.machines
    }
}