use std::fmt;
use std::fs::File;
use std::io::Write;

//TODO Überprüfung ob der schedule usw gültig ist muss hier nicht gemacht werden oder?
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

pub fn output(solution: Solution, write: bool) {
    if write {
        let mut file = File::create("data/output.txt").unwrap();//TODO Namen dynamisch anpassen
        file.write_all(solution.to_string().as_bytes()).unwrap();
    } else {
        println!("{}", solution);
    }
}