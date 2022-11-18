use std::{
    collections::HashMap,
    fs::{read_to_string, File},
    io::Write,
};

#[derive(Debug)]
pub struct FileStore {
    pub code_names: HashMap<String, String>,
}

impl FileStore {
    pub fn load(path: &str) -> Self {
        let contents = read_to_string(path).unwrap();
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
            write!(f, "{},{}", k, v)?;
        }
        Ok(())
    }

    pub fn add(&mut self, name: String, code_name: String) -> Option<bool> {
        self.code_names.entry(name).or_insert(code_name);
        Some(false)
    }

    pub fn get(&mut self, name: String, code_name: String) -> Option<&str> {
        let out = self.code_names.get(name).map(|e| e.as_str());
    }
}
