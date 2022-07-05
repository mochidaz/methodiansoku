use serde::{Serialize, Deserialize};
use reqwest;

pub struct Danbooru<'a> {
    tag: Option<&'a String>,
}

#[derive(Deserialize, Debug)]
pub struct BooruData {
    pub id: usize,
    pub large_file_url: String,
    pub tag_string_artist: String,
    pub source: String,
}

impl<'a> Danbooru<'a> {

    pub fn new(tag: Option<&'a String>) -> Self {
        Self {
            tag,
        }
    }

    pub async fn random_picture(&self) -> String {
        let client = reqwest::Client::new();
        let req = client.get(format!("{}/posts.json", "https://safebooru.donmai.us"))
            .query(&[("tags", format!("{} rating:{} random:1", match &self.tag { Some(t) => t, None => "" }, "general")), ("limit", 1.to_string())])
            .send()
            .await.unwrap();

        let response = match req.json::<Vec<BooruData>>().await {
            Ok(v) => v,
            Err(_) => return String::from("Cannot process tag! Tag limit is only one tag per request!"),
        };

        let data = match response.first() {
            Some(v) => v,
            None => return String::from(format!("Cannot process tag! No matching posts found for the following tag: {}", &self.tag.unwrap()))
        };

        let post = format!("{}/posts/{}", "https://safebooru.donmai.us", data.id);

        let image_url = &data.large_file_url;

        let artist = &data.tag_string_artist;

        let source = &data.source;

        format!(
            "Artist: {}\nPost: {}"
            , artist, post
        )
    }
}