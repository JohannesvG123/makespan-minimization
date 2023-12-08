use std::rc::Rc;

use crate::Algorithm;
use crate::input::sorted_input::SortedInput;
use crate::output::Solution;

//TODO DESIGN-FRAGE

pub struct SchedulingObjects {
    //TODO getter/setter statt pub
    pub input: Rc<SortedInput>,
    upper_bound: u32,
    schedule: Vec<(u32, u32)>,
    machines_workload: Vec<u32>,
}

impl SchedulingObjects {
    pub fn new(input: Rc<SortedInput>, upper_bound: Option<u32>, algorithm: Algorithm) -> Self {
        println!("starting {:?} algorithm...", algorithm);
        let jobs = input.get_input().get_jobs();
        let machine_count = input.get_input().get_machine_count();
        let jobs_len = jobs.len();

        let upper_bound: u32 = match upper_bound {
            None => jobs.iter().sum::<u32>() / machine_count + jobs.iter().max().unwrap(),//trvial upper bound
            Some(val) => val
        };

        SchedulingObjects {
            input,
            upper_bound, //TODO 1 überlegen ob man ub auch laufendem algo geben kann + atomic shared lb+ub
            schedule: Vec::with_capacity(jobs_len),
            machines_workload: vec![0; machine_count as usize],
        }
    }

    pub fn get_upper_bound(&mut self) -> u32 {
        self.upper_bound
    }

    pub fn get_mut_schedule(&mut self) -> &mut Vec<(u32, u32)> {
        &mut self.schedule
    }

    pub fn get_mut_machines_workload(&mut self) -> &mut [u32] {
        self.machines_workload.as_mut_slice()
    }
}

pub trait Scheduler {
    fn schedule(&mut self) -> Solution;
}