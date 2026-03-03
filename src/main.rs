mod db;
mod handlers;
mod models;

use actix_web::{App, HttpServer, web};
use db::connect_db;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database = connect_db().await;

    println!("🚀 Server running on http://0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            .route("/pokemons", web::get().to(handlers::get_all))
            .route("/pokemons", web::post().to(handlers::create))
            .route("/pokemons/{id}", web::delete().to(handlers::delete))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
