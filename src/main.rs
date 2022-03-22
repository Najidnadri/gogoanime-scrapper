mod scrapper;
mod handler;
mod error;




use actix_web::{web, Responder, HttpServer, App, get, HttpResponse};
use error::AppError;
use scrapper::{search_keyword};
use serde::{self, Deserialize, Serialize};
use handler::{AnimeList, AnimeInfo, Anime};
use thirtyfour::{self, DesiredCapabilities, WebDriver};
use crate::{scrapper::{anime_video, find_anime_info, find_latest_episodes}};

#[derive(Debug, Deserialize, Serialize)]
enum ClientRequest {
    Search(String),
    Anime(AnimeList),
}

#[derive(Debug, Deserialize, Serialize)]
enum ServerResponse {
    AnimeSearch(Vec<AnimeList>),
    LatestEpisodes(Vec<AnimeList>),
    AnimeInfo(AnimeInfo),
    Anime(Anime),
    Err(AppError),
    None,
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error>  {
    //test for find anime info
    /* 
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_option(
        "prefs",
        serde_json::json!({
            "profile.default_content_settings": {
                "images": 2
            },
            "profile.managed_default_content_settings": {
                "images": 2
            }
        }),
    ).unwrap();
    let driver = WebDriver::new("http://localhost:3000", &caps).await.unwrap();
    let anime = AnimeList {
        name: String::from("Hakuouki"),
        link: String::from("/category/hakuouki"),
        imgsrc: String::from("https://gogocdn.net/cover/hakuouki.png"),
        releasedate: String::from("Released: 2010"),
    };
    

    let result = find_anime_info(driver, anime).await.unwrap();
    */

    /* 
    //test for finding animelist
    let keyword = "isekai".to_string();
    let mut caps = DesiredCapabilities::chrome();
    //caps.set_headless().unwrap();
    caps.add_chrome_option(
        "prefs",
        serde_json::json!({
            "profile.default_content_settings": {
                "images": 2
            },
            "profile.managed_default_content_settings": {
                "images": 2
            }
        }),
    ).unwrap();
    let driver = WebDriver::new("http://localhost:4444", &caps).await.unwrap();
    let result = search_keyword(keyword, driver).await.unwrap();
    */
    /* 
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_option(
        "prefs",
        serde_json::json!({
            "profile.default_content_settings": {
                "images": 2
            },
            "profile.managed_default_content_settings": {
                "images": 2
            }
        }),
    ).unwrap();
    let driver = WebDriver::new("http://localhost:4444", &caps).await.unwrap();
    let episode = EpisodeInfo {
        episode: "EP 2".to_string(),
        link: " /hakuouki-episode-10".to_string(),
    };

    let result = anime_video(episode, driver).await.unwrap();
    */


    //create server for front end
    println!("actix web go!");

    HttpServer::new(|| {
        App::new()
        .service(search)
        .service(video)
        .service(anime_info)
        .service(greet)
        .service(latest_episodes)
    })
    .bind(("0.0.0.0", 8080))?
    .workers(4)
    .run()
    .await
    
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    let name_s = name.into_inner();
    format!("hello, your name is {}", name_s)
}

#[get("/latest")]
async fn latest_episodes() -> impl Responder {
    println!("got it! 2");
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_option(
        "prefs",
        serde_json::json!({
            "profile.default_content_settings": {
                "images": 2
            },
            "profile.managed_default_content_settings": {
                "images": 2
            }
        }),
    ).map_err(|_e| AppError::ChromeOptionErr).unwrap();
    let driver = WebDriver::new("https://muhdnajid_NLtuzJ:ZfHis32tES2Wmddcyvzv@hub-cloud.browserstack.com/wd/hub", &caps)
    .await
    .unwrap();

    let response = find_latest_episodes(driver).await.unwrap();
    let server_response = ServerResponse::LatestEpisodes(response);
    HttpResponse::Ok().json(server_response)
}

#[get("/search/{keyword}")]
async fn search(path: web::Path<String>) -> impl Responder {
    println!("updated 2");
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_option(
        "prefs",
        serde_json::json!({
            "profile.default_content_settings": {
                "images": 2
            },
            "profile.managed_default_content_settings": {
                "images": 2
            }
        }),
    ).map_err(|_e| AppError::ChromeOptionErr).unwrap();
    let driver = WebDriver::new("https://muhdnajid_NLtuzJ:ZfHis32tES2Wmddcyvzv@hub-cloud.browserstack.com/wd/hub", &caps)
    .await
    //.map_err(|_e| AppError::CreateWebDriverErr(4442))
    .unwrap();

    let keyword = path.into_inner();
    let response = search_keyword(keyword, driver).await.unwrap();
    let server_response = ServerResponse::AnimeSearch(response);   let (tx, rx) = std::sync::mpsc::channel();
    tokio::spawn(async move {

        tx.send(server_response).unwrap();
    });
    let server_response = rx.recv().unwrap();
    HttpResponse::Ok().json(server_response)
}

#[get("/anime/{episodelink}")]
async fn video(path: web::Path<String>) -> impl Responder {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_option(
        "prefs",
        serde_json::json!({
            "profile.default_content_settings": {
                "images": 2
            },
            "profile.managed_default_content_settings": {
                "images": 2
            }
        }),
    ).map_err(|_e| AppError::ChromeOptionErr).unwrap();
    let driver = WebDriver::new("https://muhdnajid_NLtuzJ:ZfHis32tES2Wmddcyvzv@hub-cloud.browserstack.com/wd/hub", &caps)
    .await
    .map_err(|_e| AppError::CreateWebDriverErr(4442))
    .unwrap();

    let episode_link = path.into_inner();
    let response = anime_video(episode_link, driver).await.unwrap();
    let server_response = ServerResponse::Anime(response);
    HttpResponse::Ok().json(server_response)
}

#[get("/anime-info/category/{animelink}")]
async fn anime_info(path: web::Path<String>) -> impl Responder {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_option(
        "prefs",
        serde_json::json!({
            "profile.default_content_settings": {
                "images": 2
            },
            "profile.managed_default_content_settings": {
                "images": 2
            }
        }),
    ).map_err(|_e| AppError::ChromeOptionErr).unwrap();
    let driver = WebDriver::new("https://muhdnajid_NLtuzJ:ZfHis32tES2Wmddcyvzv@hub-cloud.browserstack.com/wd/hub", &caps)
    .await
    .map_err(|_e| AppError::CreateWebDriverErr(4442))
    .unwrap();

    let anime_link = path.into_inner();
    let response = find_anime_info(driver, anime_link).await.unwrap();
    let server_response = ServerResponse::AnimeInfo(response);
    HttpResponse::Ok().json(server_response)
}
