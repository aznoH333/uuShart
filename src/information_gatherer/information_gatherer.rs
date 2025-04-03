use async_trait::async_trait;

use crate::solutions::problem_solver::SolutionCollection;

#[async_trait(?Send)]
pub trait InformationGatherer{
    async fn gather_information(&mut self) -> Option<SolutionCollection>;
}