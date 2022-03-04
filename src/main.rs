mod scrapper;
mod handler;

use tokio;
use thirtyfour::{self, prelude::WebDriverResult, DesiredCapabilities, WebDriver};
use crate::handler::scrap_now;

#[tokio::main]
async fn main() -> WebDriverResult<()>  {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", &caps).await?;


    //start scrapping gogoanime
    let url = "https://gogoanime.mom/search/?keyword=ova".to_string();
    let driver = scrap_now(url, driver).await.expect("error in scraping");

    driver.quit().await.expect("error while quitting webdriver");
    Ok(())
}
