use std::env;

use information_gatherer::{information_gatherer::InformationGatherer, resources::{unicorn_information_gatherer::UnicornInformationGatherer, unicorn_information_resource::UnicornInformationResource, unicorn_test_gatherer::UnicornTestGatherer}};
use utils::savable::Savable;

pub mod solutions;
pub mod information_gatherer;
pub mod selenium_wrapper;
pub mod utils;

#[tokio::main]
async fn main() {
    // load test course
    let resource = UnicornInformationResource::new("https://uuapp.plus4u.net/uu-coursekit-courseg01/286a85d928da49ebb60816c715ae15dc/course/testMenu".to_owned());

    let mut gatherer = UnicornTestGatherer::new(Box::new(resource)).await;
        
    let solver = gatherer.gather_information().await.unwrap();


    solver.save_to_file("./test4.txt".to_owned());


}
