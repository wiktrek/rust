#[macro_use]
extern crate rocket;

use rocket::State;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::http::uri::Origin;
use rocket::serde::json::{json, Value};
use reqwest::{self, Response};
use reqwest::Client;
use std::collections::HashMap;
const SEARCH_PREFIX: Origin<'static> = uri!("/tauri-releases");
const REPO: &str = "elibroftw/google-keep-desktop-app";
#[get("/")]
fn index() -> Redirect {
    let msg:Option<&str> = None;
    Redirect::to(uri!(SEARCH_PREFIX, search(msg)))
}
fn remove_suffix<'a>(s: &'a str, suffix: &str) -> &'a str {
    match s.strip_suffix(suffix) {
        Some(s) => s,
        None => s,
    }
}

async fn text_request(client: &State<Client>, url: &str) -> Result<String, reqwest::Error> {
    client.get(url).send().await?.text().await
}
async fn create_tauri_response(client: &State<Client>, github_release: &Value) -> Option<Value> {
    let platforms_available: HashMap<&str, Vec<&str>> = HashMap::from([
        ("amd64.AppImage.tar.gz", vec!["linux-x86_64"]),
        ("app.tar.gz", vec!["darwin-x86_64", "darwin-aarch64"]),
        ("x64_en-US.msi.zip", vec!["windows-x86_64"]),
    ]);

    let mut response = json!({
        "version": github_release["tag_name"].as_str()?,
        "notes": remove_suffix(&github_release["body"].as_str()?, "See the assets to download this version and install.").trim_end_matches(['\r', '\n', ' ']),
        "pub_date": github_release["published_at"].as_str()?,
        "platforms": {}
    });

    let response_platforms = response["platforms"].as_object_mut()?;
    for asset in github_release["assets"].as_array()?.iter() {
        let asset = asset.as_object()?;
        let asset_name = asset["name"].as_str()?;
        let browser_download_url = asset["browser_download_url"].as_str()?;
        for (extension, os_archs) in platforms_available.iter() {
            if asset_name.ends_with(extension) {
                for os_arch in os_archs.iter() {
                    if !response_platforms.contains_key(*os_arch) {
                        response_platforms.insert(os_arch.to_string(), json!({}));
                    }
                    response_platforms[*os_arch].as_object_mut()?.insert("url".to_string(), Value::String(browser_download_url.to_string()));
                }
            } else if asset_name.ends_with(&format!("{extension}.sig")) {
                let signature = match text_request(client, browser_download_url).await {
                    Ok(s) => s,
                    _ => String::new()
                };
                for os_arch in os_archs.iter() {
                    if !response_platforms.contains_key(*os_arch) {
                        response_platforms.insert(os_arch.to_string(), json!({}));
                    }
                    response_platforms[*os_arch].as_object_mut()?.insert("signature".to_string(), Value::String(signature.clone()));
                }
            }
        }
    }
    Some(response)
}
async fn get_latest_release(client: &Client,repo: &str)  -> Result<Value,reqwest::Error> {
    let url: String = format!("http://api.github.com/repos/{repo}/releases/latest");
    let response: Response = client.get(&url).send().await?;
    let github_release: Value = response.json::<Value>().await?;
    create_tauri_response(client, &github_release).await.ok_or(json!({})).or_else(|e| Ok(e));
    Ok(github_release)
}
#[get("/search?<msg>")]
async fn search(msg: Option<&str>, client: &State<Client>) -> Result<Value, Status> {

if let Some(msg) = msg {
    println!("repo");
    return Err(Status::NoContent);
}
get_latest_release(client, REPO).await.or(Err(Status::NoContent))

}
#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(
        reqwest::Client::builder()
        .user_agent("reqwest")
        .build()
        .unwrap()
    )
    .mount("/",routes![index])
    .mount(SEARCH_PREFIX,routes![search])
}