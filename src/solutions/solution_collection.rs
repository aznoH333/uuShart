
use std::{collections::HashMap, fs::File, io::Write};

use crate::utils::savable::Savable;

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

    pub fn log_solutions_to_console(&self) {
        println!("{}", self.to_string());
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();

        for solution in &self.solutions {
            output.push_str(solution.1.to_string().as_str());
            output.push_str("\n");
        }

        return output;
    }

}

impl Savable for SolutionCollection {

    fn save_to_file(&self, path: String) {
        let mut file = File::create(path).unwrap();
        file.write_all(self.to_string().as_bytes()).unwrap();
    }
}
