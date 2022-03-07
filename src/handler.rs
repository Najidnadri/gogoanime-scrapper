use thirtyfour::{WebDriver, By};
use serde::{self, Deserialize, Serialize};

use crate::scrapper::{find_name_link, release_date, img_src};

pub struct AnimeInfo {
    
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AnimeList {
    pub name: String,
    link: String,
    pub imgsrc: String,
    pub releasedate: String,
}

impl AnimeList {
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }

    pub fn link(&mut self, link: &str) -> &mut Self {
        self.link = link.to_string();
        self
    }

    pub fn releasedate(&mut self, date: String) -> &mut Self {
        self.releasedate = date;
        self
    }

    pub fn imgsrc(&mut self, src: String) -> &mut Self {
        self.imgsrc = src;
        self 
    }
}

pub async fn search_keyword(keyword: String, driver: WebDriver) -> Result<Vec<AnimeList>, Box<dyn std::error::Error>> {
    //data
    let mut anime_list: Vec<AnimeList> = vec![];

    let keyword = keyword.replace(" ", "%20");
    let url = format!("https://gogoanime.fan//search.html?keyword={}", keyword);
     
    //navigate to gogoanime website
    driver.get(url).await.expect("error in navigating to website");
/*
    //find pages
    let page_element = driver.find_element(By::ClassName("pagination-list")).await.expect("error searching for page");
    let page_list = page_element.find_elements(By::Tag("li")).await.expect("error searching for list tag");
    let pages = page_list.len();
    let mut current_page: usize = 1;
*/
    
    //go into items
    let items = driver.find_element(By::ClassName("items")).await.expect("error searching for item list");

    //go into each list
    let list = items.find_elements(By::Tag("li")).await.expect("cannot find list tag");
    
    for info in list {
        let mut anime = AnimeList::default();

        //name & link
        let name_link = find_name_link(info.clone()).await.expect("error finding name or link");
        anime.name(&name_link[0]);
        anime.link(&name_link[1]);

        //release date
        anime.releasedate(release_date(info.clone()).await.expect("error finding release date"));

        //imgsrc
        anime.imgsrc(img_src(info).await.expect("error finding imgsrc"));

        anime_list.push(anime);
    }

    driver.quit().await.unwrap();
    println!("{:#?}", anime_list);

    //println!("{:#?}", anime_list);

    Ok(anime_list)
}
