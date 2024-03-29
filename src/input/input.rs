#[derive(Debug)]
pub struct Input {
    machine_count: usize,
    jobs: Vec<u32>,
}

impl Input {
    pub fn new(machine_count: usize, jobs: Vec<u32>) -> Self {
        Self { machine_count, jobs }
    }

    pub fn get_machine_count(&self) -> usize {
        self.machine_count
    }

    pub fn get_jobs(&self) -> &[u32] {
        self.jobs.as_slice()
    }

    pub fn get_mut_jobs(&mut self) -> &mut [u32] {
        self.jobs.as_mut_slice()
    }

    pub fn get_job_count(&self) -> usize {
        self.jobs.len()
    }
}