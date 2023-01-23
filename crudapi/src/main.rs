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
async fn get_latest_release(client: &Client,repo: &str)  -> Result<Value,reqwest::Error> {
    let url = format!("http://api.github.com/repos/{repo}/releases/latest");
    let response = client.get(&url).send().await?;
    let github_release = response.json::<Value>().await?;
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