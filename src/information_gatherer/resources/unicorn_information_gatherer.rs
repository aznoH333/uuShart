use async_trait::async_trait;
use thirtyfour::{By, DesiredCapabilities, WebDriver};

use crate::{information_gatherer::{information_gatherer::InformationGatherer, information_resource::InformationResource, information_resource_type::InformationResourceType}, selenium_wrapper::unicorn_selenium_wrapper::UnicornSeleniumWrapper, solutions::problem_solver::ProblemSolver};

pub struct UnicornInformationGatherer{
    information_resource: Box<dyn InformationResource>
}

impl UnicornInformationGatherer{
    pub fn new(information_resource: Box<dyn InformationResource>) -> UnicornInformationGatherer {
        return UnicornInformationGatherer { information_resource: information_resource };
    }
}


#[async_trait(?Send)]
impl InformationGatherer for UnicornInformationGatherer{
    async fn gather_information(&mut self) -> Option<ProblemSolver> {
        if *self.information_resource.get_resource_type() != InformationResourceType::UuKit {
            return None;
        }
        

        let wrapper = UnicornSeleniumWrapper::new(self.information_resource.get_resource()).await;

        let mut solver = ProblemSolver::new();


        return Some(solver);


    }
}