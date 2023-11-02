use permutation::Permutation;

#[derive(Debug)]
struct Input {
    machine_count: i32,
    jobs: Vec<i32>,
}

impl Input {
    fn new(machine_count: i32, jobs: Vec<i32>) -> Self { //TODO müssen hier checks eingebaut werden wie zb. m==jobs.length?
        Input { machine_count, jobs }
    }
}

#[derive(Debug)]
pub struct SortedInput {
    //TODO coolere hierarchie oder so überlegen (?)
    input: Input,
    permutation: Permutation,
}

impl SortedInput {
    pub fn new(machine_count: i32, jobs: Vec<i32>) -> Self {
        let mut input = Input::new(machine_count, jobs);
        let permutation = permutation::sort(&(input.jobs));
        input.jobs.sort();
        //println!("{:?}",permutation.apply_slice(&(input.jobs))); //this gives us the original input
        SortedInput { input, permutation }
    }
}