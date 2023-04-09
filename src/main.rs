use random_generator::creator::generate_word;

mod random_generator;
mod config;
fn main() {
    for _ in 0..config::MAX_WORD_COUNT {
        let word = generate_word();
        print!("{} {}", word, config::WHITESPACE);
    }
}