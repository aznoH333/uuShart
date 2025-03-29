use async_trait::async_trait;

use crate::solutions::problem_solver::ProblemSolver;

#[async_trait(?Send)]
pub trait InformationGatherer{
    async fn gather_information(&mut self) -> Option<Box<dyn ProblemSolver>>;
}