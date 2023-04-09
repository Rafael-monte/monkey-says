use std::io::ErrorKind;

use rand::prelude::*;
use crate::config;

pub fn generate_word() -> String {
    let word_size = calculate_word_size();
    let mut word = String::new();
    for _ in 0..word_size {
        let random_idx = calculate_random_index(None, config::ALPHABET.len());
        if random_idx.is_err() {panic!();}
        let letter_from_alphabet = config::ALPHABET.chars().nth(random_idx.unwrap_or(0));
        word.push(letter_from_alphabet.unwrap());
    }
    return word;

}

fn calculate_word_size() -> usize {
    let word_size = calculate_random_index(Option::from(config::MIN_WORD_SIZE), config::MAX_WORD_SIZE);
    if word_size.is_err() {
        panic!()
    }
    return word_size.unwrap();
}

fn calculate_random_index(low_index: Option<usize>, upper_index: usize) -> Result<usize, ErrorKind> {
    if low_index.unwrap_or(0) > upper_index {
        eprintln!("Ocorreu um erro ao calcular índice, index inferior maior que o superior");
        return Err(ErrorKind::InvalidData);
    }
    let mut rng_generator = rand::thread_rng();
    let mut options_range:Vec<usize> = (low_index.unwrap_or(0)..upper_index).collect();
    options_range.shuffle(&mut rng_generator);
    return Ok(options_range.first().unwrap().clone());
}