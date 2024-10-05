use mongodb::{ bson::doc, options::{ ClientOptions, ServerApi, ServerApiVersion }, Client, Collection };
use dotenv::dotenv;
use serde::{ Deserialize, Serialize };
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    user_id: String,
    username: String,
    balance: i32,
}
pub async fn connect() -> mongodb::error::Result<Collection<User>> {
    dotenv().ok();
    // Replace the placeholder with your Atlas connection string
    let uri = std::env::var("DB").unwrap();
    let mut client_options = ClientOptions::parse(uri).await?;
    // Set the server_api field of the client_options object to Stable API version 1
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    // Create a new client and connect to the server
    let client = Client::with_options(client_options)?;

    // Send a ping to confirm a successful connection
    let my_coll: Collection<User> = client.database("rust").collection("users");
    let cursor = my_coll.find_one(doc! { "user_id": "1"}).await?;
    println!("{:?}", cursor);
    Ok(my_coll)
}
