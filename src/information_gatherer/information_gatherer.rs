use crate::solutions::problem_solver::ProblemSolver;

pub trait InformationGatherer{
    fn gather_information(&mut self) -> Option<Box<dyn ProblemSolver>>;
}