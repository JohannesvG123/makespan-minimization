use std::fmt;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
pub struct Solution {
    c_max: i32,
    schedule: Schedule,
}

impl Solution {
    pub fn new(c_max: i32, schedule: Schedule) -> Self {
        Solution { c_max, schedule }
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SCHEDULING_SOLUTION {0} {1}0", self.c_max, self.schedule)
    }
}


#[derive(Debug)]
///(machine_number_job1,start_time_job1),...
pub struct Schedule(Vec<(i32, i32)>);

impl Schedule {
    pub fn new(schedule: Vec<(i32, i32)>) -> Self {
        Schedule(schedule)
    }
}

impl fmt::Display for Schedule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, solution_i| {
            result.and_then(|_| write!(f, "{} {} ", (*solution_i).0, (*solution_i).1))
        })
    }
}

pub fn output(solution: Solution, write: bool, write_name: Option<String>) {
    if write {
        let write_name = write_name.unwrap();
        println!("writing output in \"{}\" ...", format!("data/{}.txt", write_name));
        let mut file = File::create(format!("data/{}.txt", write_name)).unwrap();
        file.write_all(solution.to_string().as_bytes()).unwrap();
    } else {
        println!("{}", solution);
    }
}