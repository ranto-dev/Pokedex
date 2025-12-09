use crate::models::Pokemon;
use serde_json;
use std::fs;
use std::path::Path;

const FILE_PATH: &str = "pokedex.json";

pub fn load() -> Vec<Pokemon> {
    if !Path::new(FILE_PATH).exists() {
        if let Err(e) = fs::write(FILE_PATH, "[]") {
            eprintln!("Erreur lors de la création du fichier {}: {}", FILE_PATH, e);
            return Vec::new();
        }
    }

    let data: String = match fs::read_to_string(FILE_PATH) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Erreur lors de la lecture du fichier {}: {}", FILE_PATH, e);
            return Vec::new();
        }
    };

    serde_json::from_str(&data).unwrap_or_else(|e: serde_json::Error| {
        eprintln!("Erreur de désérialisation JSON : {}", e);
        Vec::new()
    })
}

pub fn save(pokemons: &Vec<Pokemon>) {
    let data: String = match serde_json::to_string_pretty(pokemons) {
        Ok(json_data) => json_data,
        Err(e) => {
            eprintln!("Erreur lors de la sérialisation en JSON : {}", e);
            return;
        }
    };

    if let Err(e) = fs::write(FILE_PATH, data) {
        eprintln!(
            "Erreur lors de l'écriture dans le fichier {}: {}",
            FILE_PATH, e
        );
    }
}
