use core::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use std::{collections::HashMap, path::Path};

use crate::word_box::WordBox;

struct TrieNode {
    pub letter: char,
    pub children: HashMap<char, TrieNode>,
    pub valid: bool,
}

impl fmt::Debug for TrieNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}) children: {:?}",
            self.letter,
            self.valid,
            self.children.keys()
        )
    }
}

impl fmt::Display for TrieNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}) children: {:?}",
            self.letter,
            self.valid,
            self.children.keys()
        )
    }
}

impl TrieNode {
    pub fn new(letter: char) -> TrieNode {
        return TrieNode {
            letter: letter,
            children: HashMap::new(),
            valid: false,
        };
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

    #[allow(dead_code)]
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

    pub fn find_boxes(&self, start: usize, stop: usize) -> Vec<WordBox> {
        let mut boxes: Vec<WordBox> = Vec::new();

        let mut words: Vec<usize> = Vec::new();
        words.push(start);
        let mut gens = vec![vec![&self.lookup; self.size]];

        while words.len() > 0 {
            // println!("Words: {:?} \nGens: {:?}", words, gens.last().unwrap());
            // try to add the top of place to the box
            let mut next_gen: Vec<&TrieNode> = Vec::with_capacity(self.size);
            let mut failed = false;
            for i in 0..(self.size) {
                let top = &self.bank[*words.last().unwrap()];

                if gens.last().unwrap()[i].children.contains_key(&top[i]) {
                    next_gen.push(&gens.last().unwrap()[i].children[&top[i]]);
                } else {
                    failed = true;
                    break;
                }
            }

            // println!("Failed: {}", failed);

            // success condition
            if failed == false {
                if words.len() == self.size {
                    let mut wb = WordBox::new();
                    for w in &words {
                        wb.add_row(self.bank[*w].clone());
                    }
                    boxes.push(wb);
                    // println!("{}", boxes.last().unwrap());
                } else {
                    gens.push(next_gen);
                }
            }

            // in any case we advance
            // increment the top if it exists

            if failed || words.len() == self.size {
                if let Some(top) = words.last_mut() {
                    *top += 1;
                } else {
                    words.push(0);
                }
            } else {
                words.push(0);
            }

            // cascade the increment if necessary
            while let Some(top) = words.last() {
                if (words.len() == 1 && *top >= stop) || *top >= self.bank.len() {
                    words.pop(); // cascade
                    gens.pop();

                    if let Some(next_top) = words.last_mut() {
                        *next_top += 1;
                    }
                } else {
                    break; // top is within the bank
                }
            }
            // println!();
        }

        return boxes;
    }

    pub fn find_boxes_multithreaded(&self, thread_count: usize) -> Vec<WordBox> {
        let all_words = self.bank.len();
        let set_size = all_words / thread_count;

        let boxes = thread::scope(|s| {
            let mut handles = Vec::new();
            for i in 0..thread_count {
                let start = i * set_size;
                let stop = if i == thread_count - 1 {
                    all_words
                } else {
                    start + set_size
                };

                handles.push(s.spawn(move || {
                    return self.find_boxes(start, stop);
                }));
            }

            let mut boxes = Vec::new();
            for h in handles {
                match h.join() {
                    Ok(mut new_boxes) => boxes.append(&mut new_boxes),
                    Err(_) => println!("Error on join"),
                }
            }

            return boxes;
        });

        return boxes;
    }
}
