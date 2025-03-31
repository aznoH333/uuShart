use core::time;
use std::{io::stdin, thread, time::Duration};

use thirtyfour::{error::WebDriverError, By, DesiredCapabilities, WebDriver, WebElement};

const LOGIN_URL: &str = "https://unicornuniversity.net/en/";
const LOGIN_BUTTON_XPATH: &str = "/html/body/div[1]/div[1]/div[2]/span/button";
const BIG_LINK_CLASS: &str = "uu-coursekit-course-menu-block-topic-big-link"; 

pub struct UnicornSeleniumWrapper{
    current_url: String,
    driver: WebDriver,
}

impl UnicornSeleniumWrapper{
    pub async fn new(site_url: &String) -> UnicornSeleniumWrapper{
        let mut this = UnicornSeleniumWrapper {  
            current_url: site_url.to_owned(),
            driver: Self::setup_driver().await,
        };

        this.login().await;
        this.go_to_course_kit().await;

        return this;
    }

    async fn login(&mut self){
        self.driver.goto(LOGIN_URL).await.unwrap();
        
        
        let log_in_button = Self::retry::<WebElement>(async ||{return self.driver.find(By::XPath(LOGIN_BUTTON_XPATH)).await;}, 40).await;
        log_in_button.unwrap().click().await.unwrap();        
        println!("Login requered!");
        
        let mut a = String::new();
        stdin().read_line(&mut a).expect("no input");


    }

    pub async fn get_big_links(&mut self) -> Option<Vec<WebElement>> {
        return self.get_elements(By::ClassName(BIG_LINK_CLASS)).await;
    }

    pub async fn get_element(&mut self, by: By) -> Option<WebElement>{
        thread::sleep(Duration::from_millis(200));
        return Self::retry::<WebElement>(async ||{return self.driver.find(by.clone()).await;}, 100).await
    }

    pub async fn get_elements(&mut self, by: By) -> Option<Vec<WebElement>> {
        thread::sleep(Duration::from_millis(200));
        Self::retry::<WebElement>(async ||{return self.driver.find(by.clone()).await;}, 100).await.unwrap();
        return Self::retry::<Vec<WebElement>>(async ||{return self.driver.find_all(by.to_owned()).await;}, 1).await
    }

    pub async fn check_if_element_exists(&mut self, by: By, wait_ms: u64) -> bool{
        thread::sleep(Duration::from_millis(wait_ms));
        return Self::retry::<WebElement>(async ||{return self.driver.find(by.clone()).await;}, 5).await.is_some();
    }


    async fn go_to_course_kit(&mut self){
        self.driver.goto(self.current_url.clone()).await.unwrap();
    }

    async fn setup_driver() -> WebDriver{
        let mut caps = DesiredCapabilities::firefox();
        caps.add_arg("-profile").unwrap();
        caps.add_arg("/home/aznoh/.mozilla/firefox/fw83rcjj.selenium").unwrap();

        let driver = WebDriver::new("http://localhost:9515", caps).await.unwrap();
        
        

        driver.goto(LOGIN_URL).await.unwrap();

        return driver;
    }

    async fn retry<T>(f: impl AsyncFn()-> Result<T, WebDriverError>, retry_count: i32) -> Option<T>{
        for _ in 0..retry_count {
            let result = f().await;
            
            if result.is_ok(){
                return Some(result.unwrap());
            }

            thread::sleep(Duration::from_millis(200));
        }
        println!("Failed to find element after 40 retries");
        return None;
    }

    
    
}