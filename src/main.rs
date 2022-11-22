mod reddit_model;


use std::fmt::{Debug};

use crate::reddit_model::{RedditResponseRoot};

#[derive(Debug)]
struct Image {
    url: String,
    width: i32,
    height: i32,
}

#[tokio::main]
async fn main() {
    let images = load_images_from_reddit().await;
    match images {
        Ok(_) => {
            println!("Got images: {:?}", images);
        }
        Err(err) => {
            println!("Loading error: {}", err.to_string())
        }
    }
}

async fn load_images_from_reddit() -> Result<Vec<Image>, reqwest::Error> {
    let url = "https://www.reddit.com/r/wallpapers/top/.json?sort=top&t=week&limit=20";
    let client = reqwest::Client::new();
    let resp = client.get(url)
        .header("User-Agent", "Background changer by /u/matejdro - github.com/matejdro/redditwallpaper")
        .send()
        .await?
        .json::<RedditResponseRoot>()
        .await?;

    let raw_image_datas = resp.data.children.iter().filter_map(
        |post| Some(post.data.preview.as_ref()?.images.first()?.source.clone())
    );


    return Ok(raw_image_datas.map(|source| Image { url: source.url, width: source.width, height: source.height }).collect());
}
