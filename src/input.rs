use std::io;
use std::str::FromStr;

fn saisie_buffer() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur lors de la lecture de l'entrée");
    input
}

pub fn saisie_utilisateur<T: FromStr>() -> T {
    let input = saisie_buffer();
    input
        .trim()
        .parse::<T>()
        .ok()
        .expect("Erreur : impossible de convertir l'entrée en type générique")
}
