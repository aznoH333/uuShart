use crate::problems::problem_type::ProblemType;

use super::solution::Solution;

pub trait InformationUnit{
    fn get_label() -> String;
    fn get_type() -> ProblemType;
    fn get_solution() -> Box<dyn Solution>;
}