use std::{thread, time::Duration};

use async_trait::async_trait;
use thirtyfour::{By, DesiredCapabilities, WebDriver, WebElement};

use crate::{information_gatherer::{information_gatherer::InformationGatherer, information_resource::InformationResource, information_resource_type::InformationResourceType}, selenium_wrapper::unicorn_selenium_wrapper::UnicornSeleniumWrapper, solutions::solution_collection::SolutionCollection};

pub struct UnicornInformationGatherer{
    information_resource: Box<dyn InformationResource>,
    selenium_wrapper: UnicornSeleniumWrapper
}

impl UnicornInformationGatherer{
    pub async fn new(information_resource: Box<dyn InformationResource>) -> UnicornInformationGatherer {
        return UnicornInformationGatherer { 
            selenium_wrapper: UnicornSeleniumWrapper::new(information_resource.get_resource()).await,
            information_resource: information_resource,
        };
    }
}


#[async_trait(?Send)]
impl InformationGatherer for UnicornInformationGatherer{
    async fn gather_information(&mut self) -> Option<SolutionCollection> {
        if *self.information_resource.get_resource_type() != InformationResourceType::UuKit {
            return None;
        }
        
        let mut solver = SolutionCollection{};

        let big_links = self.selenium_wrapper.get_big_links().await.unwrap();

        for link in big_links{
            self.go_through_course(link).await;
        }

        return Some(solver);


    }
}

impl UnicornInformationGatherer{
    async fn go_through_course(&mut self, big_link: WebElement){
        println!("going through course {}", big_link.text().await.unwrap());

        big_link.click().await.unwrap();
        let max_count = self.selenium_wrapper.get_elements(By::ClassName("uu-coursekit-primary-button")).await.unwrap().iter().count();
        for i in 0..max_count{
            let buttons = self.selenium_wrapper.get_elements(By::ClassName("uu-coursekit-primary-button")).await.unwrap();
            
            for link_index in (0..buttons.iter().count()-2).step_by(2){
                
                buttons.get((link_index * 2) + 1 ).unwrap().click().await.unwrap();

                thread::sleep(Duration::from_millis(3000));
                // click start button
                let bs = self.selenium_wrapper.get_elements(By::ClassName("uu-coursekit-rounded-button")).await.unwrap();
                bs.get(1).unwrap().click().await.unwrap();
                thread::sleep(Duration::from_millis(1000));

                while !self.is_section_complete().await {
                    self.process_section().await;
                }
    
                self.selenium_wrapper.get_element(By::XPath("//*[@id=\"50b68c62a779747838565bbabf60b134-inner\"]")).await.unwrap().click().await.unwrap();
                
            }            


        }
    }

    async fn is_section_complete(&mut self) -> bool{
        return self.selenium_wrapper.check_if_element_exists(By::XPath("/html/body/div[1]/div/div[7]/div[4]/div/div/div/div/div/div[1]/div[2]/div[4]"), 200).await;
    }

    async fn process_section(&mut self){
        // find buttons
        let mut buttons = self.selenium_wrapper.get_elements(By::ClassName("uu5-bricks-button-filled")).await.unwrap();

        let skip_index = find_element_index_by_text(&buttons, "Přeskočit").await;

        // try to skip
        
        if skip_index != -1{
            let skip_button = buttons.get(skip_index as usize);
            skip_button.unwrap().click().await.unwrap();
            return; // skipped
        }

        // SINGLE CHOICE QUESTION

        // get answers
        let confirm_index = find_element_index_by_text(&buttons, "Potvrdit").await;
        let leave_index = find_element_index_by_text(&buttons, "Opustit").await;

        debug_print(&buttons).await;
        let mut clicked_label = String::new();
        for i in 0..buttons.iter().count(){
            if i as i32 == confirm_index || i as i32 == leave_index {
                continue;
            }
            
            // click random answer
            let button = buttons.get(i as usize).unwrap();

            if button.text().await.unwrap().is_empty(){
                continue;
            }
            clicked_label.push_str(&button.text().await.unwrap());
            button.click().await.unwrap();
            break;
        }

        buttons.get(confirm_index as usize).unwrap().click().await.unwrap();
        
        // check if answer was correct
        // refresh buttons
        buttons = self.selenium_wrapper.get_elements(By::ClassName("uu5-bricks-button-filled")).await.unwrap();

        // show solution
        let show_solution_index = find_element_index_by_text(&buttons, "Ukaž řešení").await;
        click_button_if_exists(&buttons, show_solution_index).await;


        // TODO : multi choice questions
        // TODO : match questions
        // TODO : order questions
        
        println!("doin stuff :)");
    }
}

async fn debug_print(elements: &Vec<WebElement>){
    for e in elements{
        println!("{}", e.text().await.unwrap())
    }
}

async fn find_element_index_by_text(elements: &Vec<WebElement>, text: &str) -> i32 {
    for a in 0..elements.iter().count(){
        if (elements.get(a).unwrap().text().await.unwrap().eq(text)){
            return a as i32;
        }
    }

    return -1;
}

async fn click_button_if_exists(buttons: &Vec<WebElement>, index: i32){
    if index != -1 {
        buttons.get(index as usize).unwrap().click().await.unwrap();
    }
}