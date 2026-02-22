use colored::*;
use figlet_rs::FIGfont;

pub fn show_banner() {
    let font = FIGfont::standard().unwrap();
    let figure = font.convert("POKEDEX").unwrap();

    println!("{}", figure.to_string().bright_red().bold());
    println!("{}", "⚡ Interactive Pokemon Manager ⚡".bright_blue());
    println!("👨 Developed by Ranto Andrianandraina\n")
}
