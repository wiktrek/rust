#[macro_use]
extern crate rocket;
use rocket::State;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::http::uri::Origin;
use rocket::serde::json::{json, Value};
use reqwest;
use reqwest::Client;
const SEARCH_PREFIX: Origin<'static> = uri!("/search");
const REPO: &str = "wiktrek/rust";
#[get("/")]
fn index() -> Redirect {
    let msg:Option<&str> = None;
    Redirect::to(uri!(SEARCH_PREFIX, search(msg)))
}
fn make_tauri_response(github_release: &Value) -> Option<Value> {
let mut response = json!({
    "version": github_release["tag_name"].as_str().ok_or("tag_name not found")?,
    "notes": remove_suffix(&github_release["body"].as_str()?, "See the assets to download this version and install.").trim_end_mathes(['\r', '\n', '']),
})

}
async fn get_latest_release(client: &Client,repo: &str)  -> Result<Value,reqwest::Error> {
    let url = format!("http://api.github.com/repos/{repo}/releases/latest");
    let response = client.get(&url).send().await?;
    let github_release = response.json::<Value>().await?;
    make_tauri_response(&github_release).or(json!({}));
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