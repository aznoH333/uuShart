use async_trait::async_trait;
use thirtyfour::{By, DesiredCapabilities, WebDriver};

use crate::{information_gatherer::{information_gatherer::InformationGatherer, information_resource::InformationResource, information_resource_type::InformationResourceType}, solutions::problem_solver::ProblemSolver};

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
    async fn gather_information(&mut self) -> Option<Box<dyn ProblemSolver>> {
        if *self.information_resource.get_resource_type() != InformationResourceType::UuKit {
            return None;
        }
        
        let caps = DesiredCapabilities::firefox();
        let driver = WebDriver::new("http://localhost:9515", caps).await.unwrap();
    
        // Navigate to https://wikipedia.org.
        driver.goto("https://wikipedia.org").await.unwrap();
        let elem_form = driver.find(By::Id("search-form")).await.unwrap();
    
        // Find element from element.
        let elem_text = elem_form.find(By::Id("searchInput")).await.unwrap();
    
        // Type in the search terms.
        elem_text.send_keys("selenium").await.unwrap();
    
        // Click the search button.
        let elem_button = elem_form.find(By::Css("button[type='submit']")).await.unwrap();
        elem_button.click().await.unwrap();
    
        // Look for header to implicitly wait for the page to load.
        driver.find(By::ClassName("firstHeading")).await.unwrap();
        println!("{}", driver.title().await.unwrap());
    
        // Always explicitly close the browser.
        driver.quit().await.unwrap();


        
        todo!("return solver");
    }
}