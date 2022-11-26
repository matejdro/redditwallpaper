use std::fmt::Debug;
use std::iter::FilterMap;
use rand::Rng;

use crate::reddit_model::{Data2, Preview, RedditImage, RedditResponseRoot, Source};

mod reddit_model;


#[derive(Debug, Clone)]
struct Image {
    url: String,
    width: i32,
    height: i32,
}

#[tokio::main]
async fn main() {
    let images_res = load_images_from_reddit().await;
    match images_res {
        Ok(images) => {
            download_random_image(images)
        }
        Err(err) => {
            println!("Loading error: {}", err)
        }
    }
}

fn download_random_image(images: Vec<Image>) {
    let suitable_images: Vec<Image> = images.into_iter().filter(|i| {
        let aspect_ratio = i.width as f32 / i.height as f32;
        i.width >= 2560 && (aspect_ratio - 1.77).abs() < 0.01
    }).collect();

    if suitable_images.is_empty() {
        println!("No suitable image found!");
        return;
    }

    let random_index = rand::thread_rng().gen_range(0..suitable_images.len());

    let image = suitable_images.into_iter().nth(random_index).unwrap();
    let mut escaped_url: String = String::new();
    html_escape::decode_html_entities_to_string(image.url, &mut escaped_url);

    println!("Selected {}", escaped_url)
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

    let raw_image_datas = resp.data.children.into_iter().filter_map(
        |post| Some(post.data.preview?.images.into_iter().next()?.source)
    );

    return Ok(raw_image_datas.map(|source| Image { url: source.url, width: source.width, height: source.height }).collect());
}
