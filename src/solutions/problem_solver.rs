use crate::problems::problem::Problem;

use super::solution::Solution;


pub struct ProblemSolver{
    solutions: Vec<Solution>
}

impl ProblemSolver{
    pub fn new() -> ProblemSolver {
        return ProblemSolver { solutions: Vec::new() }
    }

    pub fn add_solution(&mut self, solution: Solution){
        self.solutions.push(solution);
    }

    pub fn get_solution(&self, problem: &Problem) -> &Solution{
        todo!("aaa");
    }

    pub fn to_string(&self) -> String{
        let mut output = String::new();

        output.push_str("{\n");

        for solution in &self.solutions {
            output.push_str(&solution.to_string());
            output.push_str("\n");
        }

        output.push_str("}");

        return output;
    }



}