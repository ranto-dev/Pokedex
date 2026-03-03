use actix_web::{HttpResponse, Responder, web};
use futures::stream::StreamExt;
use mongodb::Database;
use mongodb::bson::{doc, oid::ObjectId};

use crate::models::Pokemon;

// GET /pokemons
pub async fn get_all(db: web::Data<Database>) -> impl Responder {
    let collection = db.collection::<Pokemon>("pokemons");

    let mut cursor = collection.find(None, None).await.unwrap();
    let mut pokemons: Vec<Pokemon> = Vec::new();

    while let Some(result) = cursor.next().await {
        pokemons.push(result.unwrap());
    }

    HttpResponse::Ok().json(pokemons)
}

// GET /pokemons/{id}
pub async fn get_one(db: web::Data<Database>, path: web::Path<String>) -> impl Responder {
    let collection = db.collection::<Pokemon>("pokemons");

    // Convertir String -> ObjectId
    let obj_id = match ObjectId::parse_str(path.into_inner()) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().body("Invalid ObjectId format");
        }
    };

    match collection.find_one(doc! { "_id": obj_id }, None).await {
        Ok(Some(pokemon)) => HttpResponse::Ok().json(pokemon),
        Ok(None) => HttpResponse::NotFound().body("Pokemon not found"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// POST /pokemons
pub async fn create(db: web::Data<Database>, new_pokemon: web::Json<Pokemon>) -> impl Responder {
    let collection = db.collection::<Pokemon>("pokemons");

    let mut pokemon = new_pokemon.into_inner();
    pokemon.id = None;

    let insert_result = collection.insert_one(pokemon, None).await.unwrap();

    HttpResponse::Created().json(insert_result.inserted_id)
}

// DELETE /pokemons/{id}
pub async fn delete(db: web::Data<Database>, path: web::Path<String>) -> impl Responder {
    let collection = db.collection::<Pokemon>("pokemons");

    let obj_id = ObjectId::parse_str(path.into_inner()).unwrap();

    collection
        .delete_one(doc! { "_id": obj_id }, None)
        .await
        .unwrap();

    HttpResponse::Ok().body("Deleted")
}
