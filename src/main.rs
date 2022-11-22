use std::fmt::{Debug, Error, Formatter};

struct Image  {
    url: String,
    width: i32,
    height: i32,
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.url)
            .field(&self.width)
            .field(&self.height)
            .finish()
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let images = load_images_from_reddit().await?;

    println!("Got images: {:?}", images);
    return Ok(());
}

async fn load_images_from_reddit() -> Result<Vec<Image>, Error> {
    return Ok(vec![
        Image {
            url: "images.com/image1.png".to_string(),
            width: 1280,
            height: 720
        },
        Image {
            url: "images.com/image2.png".to_string(),
            width: 1920,
            height: 1080
        }
    ])
    // return Ok(Vec::new())
}
