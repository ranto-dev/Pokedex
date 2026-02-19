use crate::models::Pokemon;
use crate::storage::{load_pokemons, save_pokemons};

pub fn add(pokemon: Pokemon) {
    let mut pokemons = load_pokemons();
    pokemons.push(pokemon);
    save_pokemons(&pokemons);
    println!("✅ Pokemon ajouté !");
}

pub fn list() {
    let pokemons = load_pokemons();

    if pokemons.is_empty() {
        println!("⚠️ Aucun pokemon trouvé.");
        return;
    }

    for p in pokemons {
        println!("{:#?}", p);
    }
}

pub fn remove(id: u8) {
    let mut pokemons = load_pokemons();
    pokemons.retain(|p| p.id != id);
    save_pokemons(&pokemons);
    println!("🗑 Pokemon supprimé !");
}

pub fn update(id: u8, updated: Pokemon) {
    let mut pokemons = load_pokemons();

    for p in &mut pokemons {
        if p.id == id {
            *p = updated.clone();
        }
    }

    save_pokemons(&pokemons);
    println!("✏️ Pokemon mis à jour !");
}
