use crate::{input, models::Pokemon, storage, views};
use std::io::{self, Write};

pub fn run() {
    let mut pokedex: Vec<Pokemon> = storage::load();

    loop {
        println!("\n=== MENU POKEDEX ===");
        println!("1. Voir liste");
        println!("2. Ajouter un Pokémon");
        println!("3. Voir détails");
        println!("4. Quitter");
        print!("Choix: ");
        io::stdout().flush().unwrap();

        let choix: String = input::user_input();

        match choix.trim() {
            "1" => views::afficher_liste(&pokedex),
            "2" => ajouter(&mut pokedex),
            "3" => details(&pokedex),
            "4" => break,
            _ => println!("Choix invalide !"),
        }

        storage::save(&pokedex);
    }
}

fn ajouter(pokedex: &mut Vec<Pokemon>) {
    let mut s: String = String::new();

    println!("Nom:");
    let nom: String = input::user_input();

    println!("Types (séparés par virgule):");
    io::stdin().read_line(&mut s).unwrap();
    let types: Vec<String> = s
        .trim()
        .split(',')
        .map(|x: &str| x.trim().to_string())
        .collect();
    s.clear();

    println!("HP:");
    io::stdin().read_line(&mut s).unwrap();
    let hp: u32 = s.trim().parse().unwrap_or(0);
    s.clear();

    println!("ATK:");
    io::stdin().read_line(&mut s).unwrap();
    let att: u32 = s.trim().parse().unwrap_or(0);
    s.clear();

    println!("DEF:");
    io::stdin().read_line(&mut s).unwrap();
    let def: u32 = s.trim().parse().unwrap_or(0);
    s.clear();

    println!("Vitesse:");
    let vitesse: u32 = s.trim().parse().unwrap_or(0);
    s.clear();

    let total: u32 = att + def + vitesse;
    println!("Total: {total}");

    println!("ID de l'évolution:");
    let id_evolution: u8 = 1;

    let id: u8 = (pokedex.len() + 1) as u8;

    pokedex.push(Pokemon {
        id,
        nom,
        types,
        total,
        hp,
        att,
        def,
        vitesse,
        id_evolution,
    });

    println!("Pokémon ajouté avec succès !");
}

fn details(pokedex: &Vec<Pokemon>) {
    let mut s: String = String::new();
    println!("ID du Pokémon à afficher:");
    io::stdin().read_line(&mut s).unwrap();
    let id: u8 = s.trim().parse().unwrap_or(255);

    if let Some(p) = pokedex.iter().find(|p: &&Pokemon| p.id == id) {
        views::afficher_details(p);
    } else {
        println!("Pokémon introuvable !");
    }
}
