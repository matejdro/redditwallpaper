use std::error::Error;
use std::fmt::Debug;
use std::io::Cursor;

use rand::Rng;

use crate::reddit_model::{RedditResponseRoot};

mod reddit_model;


#[derive(Debug, Clone)]
struct Image {
    source_thread: String,
    url: String,
    width: i32,
    height: i32,
}

#[tokio::main]
async fn main() {
    let images_res = load_images_from_reddit().await;
    match images_res {
        Ok(images) => {
            download_random_image(images).await
        }
        Err(err) => {
            println!("Loading error: {}", err)
        }
    }
}

async fn download_random_image(images: Vec<Image>) {
    let suitable_images: Vec<Image> = images.into_iter().filter(|i| {
        let aspect_ratio = i.width as f32 / i.height as f32;
        i.width > 1800 && (aspect_ratio - 1.77).abs() < 0.2
    }).collect();

    if suitable_images.is_empty() {
        println!("No suitable image found!");
        return;
    }

    let random_index = rand::thread_rng().gen_range(0..suitable_images.len());

    let image = suitable_images.into_iter().nth(random_index).unwrap();
    let mut escaped_url: String = String::new();
    html_escape::decode_html_entities_to_string(image.url, &mut escaped_url);

    println!("Selected {} from https://www.reddit.com{}", escaped_url, image.source_thread);

    let result = fetch_url(escaped_url, String::from("background.webp")).await;

    match result {
        Ok(_) => {}
        Err(e) => {
            println!("Download error: {}", (*e).to_string())
        }
    }
}

async fn fetch_url(url: String, file_name: String) -> Result<(), Box<dyn Error>> {
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

async fn load_images_from_reddit() -> Result<Vec<Image>, reqwest::Error> {
    let url = "https://www.reddit.com/r/wallpapers/top/.json?sort=top&t=month&limit=50";
    let client = reqwest::Client::new();

    let resp = client.get(url)
        .header("User-Agent", "Background changer by /u/matejdro - github.com/matejdro/redditwallpaper")
        .send()
        .await?
        .json::<RedditResponseRoot>()
        .await?;

    let raw_image_datas = resp.data.children.into_iter().filter_map(
        |post| {
            let source = post.data.preview?.images.into_iter().next()?.source;
            return Some(Image { source_thread: post.data.permalink, url: source.url, width: source.width, height: source.height });
        }
    );

    return Ok(raw_image_datas.collect());
}
