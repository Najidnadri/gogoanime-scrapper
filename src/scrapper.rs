
use thirtyfour::{self, WebElement, By};

pub async fn find_name_link(element: WebElement<'_>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut name_link = vec![];
    let name_element = element.find_element(By::ClassName("name")).await.expect("cannot find name in list");
    let name = name_element.text().await.expect("error turning into text");
    name_link.push(name);

    let html = name_element.inner_html().await.expect("inner html error");
    let link = href_link(html);
    name_link.push(link);

    Ok(name_link)
}

pub async fn release_date(element: WebElement<'_>) -> Result<String, Box<dyn std::error::Error>> {
    let date = element.find_element(By::ClassName("released")).await.expect("cannot find released class");
    let released_date = date.text().await.expect("error parsing date to string");
    Ok(released_date)
}

pub async fn img_src(element: WebElement<'_>) -> Result<String, Box<dyn std::error::Error>> {
    let img_element = element.find_element(By::ClassName("img")).await.expect("error searching for img class");
    let img_html = img_element.inner_html().await?;
    let src = imgsrclink(img_html);
    Ok(src)
}

pub fn href_link(html: String) -> String {
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

fn imgsrclink(html: String) -> String {
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