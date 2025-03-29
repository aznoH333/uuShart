use crate::{information_gatherer::{information_gatherer::InformationGatherer, information_resource::InformationResource, information_resource_type::InformationResourceType}, solutions::problem_solver::ProblemSolver};

pub struct UnicornInformationGatherer{
    information_resource: Box<dyn InformationResource>
}

impl UnicornInformationGatherer{
    pub fn new(information_resource: Box<dyn InformationResource>) -> UnicornInformationGatherer {
        return UnicornInformationGatherer { information_resource: information_resource };
    }
}

impl InformationGatherer for UnicornInformationGatherer{
    fn gather_information(&mut self) -> Option<Box<dyn ProblemSolver>> {
        if *self.information_resource.get_resource_type() != InformationResourceType::UuKit {
            return None;
        }
        
        todo!("return solver");
    }
}