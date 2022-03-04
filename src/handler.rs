use thirtyfour::{WebDriver, By};

use crate::scrapper::{find_name_link, release_date, img_src};

#[derive(Debug, Default)]
pub struct Anime {
    name: String,
    link: String,
    imgsrc: String,
    releasedate: String,
}

impl Anime {
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

pub async fn scrap_now(url: String, driver: WebDriver) -> Result<WebDriver, Box<dyn std::error::Error>> {
    let mut anime_list: Vec<Anime> = vec![];
     
    //navigate to gogoanime website
    driver.get(url).await.expect("error in navigating to website");

    //go into items
    let items = driver.find_element(By::ClassName("items")).await.expect("error searching for item list");

    //go into each list
    let list = items.find_elements(By::Tag("li")).await.expect("cannot find list tag");
    
    for info in list {
        let mut anime = Anime::default();

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

    println!("{:#?}", anime_list);

    Ok(driver)
}
