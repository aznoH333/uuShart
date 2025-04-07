use async_trait::async_trait;

use crate::solutions::solution_collection::SolutionCollection;

#[async_trait(?Send)]
pub trait InformationGatherer{
    async fn gather_information(&mut self) -> Option<&SolutionCollection>;
}