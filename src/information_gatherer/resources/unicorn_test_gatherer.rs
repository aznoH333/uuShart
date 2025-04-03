use async_trait::async_trait;
use thirtyfour::By;

use crate::{information_gatherer::{information_gatherer::InformationGatherer, information_resource::InformationResource}, selenium_wrapper::unicorn_selenium_wrapper::UnicornSeleniumWrapper, solutions::problem_solver::ProblemSolver};

pub struct UnicornTestGatherer{
    information_resource: Box<dyn InformationResource>,
    selenium_wrapper: UnicornSeleniumWrapper,

}

const SELECT_CORRECT_ANSWER: &str = "Vyber správnou odpověď";
const MARK_CORRECT_ANSWERS: &str = "Označ správné odpovědi";
const FILL_IN_SENTENCE: &str = "Doplň část věty";

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

    async fn solve_test(&mut self){
        // get question count
        let question_count = self.selenium_wrapper.get_element(By::XPath("/html/body/div[1]/div/div[7]/div[4]/div/div/div/div/div[1]/div/div[2]/div/div[2]/div[2]/div/span[2]")).await.unwrap().text().await.unwrap();

        println!("{}", question_count);

        todo!("solve tests")
    }

    async fn solve_question(&mut self){
        // get question type
        let question_type_element = self.selenium_wrapper.get_element(By::XPath("/html/body/div[1]/div/div[7]/div[4]/div/div/div/div/div[2]/div[1]/div[1]/div[2]")).await.unwrap();
        let question_type = question_type_element.text().await.unwrap();
        
    }

    async fn log_solutions(&mut self){
        todo!("log solutions")
    }

    async fn return_to_main_page(&mut self){
        todo!("return to main page");
    }

    

    // pass functions
    async fn pass_select_correct_answer_question(&mut self){
        todo!("this");
    }

    async fn pass_mark_correct_answers_question(&mut self) {
        todo!("this");
    }

    async fn pass_fill_in_sentence_question(&mut self) {
        todo!("this");
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

