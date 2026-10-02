use core::fmt;
// scan and index words to be retrieved via index
// check if word is in index
use std::{collections::HashSet, path::Path};
use std::fs::File; 
use std::io::{BufReader, BufRead}; 

pub struct WordBox {
    pub letters: Vec<Vec<char>>,
}

impl WordBox {
  pub fn add_word(&mut self, word: &String) {
    self.letters.push(word.chars().collect()); 
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

pub struct WordBank {
    size: usize,
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

        return WordBank { size, lookup, bank };
    }

    pub fn find_boxes(&self) -> Vec<WordBox> {
        let mut boxes: Vec<WordBox> = Vec::new(); 

        let mut place: Vec<usize> = vec![0; self.size];

        loop {
          // create and test 
          let mut wb: WordBox = WordBox {
            letters: Vec::new(),
          }; 

          for p in &place {
            wb.add_word(&self.bank[*p]);
          }

          let mut word: String = String::with_capacity(32); 
          let mut valid: bool = true; 
          for col in 0..wb.letters.len() {
            for row in 0..wb.letters.len() {
              word.push(wb.letters[row][col]);
            }
            if self.lookup.contains(&word) == false {
              valid = false; 
              break; 
            }
            word.clear(); 
          }
          
          if valid {  
            println!("{}", wb); 
            boxes.push(wb); 
          }

          // next word box
          place[0] += 1; 
          let mut i = 0; 
          while place[i] >= self.bank.len() {
            place[i] = 0; 
            if i+1 >= place.len(){
              return boxes; 
            } else {
              place[i+1] += 1; 
            }

            i += 1; 
          }

        }
    }
}
