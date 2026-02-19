use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use crate::models::Pokemon;

const FILE_PATH: &str = "pokedex.json";

pub fn load_pokemons() -> Vec<Pokemon> {
    let mut file = match File::open(FILE_PATH) {
        Ok(file) => file,
        Err(_) => return Vec::new(),
    };

    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    if data.trim().is_empty() {
        return Vec::new();
    }

    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
}

pub fn save_pokemons(pokemons: &Vec<Pokemon>) {
    let data = serde_json::to_string_pretty(pokemons).unwrap();

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(FILE_PATH)
        .unwrap();

    file.write_all(data.as_bytes()).unwrap();
}
