mod input;

#[derive(Debug)]
struct Pokemon {
    id: u8,
    nom: String,
    types: Vec<String>,
    total: u32,
    hp: u32,
    att: u32,
    def: u32,
    vitesse: u32,
    id_evolution: u8,
}

impl Pokemon {
    fn nouveau(last_id: u8) -> Pokemon {
        return Pokemon {
            id: last_id + 1,
            nom: String::new(),
            types: Vec::new(),
            total: 0,
            hp: 0,
            att: 0,
            def: 0,
            vitesse: 0,
            id_evolution: 0,
        };
    }
}

// lister les pokemons dans lae pokedex
fn lister_pokedex(pokedex: &Vec<Pokemon>) {
    if pokedex.len() <= 0 {
        println!("Vote pokedex est enore vide en ce moment");
    } else {
        println!(
            "+----+-----------------+-------------------------+-------+-----+-----+-----+---------+-------------+"
        );
        println!(
            "| ID | Nom             | Types                   | Total | HP  | Att | Def | Vitesse | ÉvolutionID |"
        );
        println!(
            "+----+-----------------+-------------------------+-------+-----+-----+-----+---------+-------------+"
        );

        for p in pokedex {
            let types_str = p.types.join(", ");
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

// Ajouter un nouveau pokemon dans la pokedex
fn ajouter_pokemon(pokedex: &mut Vec<Pokemon>) {
    let last_id: u8 = pokedex.len() as u8;
    let mut nouveau_pokemon: Pokemon = Pokemon::nouveau(last_id);

    println!("Quel est le nom du nouveau pokemon?");
    nouveau_pokemon.nom = input::saisie_utilisateur();

    println!("Type du nouveau pokemon");
    println!("-- enter le nombre de types de ");
    let nombre_types: u32 = input::saisie_utilisateur();
    for count in 0..nombre_types {
        println!("Saisir le {} type de ce pokemon", count);
        nouveau_pokemon.types.push(input::saisie_utilisateur());
    }

    nouveau_pokemon.hp = 100;

    println!("Saisir le capacité d'attack du pokemon");
    nouveau_pokemon.att = input::saisie_utilisateur();

    println!("Saisir le capacité de défense du pokemon");
    nouveau_pokemon.def = input::saisie_utilisateur();

    println!("Saisir la vitesse du pokemon");
    nouveau_pokemon.vitesse = input::saisie_utilisateur();

    nouveau_pokemon.total =
        nouveau_pokemon.hp + nouveau_pokemon.att + nouveau_pokemon.def + nouveau_pokemon.vitesse;

    nouveau_pokemon.id_evolution = 0;

    pokedex.push(nouveau_pokemon);
}

fn main() {
    let mut pokedex: Vec<Pokemon> = Vec::new();
    lister_pokedex(&mut pokedex);
    ajouter_pokemon(&mut pokedex);
    lister_pokedex(&mut pokedex);
}
