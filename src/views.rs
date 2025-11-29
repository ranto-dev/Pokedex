use crate::models::Pokemon;

pub fn afficher_liste(list: &Vec<Pokemon>) {
    if list.len() <= 0 {
        println!("Vote pokedex est enore vide en ce moment");
    } else {
        println!("=== LISTE DES POKÉMONS ===");
        println!(
            "+----+-----------------+-------------------------+-------+-----+-----+-----+---------+-------------+"
        );
        println!(
            "| ID | Nom             | Types                   | Total | HP  | Att | Def | Vitesse | ÉvolutionID |"
        );
        println!(
            "+----+-----------------+-------------------------+-------+-----+-----+-----+---------+-------------+"
        );

        for p in list {
            let types_str: String = p.types.join(", ");
            println!(
                "| {:<2} | {:<15} | {:<23} | {:<5} | {:<3} | {:<3} | {:<3} | {:<7} | {:<11} |",
                p.id, p.nom, types_str, p.total, p.hp, p.att, p.def, p.vitesse, p.id_evolution
            );
        }

        println!(
            "+----+-----------------+-------------------------+-------+-----+-----+-----+---------+-------------+"
        );
    }
}

pub fn afficher_details(p: &Pokemon) {
    println!("=== DÉTAILS DE {} ===", p.nom);
    println!(
        "HP: {} | ATK: {} | DEF: {} | VIT: {}",
        p.hp, p.att, p.def, p.vitesse
    );
    println!("Total: {} | Évolution vers ID: {}", p.total, p.id_evolution);
}
