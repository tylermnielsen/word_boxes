use core::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{collections::HashMap, path::Path};

use crate::word_box::WordBox;

#[derive(Default, Debug)]
struct TrieNode {
  pub letter: char, 
  pub children: HashMap<char, TrieNode>, 
  pub valid: bool,
}

impl fmt::Display for TrieNode {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} ({}) children: {:?}", self.letter, self.valid, self.children.keys())
  }
}

impl TrieNode {
  pub fn new(letter: char) -> TrieNode {
    return TrieNode {
      letter: letter, 
      children: HashMap::new(), 
      valid: false
    }
  }
}

pub struct WordBank {
    pub size: usize,
    lookup: TrieNode,
    pub bank: Vec<Vec<char>>,
}

impl WordBank {
    pub fn new(path: &Path, size: usize) -> WordBank {
        let mut bank: Vec<Vec<char>> = Vec::new();

        let file = match File::open(path) {
            Err(why) => panic!("couldn't open {}: {}", path.display(), why),
            Ok(file) => file,
        };

        let mut reader = BufReader::new(file);
        let mut line: String = String::with_capacity(32); // preallocate
        while let Ok(len) = reader.read_line(&mut line) {
            if len <= 0 {
                break;
            }
            let word = line.trim().to_string(); // remove whitespace and newline

            if word.chars().count() == size {
                bank.push(word.chars().collect());
            }

            line.clear();
        }

        let mut lookup: TrieNode = TrieNode::new('_');

        for word in &bank {
          let mut temp = &mut lookup; 
          for c in word {
            if temp.children.contains_key(c) == false {
              temp.children.insert(*c, TrieNode::new(*c)); 
            }
            // tested and created before if needed 
            temp = temp.children.get_mut(c).unwrap(); 
          }
          temp.valid = true; 
        }

        return WordBank { size, lookup, bank };
    }

    pub fn is_valid_word(&self, word: &Vec<char>) -> bool {
      let mut temp = &self.lookup; 
      
      for c in word {
        match temp.children.get(c) {
          Some(child) => temp = child, 
          None => return false,
        }
      }

      return temp.valid; 
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
                wb.add_row(&self.bank[*p]);
            }

            let mut word: Vec<char> = Vec::with_capacity(self.size);
            let mut valid: bool = true;
            for col in 0..wb.letters.len() {
                for row in 0..wb.letters.len() {
                    word.push(wb.letters[row][col]);
                }
                if self.is_valid_word(&word) == false {
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
                if i + 1 >= place.len() {
                    return boxes;
                } else {
                    place[i + 1] += 1;
                }

                i += 1;
            }
        }
    }
}
