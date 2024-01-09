use std::ops::Range;
use std::sync::{Arc, Mutex};

use crate::output::solution::Solution;

/// Sorted (by c_max) List of the best Solutions
#[derive(Debug)]
pub struct GoodSolutions {
    solutions: Vec<(u32, Arc<Mutex<Solution>>)>,
    //TODO Frage: funktioniert das so wie ich mir das vorstelle? mit den "doppel-mutexes"?
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
        if new_solution.is_satisfiable() {
            let new_c_max = new_solution.get_data().get_c_max();
            match self.solutions.binary_search_by_key(&new_c_max, |&(c_max, _)| c_max) {
                Ok(pos) => { // element with same c_max already in vector
                    if *self.solutions[pos].1.lock().unwrap() != new_solution { //if new_solution is a different solution than the old one
                        self.solutions.insert(pos, (new_c_max, Arc::new(Mutex::new(new_solution))));
                    }
                }
                Err(pos) => {
                    self.solutions.insert(pos, (new_c_max, Arc::new(Mutex::new(new_solution))));
                }
            }
            if self.solutions.len() == self.max_capacity { //eine alte solution wird verdrängt
                self.solutions.pop();
            }
        }
    }

    pub fn get_best_solution(&self) -> Arc<Mutex<Solution>> {
        self.get_solution(0)
    }

    pub fn get_cloned_solutions(&self, range: Range<usize>) -> Vec<Solution> { //TODO versionen mit und ohne copy impl und überall richtiges verwenden
        let mut o = vec![];
        for i in range {
            o.push(self.get_solution(i).lock().unwrap().clone());
        }
        o
    }

    pub fn get_solution(&self, index: usize) -> Arc<Mutex<Solution>> {
        debug_assert!(index < self.solutions.len());
        Arc::clone(&self.solutions[index].1)
    }

    pub fn get_best_solution_count(&self) -> usize {
        self.solutions.len()
    }

    pub fn get_solution_count(&self) -> usize {
        self.solutions.len()
    }

    pub fn get_max_capacity(&self) -> usize {
        self.max_capacity
    }

    pub fn set_max_capacity(&mut self, max_capacity: usize) {
        self.max_capacity = max_capacity;
    }
}