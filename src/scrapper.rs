
use thirtyfour::{self, WebElement, By, WebDriver};

use crate::{error::{AppError, ScrapError}, handler::{EpisodeInfo, AnimeList, AnimeInfo, Anime, Server}};

pub const BASE_URL: &str = "https://gogoanime.fan";

pub async fn anime_video(episode_info: EpisodeInfo, driver: WebDriver) -> Result<Anime, AppError> {
    let mut anime: Anime = Anime::new();
    let url = format!("{}{}", BASE_URL, episode_info.link.trim());

    //navigate to gogoanime website
    driver.get(url.clone())
    .await
    .map_err(|_e| AppError::ScrapErr(ScrapError::ErrNavigateUrl(url)))
    .unwrap();

    //name
    let title_class = driver.find_element(By::ClassName("title_name"))
    .await
    .map_err(|_e| AppError::ScrapErr(ScrapError::ErrFindingClass("title_name".to_string())))
    .unwrap();

    anime.name = title_class.text()
    .await
    .map_err(|_e| AppError::ScrapErr(ScrapError::ErrTextParsing))
    .unwrap();

    //iframe link
    

    //servers
    let mut servers = Vec::new();
    let server_class = driver.find_element(By::ClassName("anime_muti_link"))
    .await
    .map_err(|_e| AppError::ScrapErr(ScrapError::ErrFindingClass("anime_muti_link".to_string())))
    .unwrap();
    let server_element_list = server_class.find_elements(By::Tag("li"))
    .await
    .map_err(|_e| AppError::ScrapErr(ScrapError::ErrFindingTag("li".to_string())))
    .unwrap();

    for i in server_element_list {
        let mut server = Server {
            name: String::new(),
            link: String::new(),
        };
        
        //server name
        let class_name = i.class_name().await.map_err(|_e| AppError::ScrapErr(ScrapError::ErrFindClassName)).unwrap().unwrap();
        server.name = class_name;

        //link
        let html = i.inner_html()
        .await
        .map_err(|_e| AppError::ScrapErr(ScrapError::InnerHtmlErr))
        .unwrap();
        let link = video_link(html).await;
        server.link = link;

        servers.push(server);
    }

    anime.server_list = servers;
    driver.quit().await.map_err(|_e| AppError::QuitDriverErr).unwrap();

    println!("{:#?}", anime);

    Ok(anime)
}

pub async fn search_keyword(keyword: String, driver: WebDriver) -> Result<Vec<AnimeList>, AppError> {
    //data
    let mut anime_list: Vec<AnimeList> = vec![];

    let keyword = keyword.replace(" ", "%20");
    let url = format!("https://gogoanime.fan//search.html?keyword={}", keyword);
     
    //navigate to gogoanime website
    driver.get(url.clone())
    .await
    .map_err(|_e| AppError::ScrapErr(ScrapError::ErrNavigateUrl(url)))
    .unwrap();
    println!("after get url");

    
/*
    //find pages
    let page_element = driver.find_element(By::ClassName("pagination-list")).await.expect("error searching for page");
    let page_list = page_element.find_elements(By::Tag("li")).await.expect("error searching for list tag");
    let pages = page_list.len();
    let mut current_page: usize = 1;
*/
    
    //go into items
    let items = driver.find_element(By::ClassName("items"))
    .await
    .map_err(|_e| AppError::ScrapErr(ScrapError::ErrFindingClass("items".to_string())))
    .unwrap();

    //go into each list
    let list = items.find_elements(By::Tag("li"))
    .await
    .map_err(|_e| AppError::ScrapErr(ScrapError::ErrFindingTag("li".to_string())))
    .unwrap();
    
    for info in list {
        let mut anime = AnimeList::default();

        //name & link
        let name_link = find_name_link(info.clone()).await.unwrap();
        anime.name(&name_link[0]);
        anime.link(&name_link[1]);

        //release date
        anime.releasedate(release_date(info.clone()).await.unwrap());

        //imgsrc
        anime.imgsrc(img_src(info).await.unwrap());

        anime_list.push(anime);
    }

    driver.quit().await.map_err(|_e| AppError::QuitDriverErr).unwrap();
    //println!("{:#?}", anime_list);

    println!("{:#?}", anime_list);

    Ok(anime_list)
}

