use std::rc::Rc;

use permutation::Permutation;

use crate::input::input::Input;

#[derive(Debug)]
pub struct SortedInput {
    input: Rc<Input>,
    permutation: Permutation, //used for sorting and reversing the sorting
}

impl SortedInput {
    pub fn new(machine_count: usize, jobs: Vec<u32>) -> Self {
        let mut input = Input::new(machine_count, jobs);

        let compare_desc = |a: &u32, b: &u32| b.cmp(a);
        let permutation = permutation::sort_by(input.get_jobs(), compare_desc);

        input.get_mut_jobs().sort_by(compare_desc);

        Self {
            input: Rc::new(input),
            permutation,
        }
    }

    pub fn get_input(&self) -> Rc<Input> {
        self.input.clone()
    }

    pub fn get_mut_permutation(&mut self) -> &mut Permutation {
        &mut self.permutation
    }

    pub fn unsort_schedule<T: Clone>(&self, schedule: &[T]) -> Vec<T> { //TODO (low prio) löschen wenn nicht verwendet wird -> wurde wo anders hin ausgelagert
        self.permutation.apply_inv_slice(schedule)
    }
}