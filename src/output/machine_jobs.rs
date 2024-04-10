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

    pub fn as_slice(&self) -> &[(u32, Vec<usize>)] {
        self.0.as_slice()
    }
    pub fn as_mut_slice(&mut self) -> &mut [(u32, Vec<usize>)] {
        self.0.as_mut_slice()
    }

    pub fn sort_jobs(&mut self) {
        for (_, job_indices) in &mut self.0 {
            job_indices.sort_by(|a, b| b.cmp(a));
            //job_indices.sort_by(|&a, &b| jobs[a].cmp(&jobs[b])); //so müsste man eig vergleichen aber die jobs sind ja schon sortiert daher reicht es die indices rückwärts zu sortieren
        }
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

    pub fn get_lightest_machine_index(&self) -> usize {
        let mut lightest_index = 0;
        for i in 0..self.0.len() {
            if self.0[i].0 < self.0[lightest_index].0 {
                lightest_index = i;
            }
        }
        lightest_index
    }

    pub fn get_heaviest_machine_index(&self) -> usize {
        let mut heaviest_index = 0;
        for i in 0..self.0.len() {
            if self.0[i].0 > self.0[heaviest_index].0 {
                heaviest_index = i;
            }
        }
        heaviest_index
    }

    pub fn calculate_schedule(&self, jobs: &[u32]) -> Schedule {
        Schedule::from_machine_jobs(&self, jobs, self.0.len())
    }

    /// job indices on the current machine - NOT general job index
    /// swap_indices: (m1, j1, m2, j2)
    /// if j2==-1: j1 gets pushed on m2
    pub fn swap_jobs(&mut self, swap_indices: (usize, usize, usize, i32), jobs: &[u32], keep_sorted: bool) {
        let (machine_1_index, job_1_index_on_machine, machine_2_index, job_2_index_on_machine) = swap_indices;

        if job_2_index_on_machine == -1 { //push:
            self.push_job((machine_1_index, job_1_index_on_machine, machine_2_index), jobs)
        } else { //swap:
            let job_1_index = self.0[machine_1_index].1[job_1_index_on_machine];
            let job_2_index = self.0[machine_2_index].1[job_2_index_on_machine as usize];
            self.0[machine_1_index].0 = self.0[machine_1_index].0 + jobs[job_2_index] - jobs[job_1_index];
            self.0[machine_2_index].0 = self.0[machine_2_index].0 + jobs[job_1_index] - jobs[job_2_index];
            self.0[machine_1_index].1[job_1_index_on_machine] = job_2_index;
            self.0[machine_2_index].1[job_2_index_on_machine as usize] = job_1_index;
            if keep_sorted {
                self.0[machine_1_index].1.sort_by(|a, b| b.cmp(a));
                self.0[machine_2_index].1.sort_by(|a, b| b.cmp(a));
            }
        }
    }

    /// pushes a job from a machine to another
    /// job indices on the current machine - NOT general job index
    /// push_indices: (m1, j1, m2)
    pub fn push_job(&mut self, push_indices: (usize, usize, usize), jobs: &[u32]) {
        let (machine_1_index, job_1_index_on_machine, machine_2_index) = push_indices;
        let job_1_index = self.0[machine_1_index].1[job_1_index_on_machine];
        self.0[machine_1_index].0 = self.0[machine_1_index].0 - jobs[job_1_index];
        self.0[machine_2_index].0 = self.0[machine_2_index].0 + jobs[job_1_index];
        self.0[machine_1_index].1.remove(job_1_index_on_machine);
        self.0[machine_2_index].1.push(job_1_index);
    }
}