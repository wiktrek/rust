#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::http::uri::Origin;
use rocket::serde::json::{json, Value};
use reqwest;
use reqwest::Client;
const SEARCH_PREFIX: Origin<'static> = uri!("/search");
#[get("/")]
fn index() -> Redirect {
    let msg:Option<&str> = None;
    Redirect::to(uri!(SEARCH_PREFIX, search(msg)))
}
fn get_latest_release(client: &Client,repo: &str)  -> Result<Value,reqwest::Error> {
    let url = format!("http://api.github.com/repos/{repo}/releases/latest");
    let response = client.get(&url).send().await?;
    let github_release = response.json::<Value>().await?;
    github_release
}
#[get("/search?<msg>")]
fn search(msg: Option<&str>) -> Result<Value, Status> {

if let Some(msg) = msg {
    println!("{msg}");
    return Err(Status::NoContent);
}
get_latest_release(, ).or_else(Err(Status::NoContent))

}
#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/",routes![index])
    .mount(SEARCH_PREFIX,routes![search])
}