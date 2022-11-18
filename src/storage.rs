use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

#[derive(Debug)]
pub struct FileStore {
    pub code_names: HashMap<String, String>,
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
