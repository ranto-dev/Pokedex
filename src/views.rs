use crate::models::Pokemon;

pub fn afficher_liste(list: &Vec<Pokemon>) {
    println!("=== LISTE DES POKÉMONS ===");
    for p in list {
        println!("#{} - {} ({:?})", p.id, p.nom, p.types);
    }
}

pub fn afficher_details(p: &Pokemon) {
    println!("=== DÉTAILS DE {} ===", p.nom);
    println!("HP: {} | ATK: {} | DEF: {} | VIT: {}", p.hp, p.att, p.def, p.vitesse);
    println!("Total: {} | Évolution vers ID: {}", p.total, p.id_evolution);
}