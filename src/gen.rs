use std::str::FromStr;

use rand::{rngs::StdRng, Rng};

#[derive(Debug)]
struct Obfuscator<'a> {
    subjects: Vec<&'a str>,
    adjectives: Vec<&'a str>,
    rng: &'a mut StdRng,
}

impl Obfuscator<'_> {
    pub fn obfuscate(&mut self) -> String {
        let i_sub = self.rng.gen_range(0..self.subjects.len());
        let i_adj = self.rng.gen_range(0..self.adjectives.len());
        let mut code_name = String::from_str(self.adjectives.get(i_adj).unwrap()).unwrap();
        code_name.push_str("-");
        code_name.push_str(self.subjects.get(i_sub).unwrap());
        return code_name;
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;

    use super::*;
    #[test]
    fn base_case() {
        let mut obf = Obfuscator {
            subjects: vec!["donkey", "cucumber", "fish", "skyscraper"],
            adjectives: vec!["tall", "small", "wide", "funky", "crazy"],
            rng: &mut StdRng::from_entropy(),
        };
        let res = obf.obfuscate();
        assert_eq!("rich-donkey", res)
    }
}
