use reqwest::*;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Uuid {
    name: String,
    id: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("No API KEY");
    let uuid: Uuid = serde_json::from_str((get("https://api.mojang.com/users/profiles/minecraft/wiktrek").await?.text().await?).as_str()).expect("Error parsing JSON");
    let client = Client::new();
    let body = client.get(format!("https://api.hypixel.net/v2/skyblock/profile?profile={}", uuid.id))
        .header("API-Key", format!("{}", api_key)).send()
        .await?
        .text()
        .await?;
    
    println!("body = {body:?} {}", uuid.id);
    Ok(())
}

