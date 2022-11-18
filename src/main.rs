use std::env::args;

use rand::{rngs::StdRng, SeedableRng};

mod gen;
mod storage;

const DEFAULT_FILE_PATH: &'static str = "./names.dat";

fn main() {
    let vec: Vec<String> = args().collect();
    let mut obf = gen::Obfuscator {
        subjects: vec!["donkey", "cucumber", "fish", "skyscraper"],
        adjectives: vec!["tall", "small", "wide", "funky", "crazy"],
        rng: &mut StdRng::from_entropy(),
    };
    let mut store = storage::FileStore::load(DEFAULT_FILE_PATH);
    let op = parse_args(&vec);
    match op {
        Op::Add(name) => {
            let code_name = obf.obfuscate();
            store.add(&name.to_string(), &code_name);
            println!("{:?}: {} --> {}", op, name, code_name);
            store.save(DEFAULT_FILE_PATH).unwrap();
        }
        Op::Get(name) => match store.get(name) {
            Some(code_name) => println!("{}", code_name),
            None => println!("not found"),
        },
        Op::Invalid => println!("no input given"),
    }
}

#[derive(Debug)]
enum Op<'a> {
    Add(&'a str),
    Get(&'a str),
    Invalid,
}

fn parse_args(args: &Vec<String>) -> Op {
    if args.len() < 3 {
        return Op::Invalid;
    }
    let val = if let Some(val) = args.get(2) {
        if val.len() == 0 {
            return Op::Invalid;
        }
        val
    } else {
        return Op::Invalid;
    };
    match args.get(1) {
        Some(op) => match op.as_str() {
            "add" => Op::Add(val),
            "get" => Op::Get(val),
            _ => Op::Invalid,
        },
        None => Op::Invalid,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_args_add_ok() {
        let name = "name";
        let args: Vec<String> = vec!["".to_string(), "add".to_string(), name.to_string()];
        match parse_args(&args) {
            Op::Add(op_arg) => assert!(
                name == op_arg,
                "operation arg does not match expected value"
            ),
            _ => assert!(false),
        }
    }

    #[test]
    fn parse_args_add_invalid() {
        let name = "";
        let args: Vec<String> = vec!["".to_string(), "add".to_string(), name.to_string()];
        match parse_args(&args) {
            Op::Invalid => assert!(true),
            _ => assert!(false, "no valid operation should result from empty arg"),
        }
    }
}
