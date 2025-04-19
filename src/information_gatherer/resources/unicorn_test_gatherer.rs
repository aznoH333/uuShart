use std::{collections::HashMap, thread, time::Duration};

use async_trait::async_trait;
use thirtyfour::{By, WebElement};

use crate::{information_gatherer::{information_gatherer::InformationGatherer, information_resource::InformationResource}, selenium_wrapper::unicorn_selenium_wrapper::UnicornSeleniumWrapper, solutions::{solution::Solution, solution_collection::SolutionCollection}};

use super::unicorn_question_type::UnicornQuestionType;

pub struct UnicornTestGatherer{
    information_resource: Box<dyn InformationResource>,
    selenium_wrapper: UnicornSeleniumWrapper,
    // just a hacky way to pass this value between multiple functions gets cleared often and breaks if things are called in the wrong order
    question_order: Vec<String>, 
    // solutions map < label, list of answers >
    solutions: SolutionCollection,
    gathered_information: i32,

}

impl UnicornTestGatherer{
    pub async fn new(information_resource: Box<dyn InformationResource>) -> UnicornTestGatherer {
        return UnicornTestGatherer{
            selenium_wrapper: UnicornSeleniumWrapper::new(information_resource.get_resource()).await,
            information_resource: information_resource,
            question_order: Vec::new(),
            solutions: SolutionCollection::new(),
            gathered_information: 0
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
        let question_type = self.determine_question_type().await;

        match question_type {
            UnicornQuestionType::SINGLE_CHOICE => { self.pass_select_correct_answer_question().await } // TODO : rewrite theese solve/log questions to be an enum function
            UnicornQuestionType::MULTI_SELECT => { self.pass_mark_correct_answers_question().await }
            UnicornQuestionType::FILL_SENTENCE => { self.pass_fill_in_sentence_question().await }
            UnicornQuestionType::YES_NO => { self.pass_yes_no_question().await }
            _ => {
                panic!("test fill failed : unknown question type");
            }
        }

        self.question_order.push(question_type.to_string().to_owned());
    }

    async fn log_solutions(&mut self){
        let answers = self.selenium_wrapper.get_elements(By::ClassName("uu5-bricks-panel")).await.unwrap();
        
        for index in 0..self.question_order.iter().count() {
            let answer = answers.get(index).unwrap();
            let question_type = self.question_order.get(index).unwrap().to_owned();

            let is_correct = answer.id().await.unwrap().unwrap().starts_with("Correct"); // TODO : this whole question type thingy sucks and i hate it
            
            match UnicornQuestionType::from_string(&question_type) {
                UnicornQuestionType::SINGLE_CHOICE => { self.log_select_correct_answer_question(is_correct, &answer).await; }
                UnicornQuestionType::MULTI_SELECT => { self.log_mark_correct_answers_question(is_correct, &answer).await; }
                UnicornQuestionType::FILL_SENTENCE => { self.log_fill_in_sentence_question(is_correct, &answer).await; }
                UnicornQuestionType::YES_NO => { self.log_yes_no_question(is_correct, &answer).await; }
                _ => { 
                    panic!("log failed : unknown question type {}", question_type); 
                }
            }
        }
    }

    async fn return_to_main_page(&mut self){
        self.selenium_wrapper.go_to(self.information_resource.get_resource()).await;
    }

    async fn determine_question_type(&mut self) -> UnicornQuestionType{
        let is_single_select = self.selenium_wrapper.check_if_element_exists_and_is_clickable(By::ClassName("uu-coursekit-question-t02-white-frame-answer-button"), 10).await;
        let is_mutli_choice = self.selenium_wrapper.check_if_element_exists_and_is_clickable(By::ClassName("uu-coursekit-question-t03-white-frame-answer-button"), 10).await;
        let is_fill_in_sentence = self.selenium_wrapper.check_if_element_exists_and_is_clickable(By::ClassName("uu-coursekit-question-t01-white-frame-answer-button"), 10).await;
        let is_yes_no = self.selenium_wrapper.check_if_element_exists(By::ClassName("uu-coursekit-question-t10-white-frame-answer-button"), 10).await;

        println!("result {} {} {}", is_single_select.to_string(), is_mutli_choice.to_string(), is_fill_in_sentence.to_string());
        if is_single_select as u8 + is_mutli_choice as u8 + is_fill_in_sentence as u8 > 1 {
            panic!("indecisive question type {} {} {}", is_single_select.to_string(), is_mutli_choice.to_string(), is_fill_in_sentence.to_string());
        }

        if is_single_select {
            return UnicornQuestionType::SINGLE_CHOICE;
        }else if is_mutli_choice {
            return UnicornQuestionType::MULTI_SELECT;
        }else if is_fill_in_sentence{
            return UnicornQuestionType::FILL_SENTENCE;
        }else if is_yes_no{
            return UnicornQuestionType::YES_NO;
        }

        todo!("beams");
    }
    

    // pass functions
    async fn pass_select_correct_answer_question(&mut self){
        self.selenium_wrapper.click_element_from_batch(By::ClassName("uu-coursekit-question-t02-white-frame-answer-button"), 0).await;
        self.selenium_wrapper.click_element_from_batch(By::ClassName("uu-coursekit-rounded-button-large"),1).await;
    }

    async fn pass_mark_correct_answers_question(&mut self) {
        self.selenium_wrapper.click_element(By::ClassName("uu-coursekit-question-t03-white-frame-answer-button")).await;
        self.selenium_wrapper.click_element_from_batch(By::ClassName("uu-coursekit-rounded-button-large"),1).await;
    }

    async fn pass_fill_in_sentence_question(&mut self) {
        self.selenium_wrapper.click_element_from_batch(By::ClassName("uu-coursekit-question-t01-white-frame-answer-button"), 0).await;
        self.selenium_wrapper.click_element_from_batch(By::ClassName("uu-coursekit-rounded-button-large"),1).await;
    }

    async fn pass_yes_no_question(&mut self) {
        self.selenium_wrapper.click_element_from_batch(By::ClassName("uu-coursekit-question-t10-white-frame-answer-button"), 0).await;
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
        if self.solutions.add_solution(Solution::new(label, answers)) { self.gathered_information += 1; }
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

        if self.solutions.add_solution(Solution::new(label, answers)) { self.gathered_information += 1; }
    }

    async fn log_fill_in_sentence_question(&mut self, is_correct: bool, element: &WebElement){
        let label = element.find(By::ClassName("uu-coursekit-dark-text")).await.unwrap().text().await.unwrap();

        let correct_answer = element
            .find(By::ClassName(if is_correct { "uu-coursekit-correct-state" } else { "uu-coursekit-result-state" })).await.unwrap()
            .text().await.unwrap();
        
        let mut answers: Vec<String> = Vec::new();
        answers.push(correct_answer);
        if self.solutions.add_solution(Solution::new(label, answers)) { self.gathered_information += 1; }
    }

    async fn log_yes_no_question(&mut self, is_correct: bool, element: &WebElement) {
        let label = element.find(By::ClassName("uu-coursekit-dark-text")).await.unwrap().text().await.unwrap();

        let correct_answer = element
            .find(By::ClassName(if is_correct { "uu-coursekit-question-t10-white-frame-answer-correct-answer-button-correct" } else { "uu-coursekit-question-t10-white-frame-result-answer-button-correct" })).await.unwrap()
            .text().await.unwrap();
        
        let mut answers: Vec<String> = Vec::new();
        answers.push(correct_answer);
        if self.solutions.add_solution(Solution::new(label, answers)) { self.gathered_information += 1; }
    }

}

#[async_trait(?Send)]
impl InformationGatherer for UnicornTestGatherer {
    async fn gather_information(&mut self) -> Option<&SolutionCollection> {
        
        // process main page    
        let button_count = self.selenium_wrapper.get_elements(By::ClassName("uu-coursekit-button-full")).await.unwrap().iter().count();
        
        // iterate tests
        for i in 0..button_count {
            // nemame technologii na do while to je shart
            
            loop {
                self.gathered_information = 0;
                self.start_test(i).await;
                self.solve_test().await;
                self.log_solutions().await;
                self.return_to_main_page().await;

                if self.gathered_information < 2{
                    break; // tohle je mega fart.
                }
            }
            
        }


        return Some(&self.solutions);
    }


}

