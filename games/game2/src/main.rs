use dotenv;
use std::io;
use rand::Rng;
use sqlx::postgres::{PgPoolOptions, PgRow};


fn main() -> io::Result<()> {
    println!("Hello, world!");
    read_line();
    db();
Ok(())
}
fn read_line() -> String{
    let mut user_input = String::new();
    println!("write 1 for adding a question, write 2 for anwsering questions");
    io::stdin()
    .read_line(&mut user_input).expect("Couldn’t read from stdin");

    return user_input.clone().replace("\r\n", "")
}
#[tokio::main]
async fn db() -> Result<(), sqlx::Error> {
let pool = PgPoolOptions::new().max_connections(5).connect(dotenv::var("DB").unwrap().as_str()).await?;

Ok(())
}