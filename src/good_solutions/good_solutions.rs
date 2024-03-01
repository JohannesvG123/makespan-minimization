use std::sync::Arc;

use concurrent_map::ConcurrentMap;
use permutation::Permutation;

use crate::output::{get_directory_name, log, output_solution};
use crate::output::solution::Solution;

/// Sorted (by c_max) Collection of the max_capacity best Solutions
#[derive(Debug, Clone)]
pub struct GoodSolutions {
    solutions: ConcurrentMap<(u32, usize), Solution>,
    //((c_max,index),solution),... index is needed for saving multiple solutions with the same c_max
    max_capacity: usize,
}

impl GoodSolutions {
    pub fn new(max_capacity: usize) -> Self {
        debug_assert!(max_capacity >= 1);
        Self { solutions: ConcurrentMap::new(), max_capacity }
    }

    pub fn add_solution(&self, new_solution: Solution) {
        if new_solution.is_satisfiable() {
            let new_c_max = new_solution.get_data().get_c_max();
            let mut new_index: usize = 0;

            //check if new_solution is actually new:
            if self.solutions.contains_key(&(new_c_max, 0)) {
                //there is already at least one solution with the same c_max
                //TODO PRIO hier manchmal bug, dass zwischen den zeilken die struktur verändert wird und daher solutions_to_check leer ist
                let mut solutions_to_check: Vec<_> = self.solutions.range((new_c_max, 0)..(new_c_max + 1, 0)).collect();
                for (_, solution) in &solutions_to_check {
                    if new_solution == *solution {
                        //new_solution is not new
                        return;
                    }
                }
                //new_solution is actually new
                let old_index = match solutions_to_check.pop() {
                    None => { return; } //tritt nur ein, wenn solutions_to_check leer ist wegen paralleler Ausführung
                    Some(((_, i), _)) => { i }
                };
                new_index = old_index + 1;
            }
            self.solutions.insert((new_c_max, new_index), new_solution);
            while self.solutions.len() > self.max_capacity {
                //too many solutions saved
                self.solutions.pop_last();
            }
        }
    }

    /// returns cloned best solution or None
    pub fn get_best_solution(&self) -> Option<Solution> {
        match self.solutions.first() {
            None => None,
            Some((_, solution)) => Some(solution),
        }
    }

    /// returns cloned x-th best solution or None (ATTENTION: if there are n<x solutions the n-th solution will be returned)
    pub fn get_x_best_solution(&self, x: usize) -> Option<Solution> {
        if self.solutions.len() - 1 < x {
            match self.solutions.last() {
                None => {}
                Some((_, solution)) => { return Some(solution); }
            }
        } else {
            let mut i = 0;
            for (_, solution) in self.solutions.iter() {
                if i == x {
                    return Some(solution);
                }
                i += 1;
            }
            println!("ahhhhh");
        }
        None
    }

    /// returns cloned best n solutions (or fewer, when there are no n solutions)
    pub fn get_best_solutions(&self, n: usize) -> Vec<Solution> {
        let mut solutions = vec![];
        for (_, solution) in self.solutions.iter() {
            solutions.push(solution);
            if solutions.len() == n {
                break;
            }
        }
        solutions
    }

    pub fn get_solution_count(&self) -> usize {
        self.solutions.len()
    }

    pub fn get_max_capacity(&self) -> usize {
        self.max_capacity
    }

    pub fn write_output(&self, perm: Arc<Permutation>, write: bool, directory_name: Option<String>, input_file_name: &str, write_separate_files: bool, measurement: bool) {
        log(String::from("writing output..."), false, measurement, None);


        let directory_name_str = get_directory_name(directory_name, input_file_name);

        for (_, solution) in self.solutions.iter() {
            output_solution(&solution, Arc::clone(&perm), write, directory_name_str.clone(), input_file_name, write_separate_files, measurement);
        }
    }
}