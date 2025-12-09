use std::io;
use std::str::FromStr;

fn buffer_created() -> String {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur lors de la lecture de l'entrée");
    input
}

pub fn user_input<T: FromStr>() -> T {
    let input: String = buffer_created();
    input
        .trim()
        .parse::<T>()
        .ok()
        .expect("Erreur : impossible de convertir l'entrée en type générique")
}
