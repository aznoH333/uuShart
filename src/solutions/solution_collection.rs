
use std::collections::HashMap;

use super::solution::Solution;


pub struct SolutionCollection{
    solutions: HashMap<String, Solution>
}

impl SolutionCollection{
    pub fn new() -> SolutionCollection {
        return SolutionCollection { solutions: HashMap::new() };
    }

    pub fn add_solution(&mut self, solution: Solution){

        if !self.solutions.contains_key(solution.get_label()) {
            self.solutions.insert(solution.get_label().to_owned(), solution);
        }

    }

}