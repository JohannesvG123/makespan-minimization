use crate::output::schedule::Schedule;

#[derive(Debug, Clone, Eq, PartialEq)]
///<(machine0_workload,<machine0_job_numbers...>),...>
pub struct MachineJobs(Vec<(u32, Vec<usize>)>);

impl MachineJobs {
    pub fn new(machine_jobs: Vec<(u32, Vec<usize>)>) -> Self {
        Self(machine_jobs)
    }

    pub fn empty(machine_count: usize) -> Self {
        Self(vec![(0, vec![]); machine_count])
    }

    pub fn as_slice(&self) -> &[(u32, Vec<usize>)] { //TODO 2 hier noch den Vec weg bekommen als slice oder sooo? evtl aber nur
        self.0.as_slice()
    }
    pub fn as_mut_slice(&mut self) -> &mut [(u32, Vec<usize>)] {
        self.0.as_mut_slice()
    }

    pub fn get_machine_workload(&self, machine_index: usize) -> u32 {
        self.0[machine_index].0
    }

    pub fn get_machine_jobs(&self, machine_index: usize) -> &[usize] {
        self.0[machine_index].1.as_slice()
    }

    pub fn assign_job(&mut self, job_length: u32, machine_index: usize, job_index: usize) {
        self.0[machine_index].0 += job_length; //machine_workload aktualisieren
        self.0[machine_index].1.push(job_index) //job der maschine zuordnen
    }

    pub fn get_c_max(&self) -> u32 {
        let mut c_max = 0;
        for &(machine_workload, _) in self.0.iter() {
            if machine_workload > c_max {
                c_max = machine_workload;
            }
        }
        c_max
    }

    pub fn get_machines_with_workload(&self, workload: u32) -> Vec<usize> {
        let mut heaviest_machines = vec![];
        for i in 0..self.0.len() {
            if self.0[i].0 == workload {
                heaviest_machines.push(i);
            }
        }
        heaviest_machines
    }

    pub fn calculate_schedule(&self, jobs: &[u32]) -> Schedule {
        Schedule::from_machine_jobs(&self, jobs, self.0.len())
    }

    /// job indices on the current machine - NOT general job index
    /// swap_indices: (usize, usize, usize, usize)
    pub fn swap_jobs(&mut self, swap_indices: (usize, usize, usize, usize), jobs: &[u32]) {
        let (machine_1_index, job_1_index_on_machine, machine_2_index, job_2_index_on_machine) = swap_indices;

        let job_1_index = self.0[machine_1_index].1[job_1_index_on_machine];
        let job_2_index = self.0[machine_2_index].1[job_2_index_on_machine];
        self.0[machine_1_index].0 = self.0[machine_1_index].0 + jobs[job_2_index] - jobs[job_1_index];
        self.0[machine_2_index].0 = self.0[machine_2_index].0 + jobs[job_1_index] - jobs[job_2_index];
        self.0[machine_1_index].1[job_1_index_on_machine] = job_2_index;
        self.0[machine_2_index].1[job_2_index_on_machine] = job_1_index;
    }
}