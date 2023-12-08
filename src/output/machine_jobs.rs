#[derive(Debug)]
///<(machine0_workload,<machine0_job_numbers...>),...>
pub struct MachineJobs(Vec<(u32, Vec<u32>)>);

impl MachineJobs {
    pub fn new(machine_jobs: Vec<(u32, Vec<u32>)>) -> Self {
        Self(machine_jobs)
    }

    pub fn as_slice(&self) -> &[(u32, Vec<u32>)] { //TODO hier noch den Vec weg bekommen als slice oder sooo? evtl aber nur
        self.0.as_slice()
    }
    pub fn as_mut_slice(&mut self) -> &mut [(u32, Vec<u32>)] {
        self.0.as_mut_slice()
    }
}