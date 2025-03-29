use crate::problems::problem::Problem;

use super::solution::Solution;

pub trait ProblemSolver{
    fn get_solution(&self, problem: &Problem) -> &Box<dyn Solution>;
}