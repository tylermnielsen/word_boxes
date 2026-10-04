use core::fmt;

#[derive(Debug, Clone)]
pub struct WordBox {
    pub letters: Vec<Vec<char>>,
}

impl WordBox {
    pub fn add_row(&mut self, word: &Vec<char>) {
        self.letters.push(word.clone());
    }
}

impl fmt::Display for WordBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut wb: String = String::new();
        for word in &self.letters {
            wb += word.iter().collect::<String>().as_str();
            wb += "\n";
        }
        write!(f, "{}", wb)
    }
}
