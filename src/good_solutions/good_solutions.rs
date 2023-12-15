use std::sync::{Arc, Mutex};

use crate::output::solution::Solution;

/// Sorted (by c_max) List of the best Solutions
pub struct GoodSolutions {
    //TODO 1 fertig implementieren
    //todo struct selber wird in nem arc mutex gehalten und jede solution kann als arc mutzex geholt werden
    solutions: Vec<(u32, Arc<Mutex<Solution>>)>,
    //TODO Frage: funktioniert das so wie ich mir das vorstelle? mit den mutexes
    //(c_max1,solution1),(c_max2,solution2)...
    max_capacity: usize,
}

impl GoodSolutions {
    pub fn new(max_capacity: usize) -> Self {
        debug_assert!(max_capacity >= 1);
        Self { solutions: Vec::with_capacity(max_capacity), max_capacity }
    }

    ///adds solution if it is better than the current best solutions
    pub fn add_solution(&mut self, new_solution: Solution) {
        let new_c_max = new_solution.get_data().get_c_max();
        match self.solutions.binary_search_by_key(&new_c_max, |&(c_max, _)| c_max) {
            Ok(pos) => { // element already in vector
                //TODO in dem Fall prüfen ob die solutions gleich sind und dann ggf die neue solution rein schieben     (mit unterscheidung analog wie im Err Fall)
            }
            Err(pos) => {
                self.solutions.insert(pos, (new_c_max, Arc::new(Mutex::new(new_solution))));
            }
        }
        if self.solutions.len() == self.max_capacity { //eine alte solution wird verdrängt
            self.solutions.pop();
        }
    }

    pub fn get_best_solution(&self) {
        self.get_solution(0);
    }

    pub fn get_solution(&self, index: usize) -> Arc<Mutex<Solution>> {
        debug_assert!(index < self.solutions.len());
        Arc::clone(&self.solutions[index].1)
    }

    pub fn get_best_solution_count(&self) -> usize {
        self.solutions.len()
    }

    pub fn get_max_capacity(&self) -> usize {
        self.max_capacity
    }

    pub fn set_max_capacity(&mut self, max_capacity: usize) {
        self.max_capacity = max_capacity;
    }
}