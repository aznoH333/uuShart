use core::time;
use std::{io::stdin, thread, time::Duration};

use thirtyfour::{error::WebDriverError, By, DesiredCapabilities, WebDriver, WebElement};

const LOGIN_URL: &str = "https://unicornuniversity.net/en/";
const LOGIN_BUTTON_XPATH: &str = "/html/body/div[1]/div[1]/div[2]/span/button";

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

        thread::sleep(time::Duration::from_secs(4));

        return this;
    }

    async fn login(&mut self){
        self.driver.goto(LOGIN_URL).await.unwrap();
        
        
        let log_in_button = Self::retry(async ||{return self.driver.find(By::XPath(LOGIN_BUTTON_XPATH)).await;}).await.unwrap();
        log_in_button.click().await.unwrap();        
        println!("Login requered!");
        
        let mut a = String::new();
        stdin().read_line(&mut a).expect("no input");


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
        /*
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
        driver.quit().await.unwrap();*/
    }

    async fn retry(f: impl AsyncFn()-> Result<WebElement, WebDriverError>) -> Result<WebElement, WebDriverError>{
        for _ in 0..40 {
            let result = f().await;
            
            if result.is_ok(){
                return result;
            }

            thread::sleep(Duration::from_millis(2000));
        }
        return f().await;
    }
    
}