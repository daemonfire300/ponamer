use std::{
    collections::HashMap,
    fs::File,
    io::{read_to_string, Read, Write},
};

use rusqlite::{params, Connection};

pub fn load_obfuscation_elements_from_file(path: &str) -> Option<Vec<String>> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return None,
    };
    let contents = if let Ok(contents) = read_to_string(file) {
        contents
    } else {
        return None;
    };
    let out: Vec<String> = contents.lines().map(|l| l.to_owned()).collect();
    return Some(out);
}

#[derive(Debug)]
pub struct FileStore {
    pub code_names: HashMap<String, String>,
}

pub struct SqlLiteStore {
    conn: Connection,
}

impl SqlLiteStore {
    pub fn load(path: &str) -> Self {
        let res = Connection::open(path);
        match res {
            Ok(conn) => {
                let mut s = Self { conn };
                s.build_table();
                s.build_indices();
                return s;
            }
            Err(err) => panic!("{}", err),
        }
    }

    fn build_table(&mut self) {
        match self.conn.execute(
            "CREATE TABLE IF NOT EXISTS code_names(
                        customer_name TEXT,
                        code_name TEXT
)",
            (),
        ) {
            Ok(_) => println!("Tables created..."),
            Err(err) => println!("Creating tables failed: {}", err),
        }
    }

    fn build_indices(&mut self) {
        match self.conn.execute(
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_code_names ON code_names(
                        code_name
)",
            (),
        ) {
            Ok(_) => println!("Indices created..."),
            Err(err) => println!("Creating indices failed: {}", err),
        }
    }

    pub fn add(&mut self, name: &str, code_name: &str) -> Option<bool> {
        match self.conn.execute(
            "INSERT INTO code_names(customer_name,code_name) VALUES (?1,?2)
                ON CONFLICT DO NOTHING
            ",
            params![name, code_name],
        ) {
            Ok(_) => Some(true),
            Err(err) => {
                println!("Failed to insert: {}", err);
                None
            }
        }
    }

    pub fn get(&mut self, name: &str) -> Option<String> {
        let mut stmt = self
            .conn
            .prepare("SELECT code_name FROM code_names WHERE customer_name = ?")
            .unwrap();
        let iter = stmt
            .query_map(params![name], |row| Ok(row.get::<_, String>(0).unwrap()))
            .unwrap();
        iter.into_iter().take(1).map(|e| e.unwrap()).last()
    }
}

impl FileStore {
    pub fn load(path: &str) -> Self {
        let mut file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let code_names = HashMap::<String, String>::new();
        let res = contents.lines().map(|line| line.split(',')).fold(
            code_names,
            |mut acc: HashMap<String, String>, mut spl| -> HashMap<String, String> {
                acc.insert(
                    spl.next().unwrap().to_string(),
                    spl.next().unwrap().to_string(),
                );
                acc
            },
        );
        Self { code_names: res }
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let mut f = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;
        for (k, v) in self.code_names.iter() {
            write!(f, "{},{}\n", k, v)?;
        }
        Ok(())
    }

    pub fn add(&mut self, name: &str, code_name: &str) -> Option<bool> {
        self.code_names
            .entry(name.to_string())
            .or_insert(code_name.to_string());
        Some(false)
    }

    pub fn get(&mut self, name: &str) -> Option<&str> {
        match self.code_names.get(name) {
            Some(val) => Some(val.as_str()),
            _ => None,
        }
    }
}
