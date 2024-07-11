use reqwest::blocking::get;
use reqwest::Url;
use scraper::{Html, Selector};
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let base_url = Url::parse("https://www.jujutsuyugen.com/blog")?;

    println!("Fetching images from: {}", base_url);

    let body = get(base_url.as_str())?.text()?;
    let document = Html::parse_document(&body);
    let img_selector = Selector::parse("a > div > div > img").unwrap();

    let dir_name = "./images";

    create_dir_all(dir_name)?;

    println!("Downloading images...");

    for img in document.select(&img_selector) {
        if let Some(img_src) = img.value().attr("src") {
            let img_url = base_url.join(img_src)?;
            let mut img_response = get(img_url.as_str())?;

            let mut img_bytes = Vec::new();
            img_response.copy_to(&mut img_bytes)?;

            let img_name = img_src.split('/').last().unwrap_or("image.jpg");
            let img_path = format!("{}/{}", dir_name, img_name);
            let mut img_file = File::create(img_path)?;

            img_file.write_all(&img_bytes)?;

            println!("Downloaded: {}", img_name);
        }
    }

    println!("Download complete!");

    Ok(())
}
