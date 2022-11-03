use std::env::args;

use rand::{rngs::StdRng, SeedableRng};

mod gen;

fn main() {
    let vec: Vec<String> = args().collect();
    let mut obf = gen::Obfuscator {
        subjects: vec!["donkey", "cucumber", "fish", "skyscraper"],
        adjectives: vec!["tall", "small", "wide", "funky", "crazy"],
        rng: &mut StdRng::from_entropy(),
    };
    match vec.get(1) {
        Some(name) => {
            let code_name = obf.obfuscate();
            println!("{} --> {}", name, code_name)
        }
        None => println!("no input given"),
    }
}
