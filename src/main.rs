use information_gatherer::{information_gatherer::InformationGatherer, resources::{unicorn_information_gatherer::UnicornInformationGatherer, unicorn_information_resource::UnicornInformationResource}};


pub mod solutions;
pub mod problems;
pub mod information_gatherer;

fn main() {
    let resource = UnicornInformationResource::new("test".to_owned());

    let mut gatherer = UnicornInformationGatherer::new(Box::new(resource));

    gatherer.gather_information();

    println!("Hello, world!");
}
