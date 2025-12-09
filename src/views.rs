use crate::models::Pokemon;

pub fn afficher_liste(list: &Vec<Pokemon>) {
    if list.len() <= 0 {
        println!("Vote pokedex est enore vide en ce moment");
    } else {
        println!("=== LA LISTE DES POKÉMONS ===");
        println!("+----+-----------------+");
        println!("| ID | Nom             |");
        println!("+----+-----------------+");

        for pokemon in list {
            println!("| {:<2} | {:<15} |", pokemon.id, pokemon.nom);
        }

        println!("+----+-----------------+");
    }
}

pub fn afficher_details(pokemon: &Pokemon) {
    println!("=== DÉTAILS DE {} ===", pokemon.nom.to_uppercase());
    println!(
        "HP: {} | ATK: {} | DEF: {} | VIT: {}",
        pokemon.hp, pokemon.att, pokemon.def, pokemon.vitesse
    );
    println!(
        "Total: {} | Évolution vers ID: {}",
        pokemon.total, pokemon.id_evolution
    );
}
