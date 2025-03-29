use information_gatherer::{information_gatherer::InformationGatherer, resources::{unicorn_information_gatherer::UnicornInformationGatherer, unicorn_information_resource::UnicornInformationResource}};

pub mod solutions;
pub mod problems;
pub mod information_gatherer;
pub mod selenium_wrapper;

#[tokio::main]
async fn main() {

    let resource = UnicornInformationResource::new("https://uuapp.plus4u.net/uu-coursekit-courseg01/99923616732453117-8d5e19993bc042da8b5dd7812c93dfef/course/courseMenu".to_owned());

    let mut gatherer = UnicornInformationGatherer::new(Box::new(resource));
        
    let solver = gatherer.gather_information().await.unwrap();



    println!("{}", solver.to_string());


}