pub async fn find_anime_info(driver: WebDriver, anime: AnimeList) -> Result<AnimeInfo, AppError> {
    let url = format!("{}{}", BASE_URL, anime.link);
    let mut anime_info: AnimeInfo = AnimeInfo::new();

    //navigate to gogoanime website
    driver.get(url.clone()).await.map_err(|_e| AppError::ScrapErr(ScrapError::ErrNavigateUrl(url))).unwrap();

    //imgsrc
    anime_info.imgsrc = anime.imgsrc;

    //name
    anime_info.name = anime.name;

    //released
    anime_info.released = anime.releasedate;

    //description, genre, status
    let anime_info_body = driver.find_element(By::ClassName("anime_info_body_bg"))
    .await
    .map_err(|_e| AppError::ScrapErr(ScrapError::ErrFindingClass("anime_info_body_bg".to_string())))
    .unwrap()
    .find_elements(By::ClassName("type"))
    .await
    .map_err(|_e| AppError::ScrapErr(ScrapError::ErrFindingClass("type".to_string())))
    .unwrap();

    for i in anime_info_body {
        let mut element_text = i.text()
        .await
        .map_err(|_e| AppError::ScrapErr(ScrapError::ErrTextParsing))
        .unwrap();
        if element_text.contains("Plot Summary:") {
            element_text.drain(..14);
            anime_info.description = element_text;
        } else if element_text.contains("Genre:") {
            element_text.drain(..7);
            anime_info.genre = element_text;
        } else if element_text.contains("Status") {
            element_text.drain(..8);
            anime_info.status = element_text;
        }
    }
    
    
    //episodes
    let episodes_element = driver.find_element(By::Id("episode_related"))
    .await
    .map_err(|_e| AppError::ScrapErr(ScrapError::ErrFindingId("episode_related".to_string())))
    .unwrap();

    let episode_list = episodes_element
    .find_elements(By::Tag("li"))
    .await
    .map_err(|_e| AppError::ScrapErr(ScrapError::ErrFindingTag("li".to_string())))
    .unwrap();

    let mut episodes = Vec::new();

    for i in episode_list {
        let mut episode = EpisodeInfo::new();
        let html = i.inner_html().await.map_err(|_e| AppError::ScrapErr(ScrapError::InnerHtmlErr)).unwrap();
        let episode_link = href_link(html).await;
        episode.link = episode_link;

        //find which episode
        let name_class = i.find_element(By::ClassName("name"))
        .await
        .map_err(|_e| AppError::ScrapErr(ScrapError::ErrFindingClass("name".to_string())))
        .unwrap();
        episode.episode = name_class.text().await.map_err(|_e| AppError::ScrapErr(ScrapError::ErrTextParsing)).unwrap();
        episodes.push(episode);
    }
    anime_info.episodes = episodes;

    println!("{:#?}", anime_info);

    driver.quit().await.map_err(|_e| AppError::QuitDriverErr).unwrap();

    Ok(anime_info)
}

async fn find_name_link(element: WebElement<'_>) -> Result<Vec<String>, AppError> {
    let mut name_link = vec![];
    let name_element = element.find_element(By::ClassName("name"))
    .await
    .map_err(|_e| AppError::ScrapErr(ScrapError::ErrFindingClass("name".to_string())))
    .unwrap();

    let name = name_element.text()
    .await
    .map_err(|_e| AppError::ScrapErr(ScrapError::ErrTextParsing))
    .unwrap();
    name_link.push(name);

    let html = name_element.inner_html()
    .await
    .map_err(|_e| AppError::ScrapErr(ScrapError::InnerHtmlErr))
    .unwrap();
    let link = href_link(html).await;
    name_link.push(link);

    Ok(name_link)
}

async fn release_date(element: WebElement<'_>) -> Result<String, AppError> {
    let date = element.find_element(By::ClassName("released"))
    .await
    .map_err(|_e| AppError::ScrapErr(ScrapError::ErrFindingClass("released".to_string())))
    .unwrap()
    .text()
    .await
    .map_err(|_e| AppError::ScrapErr(ScrapError::ErrTextParsing))
    .unwrap();
    
    Ok(date)
}

async fn img_src(element: WebElement<'_>) -> Result<String, AppError> {
    let img_element = element.find_element(By::ClassName("img"))
    .await
    .map_err(|_e| AppError::ScrapErr(ScrapError::ErrTextParsing))
    .unwrap()
    .inner_html()
    .await
    .map_err(|_e| AppError::ScrapErr(ScrapError::InnerHtmlErr))
    .unwrap();

    let img_src = imgsrclink(img_element).await;
    Ok(img_src)
}

async fn href_link(html: String) -> String {
    let mut n: usize = 0;
    let mut quote_location = vec![];
    for i in html.chars() {
        if i == '"' {
            quote_location.push(n);
        }
        n += 1;
    }
    let mut initial = html.find("href").unwrap();
    initial += 5;
    let index = quote_location.iter().position(|&r| r == initial).unwrap();
    let ending = quote_location[index + 1];

    let result: String = html.clone().drain(initial + 1 .. ending).collect();
    result
}

async fn video_link(html: String) -> String {
    let mut n: usize = 0;
    let mut quote_location = vec![];
    for i in html.chars() {
        if i == '"' {
            quote_location.push(n);
        }
        n += 1;
    }
    let mut initial = html.find("data-video").unwrap();
    initial += 11;
    let index = quote_location.iter().position(|&r| r == initial).unwrap();
    let ending = quote_location[index + 1];

    let result: String = html.clone().drain(initial + 1 .. ending).collect();
    result
}

async fn imgsrclink(html: String) -> String {
    //location of all quote marks
    let mut n: usize = 0;
    let mut quote_location = vec![];
    for i in html.chars() {
        if i == '"' {
            quote_location.push(n);
        }
        n += 1;
    }

    //location of the initial quote mark
    let mut initial = html.find("src").unwrap();
    initial += 4;

    let index = quote_location.iter().position(|&r| r == initial).unwrap();
    let ending = quote_location[index + 1];

    let result: String = html.clone().drain(initial + 1 .. ending).collect();
    result
}