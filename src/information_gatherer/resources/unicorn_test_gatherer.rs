use std::{collections::HashMap, thread, time::Duration};

use async_trait::async_trait;
use thirtyfour::{By, WebElement};

use crate::{information_gatherer::{information_gatherer::InformationGatherer, information_resource::InformationResource}, selenium_wrapper::unicorn_selenium_wrapper::UnicornSeleniumWrapper, solutions::{solution::Solution, solution_collection::SolutionCollection}};

pub struct UnicornTestGatherer{
    information_resource: Box<dyn InformationResource>,
    selenium_wrapper: UnicornSeleniumWrapper,
    // just a hacky way to pass this value between multiple functions gets cleared often and breaks if things are called in the wrong order
    question_order: Vec<String>, 
    // solutions map < label, list of answers >
    solutions: SolutionCollection

}

const SELECT_CORRECT_ANSWER: &str = "Vyber správnou odpověď";
const MARK_CORRECT_ANSWERS: &str = "Označ správné odpovědi";
const FIND_CORRECT_ANSWERS: &str = "Najdi všechny správné odpovědi";
const FILL_IN_SENTENCE: &str = "Doplň část věty";

impl UnicornTestGatherer{
    pub async fn new(information_resource: Box<dyn InformationResource>) -> UnicornTestGatherer {
        return UnicornTestGatherer{
            selenium_wrapper: UnicornSeleniumWrapper::new(information_resource.get_resource()).await,
            information_resource: information_resource,
            question_order: Vec::new(),
            solutions: SolutionCollection::new(),
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
        let question_count = 
            self.selenium_wrapper.get_element(By::XPath("/html/body/div[1]/div/div[7]/div[4]/div/div/div/div/div[1]/div/div[2]/div/div[2]/div[2]/div/span[2]"))
            .await.unwrap()
            .text().await.unwrap()[1..]
            .parse::<i32>().unwrap();

        self.question_order.clear();
        for _ in 0..question_count{
            self.solve_question().await;
        }
        // navigate to resulsts page
        self.selenium_wrapper.click_element_from_batch(By::ClassName("uu-coursekit-rounded-button-large"), 1).await;
        thread::sleep(Duration::from_millis(200));
        self.selenium_wrapper.click_element_from_batch(By::ClassName("uu-coursekit-leave-warning-button"), 1).await;
        self.selenium_wrapper.click_element_from_batch(By::ClassName("uu-coursekit-rounded-button-large"), 0).await;
    }

    async fn solve_question(&mut self){
        // get question type
        let question_type = 
            self.selenium_wrapper.get_element(By::XPath("/html/body/div[1]/div/div[7]/div[4]/div/div/div/div/div[2]/div[1]/div[1]/div[2]"))
            .await.unwrap()
            .text().await.unwrap();

        match question_type.as_str() {
            SELECT_CORRECT_ANSWER => { self.pass_select_correct_answer_question().await }
            MARK_CORRECT_ANSWERS => { self.pass_mark_correct_answers_question().await }
            FIND_CORRECT_ANSWERS => { self.pass_mark_correct_answers_question().await }
            FILL_IN_SENTENCE => { self.pass_fill_in_sentence_question().await }
            _ => {
                panic!("test fill failed : unknown question type {}", question_type);
            }
        }

        self.question_order.push(question_type.to_owned());
    }

    async fn log_solutions(&mut self){
        let answers = self.selenium_wrapper.get_elements(By::ClassName("uu5-bricks-panel")).await.unwrap();
        
        for index in 0..self.question_order.iter().count() {
            let answer = answers.get(index).unwrap();
            let question_type = self.question_order.get(index).unwrap().to_owned();

            let is_correct = answer.id().await.unwrap().unwrap().starts_with("Correct");
            
            match question_type.as_str() {
                SELECT_CORRECT_ANSWER => { self.log_select_correct_answer_question(is_correct, &answer).await; }
                MARK_CORRECT_ANSWERS => { self.log_mark_correct_answers_question(is_correct, &answer).await; }
                FIND_CORRECT_ANSWERS => { self.log_mark_correct_answers_question(is_correct, &answer).await; }
                FILL_IN_SENTENCE => { self.log_fill_in_sentence_question(is_correct, &answer).await; }
                _ => { 
                    panic!("log failed : unknown question type {}", question_type); 
                }
            }
        }
    }

    async fn return_to_main_page(&mut self){
        self.selenium_wrapper.go_to(self.information_resource.get_resource()).await;
    }

    

    // pass functions
    async fn pass_select_correct_answer_question(&mut self){
        self.selenium_wrapper.click_element_from_batch(By::ClassName("uu-coursekit-question-t02-white-frame-answer-button"), 0).await;
        self.selenium_wrapper.click_element_from_batch(By::ClassName("uu-coursekit-rounded-button-large"),1).await;
    }

    async fn pass_mark_correct_answers_question(&mut self) {
        self.selenium_wrapper.click_element(By::XPath("/html/body/div[1]/div/div[7]/div[4]/div/div/div/div/div[2]/div[1]/div[3]/div[1]/div/div")).await;
        self.selenium_wrapper.click_element_from_batch(By::ClassName("uu-coursekit-rounded-button-large"),1).await;
    }

    async fn pass_fill_in_sentence_question(&mut self) {
        self.selenium_wrapper.click_element_from_batch(By::ClassName("uu-coursekit-question-t01-white-frame-answer-button"), 0).await;
        self.selenium_wrapper.click_element_from_batch(By::ClassName("uu-coursekit-rounded-button-large"),1).await;
    }

    // solution loggers
    async fn log_select_correct_answer_question(&mut self, is_correct: bool, element: &WebElement){
        let label = element.find(By::ClassName("uu-coursekit-dark-text")).await.unwrap().text().await.unwrap();

        let correct_answer = element
            .find(By::ClassName(if is_correct { "uu-coursekit-correct-state" } else { "uu-coursekit-result-state" })).await.unwrap()
            .parent().await.unwrap()
            .text().await.unwrap();

        println!("{} : {}", label, correct_answer);

        // log to solutions
        let mut answers: Vec<String> = Vec::new();
        answers.push(correct_answer);
        self.solutions.add_solution(Solution::new(label, answers));
    }

    async fn log_mark_correct_answers_question(&mut self, is_correct: bool, element: &WebElement){
        let label = element.find(By::ClassName("uu-coursekit-dark-text")).await.unwrap().text().await.unwrap();

        let answer_elements = element
            .find_all(By::ClassName(if is_correct { "uu-coursekit-correct-state" } else { "uu-coursekit-result-state" })).await.unwrap();
        


        let mut answers: Vec<String> = Vec::new();
        for answer_element in answer_elements {
            if answer_element.tag_name().await.unwrap() != "div" {
                continue;
            } 
            answers.push(answer_element.text().await.unwrap());
        }

        self.solutions.add_solution(Solution::new(label, answers));
    }

    async fn log_fill_in_sentence_question(&mut self, is_correct: bool, element: &WebElement){
        let label = element.find(By::ClassName("uu-coursekit-dark-text")).await.unwrap().text().await.unwrap();

        let correct_answer = element
            .find(By::ClassName(if is_correct { "uu-coursekit-correct-state" } else { "uu-coursekit-result-state" })).await.unwrap()
            .text().await.unwrap();
        
        let mut answers: Vec<String> = Vec::new();
        answers.push(correct_answer);
        self.solutions.add_solution(Solution::new(label, answers));
    }

}

#[async_trait(?Send)]
impl InformationGatherer for UnicornTestGatherer {
    async fn gather_information(&mut self) -> Option<&SolutionCollection> {
        
        // process main page    
        let button_count = self.selenium_wrapper.get_elements(By::ClassName("uu-coursekit-button-full")).await.unwrap().iter().count();
        
        // iterate tests
        for i in 0..button_count {
            self.start_test(i).await;
            self.solve_test().await;
            self.log_solutions().await;
            self.return_to_main_page().await;
        }


        return Some(&self.solutions);
    }


}

