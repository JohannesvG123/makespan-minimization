use std::sync::Arc;

use permutation::Permutation;

use crate::input::input::Input;

#[derive(Debug)]
pub struct SortedInput {
    input: Arc<Input>,
    permutation: Arc<Permutation>, //used for sorting and reversing the sorting
}

impl SortedInput {
    pub fn new(machine_count: usize, jobs: Vec<u32>) -> Self {
        let mut input = Input::new(machine_count, jobs);

        let compare_desc = |a: &u32, b: &u32| b.cmp(a);
        let permutation = permutation::sort_by(input.get_jobs(), compare_desc);

        input.get_mut_jobs().sort_by(compare_desc);

        Self {
            input: Arc::new(input),
            permutation: Arc::new(permutation),
        }
    }

    pub fn get_input(&self) -> Arc<Input> {
        Arc::clone(&self.input)
    }

    pub fn get_permutation(&self) -> Arc<Permutation> {
        Arc::clone(&self.permutation)
    }
}