mod models;
mod storage;
mod commands;
mod banner;

use clap::{Parser, Subcommand};
use models::Pokemon;
use commands::*;
use banner::show_banner;

#[derive(Parser)]
#[command(
    name = "pokedex",
    author = "ranto-dev",
    version = "1.0.0",
    about = "⚡ Manage your Pokemon collection from terminal",
    long_about = "
POKEDEX CLI 🧭

A powerful Pokemon manager built in Rust.
Store, update, list and manage your Pokemon collection
using a JSON database.

Built with ❤️ in Rust.
",
    after_help = "
EXAMPLES:
    pokedex add 1 Pikachu Electric 320 35 55 40 90 0
    pokedex list
    pokedex remove 1
    pokedex update 6 Charizard Fire,Flying 540 80 90 80 110 0

TIP:
    Separate multiple types with commas.
"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "➕ Add a new Pokemon")]
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

    #[command(about = "📋 List all Pokemons")]
    List,

    #[command(about = "🗑 Remove a Pokemon by ID")]
    Remove {
        id: u8,
    },

    #[command(about = "✏️ Update a Pokemon")]
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
