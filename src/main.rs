use random_generator::{creator::generate, generation_types::GenerationType};

mod random_generator;
mod config;
fn main() {
    generate(GenerationType::ITERATIVE);
}