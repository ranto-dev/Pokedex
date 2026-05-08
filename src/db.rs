// Database Connecion

use mongodb::{Client, Database};
use std::env;

pub async fn connect_db() -> Database {
    let uri: String =
        env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

    let client: Client = Client::with_uri_str(uri)
        .await
        .expect("Failed to connect to MongoDB");

    client.database("pokedex")
}
