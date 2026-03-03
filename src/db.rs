use mongodb::{Client, Database};
use std::env;

pub async fn connect_db() -> Database {
    let uri = env::var("MONGO_URI").expect("MONGO_URI must be set");

    let client = Client::with_uri_str(uri)
        .await
        .expect("Failed to connect to MongoDB");

    client.database("pokedex")
}
