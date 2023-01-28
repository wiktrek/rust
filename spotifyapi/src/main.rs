use reqwest::{self, header};
use reqwest::header::{ACCEPT,AUTHORIZATION,CONTENT_TYPE};
use serde::{Deserialize,Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct ExternalUrls{
    spotify: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Artist{
    name: String,
    external_url: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
struct Album{
    name: String,
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Track{
name: String,
href: String,
popularity: u32,
album: Album,
external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
struct APIResponse{
    tracks: Items<Track>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Items<T> {
    items: Vec<T>
}
fn print_tracks(tracks: Vec<&Track>) {
for 
}
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let search_query = &args[1];
    let auth_token = &args[2];
    let url = format!(
      "https://api.spotify.com/search?q={query}&type=track,artist",
      query = search_query
    );
    let client = reqwest::Client::new();
    let response = client
    .get(url)
    .header(AUTHORIZATION, format!("Bearer {}", auth_token))
    .header(CONTENT_TYPE, "application/json")
    .header(ACCEPT, "application/jso")
    .send()
    .await
    .unwrap();
    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<APIResponse>().await {
                Ok(parsed) => print_tracks(parsed.tracks.items.iter().collect()),
                Err(_) => println!("the response didn't match the struct / shape")
            }
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");
        }
    }

}