use std::{collections::HashMap, env::args};

use rand::{rngs::StdRng, SeedableRng};

mod gen;
mod storage;

fn main() {
    let vec: Vec<String> = args().collect();
    let mut obf = gen::Obfuscator {
        subjects: vec!["donkey", "cucumber", "fish", "skyscraper"],
        adjectives: vec!["tall", "small", "wide", "funky", "crazy"],
        rng: &mut StdRng::from_entropy(),
    };
    let mut store = storage::FileStore {
        code_names: HashMap::new(),
    };
    match vec.get(1) {
        Some(name) => {
            match name.as_str() {
                "add" => {
                    let code_name = obf.obfuscate();
                    store.add(name.to_string(), code_name);
                }
                "get" => {}
                _ => {}
            }
            let code_name = obf.obfuscate();
            println!("{} --> {}", name, code_name)
        }
        None => println!("no input given"),
    }
    store.save("./names.dat").unwrap();
}
