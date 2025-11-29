use crate::{models::Pokemon, storage, views};
use std::io::{self, Write};

pub fn run() {
    let mut pokedex = storage::load();

    loop {
        println!("\n=== MENU POKEDEX ===");
        println!("1. Voir liste");
        println!("2. Ajouter un Pokémon");
        println!("3. Voir détails");
        println!("4. Quitter");
        print!("Choix: ");
        io::stdout().flush().unwrap();

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();

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
    let mut s = String::new();

    println!("Nom:");
    io::stdin().read_line(&mut s).unwrap();
    let nom = s.trim().to_string();
    s.clear();

    println!("Types (séparés par virgule):");
    io::stdin().read_line(&mut s).unwrap();
    let types = s.trim().split(',').map(|x| x.trim().to_string()).collect();
    s.clear();

    println!("HP:");
    io::stdin().read_line(&mut s).unwrap();
    let hp = s.trim().parse().unwrap_or(0);
    s.clear();

    println!("ATK:");
    io::stdin().read_line(&mut s).unwrap();
    let att = s.trim().parse().unwrap_or(0);
    s.clear();

    println!("DEF:");
    io::stdin().read_line(&mut s).unwrap();
    let def = s.trim().parse().unwrap_or(0);
    s.clear();

    println!("Vitesse:");
    io::stdin().read_line(&mut s).unwrap();
    let vitesse = s.trim().parse().unwrap_or(0);
    s.clear();

    println!("Total:");
    io::stdin().read_line(&mut s).unwrap();
    let total = s.trim().parse().unwrap_or(0);
    s.clear();

    println!("ID de l'évolution:");
    io::stdin().read_line(&mut s).unwrap();
    let id_evolution = s.trim().parse().unwrap_or(0);

    let id = (pokedex.len() + 1) as u8;

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
    let mut s = String::new();
    println!("ID du Pokémon à afficher:");
    io::stdin().read_line(&mut s).unwrap();
    let id: u8 = s.trim().parse().unwrap_or(255);

    if let Some(p) = pokedex.iter().find(|p| p.id == id) {
        views::afficher_details(p);
    } else {
        println!("Pokémon introuvable !");
    }
}
