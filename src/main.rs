mod scrapper;
mod handler;

use handler::AnimeList;
use tokio;
use thirtyfour::{self, prelude::WebDriverResult, DesiredCapabilities, WebDriver};
use crate::handler::scrap_now;

enum ClientRequest {
    Search(String),
    Anime(String),
}

enum ServerResponse {
    AnimeList(Vec<AnimeList>),
    AnimeInfo(Anime)
}

struct Anime {
    name: String,
    release_date: String,
    img_src: String,
    description: String,
    episodes: usize,
    status: String,
    genre: String,
}

#[tokio::main]
async fn main() -> WebDriverResult<()>  {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", &caps).await?;

    let demo_search = "ova".to_string();

    //start scrapping gogoanime
    let url = "https://gogoanime.fan//search.html?keyword=ova".to_string();
    let driver = scrap_now(url, driver).await.expect("error in scraping");

    driver.quit().await.expect("error while quitting webdriver");
    Ok(())
}
