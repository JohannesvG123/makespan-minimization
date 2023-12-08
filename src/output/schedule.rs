use std::fmt;

#[derive(Debug)]
///<(machine_number_job1,start_time_job1),...>
pub struct Schedule(Vec<(u32, u32)>);

impl Schedule {
    pub fn new(schedule: Vec<(u32, u32)>) -> Self {
        Self(schedule)
    }

    pub fn as_slice(&self) -> &[(u32, u32)] {
        self.0.as_slice()
    }
    pub fn as_mut_slice(&mut self) -> &mut [(u32, u32)] {
        self.0.as_mut_slice()
    }
}

impl fmt::Display for Schedule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, solution_i| {
            result.and_then(|_| write!(f, "{} {} ", (*solution_i).0, (*solution_i).1))
        })
    }
}