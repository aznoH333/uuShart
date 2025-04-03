use async_trait::async_trait;
use thirtyfour::By;

use crate::{information_gatherer::{information_gatherer::InformationGatherer, information_resource::InformationResource}, selenium_wrapper::unicorn_selenium_wrapper::UnicornSeleniumWrapper, solutions::problem_solver::ProblemSolver};

pub struct UnicornTestGatherer{
    information_resource: Box<dyn InformationResource>,
    selenium_wrapper: UnicornSeleniumWrapper
}

impl UnicornTestGatherer{
    pub async fn new(information_resource: Box<dyn InformationResource>) -> UnicornTestGatherer {
        return UnicornTestGatherer{
            selenium_wrapper: UnicornSeleniumWrapper::new(information_resource.get_resource()).await,
            information_resource: information_resource,
        }
    }

    async fn start_test(&mut self, test_index: usize){
        // choose test
        let buttons = self.selenium_wrapper.get_elements(By::ClassName("uu-coursekit-button-full")).await.unwrap();
        buttons.get(test_index).unwrap().click().await.unwrap();

        // start test
        self.selenium_wrapper.click_element(By::ClassName("uu5-bricks-button-xl")).await;

    }

    /*
        Vyber správnou odpověď
        Označ správné odpovědi
        Doplň část věty
     */

    async fn solve_test(&mut self){
        // get question type
        let question_type_element = self.selenium_wrapper.get_element(By::XPath("/html/body/div[1]/div/div[7]/div[4]/div/div/div/div/div[2]/div[1]/div[1]/div[2]")).await.unwrap();

        println!("{}", question_type_element.text().await.unwrap());

        todo!("solve tests")
    }

    async fn log_solutions(&mut self){
        todo!("log solutions")
    }

    async fn return_to_main_page(&mut self){
        todo!("return to main page");
    }

}

#[async_trait(?Send)]
impl InformationGatherer for UnicornTestGatherer {
    async fn gather_information(&mut self) -> Option<ProblemSolver> {
        
        // process main page    
        let button_count = self.selenium_wrapper.get_elements(By::ClassName("uu-coursekit-button-full")).await.unwrap().iter().count();
        
        // iterate tests
        for i in 0..button_count {
            self.start_test(i).await;
            self.solve_test().await;
            self.log_solutions().await;
            self.return_to_main_page().await;
        }


        todo!();
    }


}

