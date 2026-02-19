mod models;
mod storage;
mod commands;
mod banner;

use clap::{Parser, Subcommand};
use models::Pokemon;
use commands::*;
use banner::show_banner;


#[derive(Parser)]
#[command(name = "Pokedex CLI")]
#[command(about = "Manage your pokemon list")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        id: u8,
        nom: String,
        types: String,
        total: u32,
        hp: u32,
        att: u32,
        def: u32,
        vitesse: u32,
        id_evolution: u8,
    },
    List,
    Remove {
        id: u8,
    },
    Update {
        id: u8,
        nom: String,
        types: String,
        total: u32,
        hp: u32,
        att: u32,
        def: u32,
        vitesse: u32,
        id_evolution: u8,
    },
}

fn main() {
    show_banner();
    
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { id, nom, types, total, hp, att, def, vitesse, id_evolution } => {
            let pokemon = Pokemon {
                id,
                nom,
                types: types.split(',').map(|s| s.trim().to_string()).collect(),
                total,
                hp,
                att,
                def,
                vitesse,
                id_evolution,
            };
            add(pokemon);
        }

        Commands::List => list(),

        Commands::Remove { id } => remove(id),

        Commands::Update { id, nom, types, total, hp, att, def, vitesse, id_evolution } => {
            let pokemon = Pokemon {
                id,
                nom,
                types: types.split(',').map(|s| s.trim().to_string()).collect(),
                total,
                hp,
                att,
                def,
                vitesse,
                id_evolution,
            };
            update(id, pokemon);
        }
    }
}
