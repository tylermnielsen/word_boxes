// scan and index words to be retrieved via index
// check if word is in index
use std::{collections::HashSet, path::Path};
use std::fs::File; 
use std::io::{BufReader, BufRead}; 

pub struct WordBank {
    pub lookup: HashSet<String>,
    pub bank: Vec<String>,
}

impl WordBank {
    pub fn new(path: &Path, size: usize) -> WordBank {
        let mut bank: Vec<String> = Vec::new();

        let file = match File::open(path) {
          Err(why) => panic!("couldn't open {}: {}", path.display(), why),
          Ok(file) => file, 
        };

        let mut reader = BufReader::new(file); 
        let mut line: String = String::with_capacity(32); // preallocate
        while let Ok(len) = reader.read_line(&mut line) && (len > 0) {
          let word = line.trim().to_string(); // remove whitespace and newline

          if word.chars().count() == size {
            bank.push(word.clone()); 
          }

          line.clear(); 
        }

        let lookup: HashSet<String> = bank.iter().cloned().collect();   

        return WordBank { lookup, bank };
    }
}
