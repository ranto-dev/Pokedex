use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pokemon {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub nom: String,
    pub types: Vec<String>,
    pub total: u32,
    pub hp: u32,
    pub att: u32,
    pub def: u32,
    pub vitesse: u32,
    pub id_evolution: u8,
}