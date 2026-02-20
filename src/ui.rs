use dialoguer::{Input, Select};
use colored::*;
use prettytable::{Table, row};

use crate::models::Pokemon;
use crate::storage::{load_pokemons, save_pokemons};

pub fn start() {
    loop {
        let options = vec![
            "➕ Add Pokemon",
            "📋 List Pokemons",
            "✏️ Update Pokemon",
            "🗑 Delete Pokemon",
            "🚪 Exit",
        ];

        let selection = Select::new()
            .with_prompt("Choose an action")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => add_pokemon(),
            1 => list_pokemons(),
            2 => update_pokemon(),
            3 => delete_pokemon(),
            4 => {
                println!("{}", "Goodbye Trainer 👋".bright_green());
                break;
            }
            _ => {}
        }
    }
}

fn add_pokemon() {
    let id: u8 = Input::new().with_prompt("ID").interact().unwrap();
    let nom: String = Input::new().with_prompt("Name").interact().unwrap();
    let types_input: String = Input::new().with_prompt("Types (comma separated)").interact().unwrap();
    let total: u32 = Input::new().with_prompt("Total").interact().unwrap();
    let hp: u32 = Input::new().with_prompt("HP").interact().unwrap();
    let att: u32 = Input::new().with_prompt("Attack").interact().unwrap();
    let def: u32 = Input::new().with_prompt("Defense").interact().unwrap();
    let vitesse: u32 = Input::new().with_prompt("Speed").interact().unwrap();
    let id_evolution: u8 = Input::new().with_prompt("Evolution ID").interact().unwrap();

    let pokemon = Pokemon {
        id,
        nom,
        types: types_input.split(',').map(|s| s.trim().to_string()).collect(),
        total,
        hp,
        att,
        def,
        vitesse,
        id_evolution,
    };

    let mut pokemons = load_pokemons();
    pokemons.push(pokemon);
    save_pokemons(&pokemons);

    println!("{}", "Pokemon added successfully!".green());
}

fn list_pokemons() {
    let pokemons = load_pokemons();

    if pokemons.is_empty() {
        println!("{}", "No Pokemon found.".yellow());
        return;
    }

    let mut table = Table::new();
    table.add_row(row![
        "ID", "Name", "Types", "HP", "ATT", "DEF", "SPD", "TOTAL", "EVOL"
    ]);

    for p in pokemons {
        table.add_row(row![
            p.id,
            p.nom,
            p.types.join(", "),
            p.hp,
            p.att,
            p.def,
            p.vitesse,
            p.total,
            p.id_evolution
        ]);
    }

    table.printstd();
}

fn delete_pokemon() {
    let id: u8 = Input::new().with_prompt("Enter Pokemon ID to delete").interact().unwrap();

    let mut pokemons = load_pokemons();
    pokemons.retain(|p| p.id != id);
    save_pokemons(&pokemons);

    println!("{}", "Pokemon deleted.".red());
}

fn update_pokemon() {
    let id: u8 = Input::new().with_prompt("Enter Pokemon ID to update").interact().unwrap();

    let nom: String = Input::new().with_prompt("New Name").interact().unwrap();
    let types_input: String = Input::new().with_prompt("New Types").interact().unwrap();
    let total: u32 = Input::new().with_prompt("New Total").interact().unwrap();
    let hp: u32 = Input::new().with_prompt("New HP").interact().unwrap();
    let att: u32 = Input::new().with_prompt("New Attack").interact().unwrap();
    let def: u32 = Input::new().with_prompt("New Defense").interact().unwrap();
    let vitesse: u32 = Input::new().with_prompt("New Speed").interact().unwrap();
    let id_evolution: u8 = Input::new().with_prompt("New Evolution ID").interact().unwrap();

    let mut pokemons = load_pokemons();

    for p in &mut pokemons {
        if p.id == id {
            *p = Pokemon {
                id,
                nom: nom.clone(),
                types: types_input.split(',').map(|s| s.trim().to_string()).collect(),
                total,
                hp,
                att,
                def,
                vitesse,
                id_evolution,
            };
        }
    }

    save_pokemons(&pokemons);
    println!("{}", "Pokemon updated.".bright_green());
}