use information_gatherer::{information_gatherer::InformationGatherer, resources::{unicorn_information_gatherer::UnicornInformationGatherer, unicorn_information_resource::UnicornInformationResource, unicorn_test_gatherer::UnicornTestGatherer}};
use utils::savable::Savable;

pub mod solutions;
pub mod information_gatherer;
pub mod selenium_wrapper;
pub mod utils;

#[tokio::main]
async fn main() {
    // load test course
    let resource = UnicornInformationResource::new("https://uuapp.plus4u.net/uu-coursekit-courseg01/99923616732453117-8d5e19993bc042da8b5dd7812c93dfef/course/testMenu".to_owned());

    let mut gatherer = UnicornTestGatherer::new(Box::new(resource)).await;
        
    let solver = gatherer.gather_information().await.unwrap();


    solver.save_to_file("./test.txt".to_owned());


}
