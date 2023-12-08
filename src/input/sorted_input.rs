use permutation::Permutation;

use crate::input::input::Input;

#[derive(Debug)]
pub struct SortedInput {
    input: Input,
    permutation: Permutation, //used for sorting and reversing the sorting
}

impl SortedInput {
    pub fn new(machine_count: u32, jobs: Vec<u32>) -> Self {
        let mut input = Input::new(machine_count, jobs);

        let compare_desc = |a: &u32, b: &u32| b.cmp(a);
        let permutation = permutation::sort_by(input.get_jobs(), compare_desc);

        input.get_mut_jobs().sort_by(compare_desc);

        SortedInput { input, permutation }
    }

    pub fn get_input(&self) -> &Input {
        &self.input
    }

    pub fn get_permutation(&self) -> &Permutation {
        &self.permutation
    }

    pub fn unsort_schedule<T: Clone>(&self, schedule: &[T]) -> Vec<T> { //TODO as slice?
        self.permutation.apply_inv_slice(schedule)
    }
}