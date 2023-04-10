use std::io::ErrorKind;

use rand::prelude::*;
use crate::config;

use super::generation_types::{GenerationType};

#[warn(unreachable_patterns)]
pub fn generate(generation_type: GenerationType) -> () {
    match generation_type {
        GenerationType::ITERATIVE => generate_in_iterative_way(),
        GenerationType::LOGICAL => generate_in_logical_way(),        
        _ => eprintln!("Ocorreu um erro ao gerar. Tipo de geração inválido")
    }
}

fn generate_in_iterative_way() -> () {
    for _ in 0..config::MAX_WORD_COUNT {
        let word = generate_word();
        print!("{}{}", word, config::WHITESPACE);
    }
    return;
}

fn generate_in_logical_way() -> () {
    let mut word = String::new();
    while word != config::TARGET_WORD {
        word = generate_word();
        print!("{}{}", word, config::WHITESPACE);
    }
}

fn generate_word() -> String {
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