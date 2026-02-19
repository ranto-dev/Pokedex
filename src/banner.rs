use colored::*;
use figlet_rs::FIGfont;

pub fn show_banner() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("POKEDEX").unwrap();

    println!("{}", figure.to_string().bright_red().bold());

    println!(
        "{}",
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            .bright_blue()
    );

    println!(
        "{}",
        "⚡ Manage your Pokemon like a real Trainer ⚡"
            .bright_yellow()
            .bold()
    );

    println!(
        "{}",
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n"
            .bright_blue()
    );
}
