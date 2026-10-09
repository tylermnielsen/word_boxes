use core::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::sync::mpsc;
use std::collections::HashMap;
use std::path::Path;
use std::thread;

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
    width: usize,
    height: usize,
    lookup: TrieNode,
    pub bank: Vec<Vec<char>>,
}

impl WordBank {
    pub fn new(path: &Path, width: usize, height: usize) -> WordBank {
        let mut bank: Vec<Vec<char>> = Vec::new();
        let mut lookup_bank: Vec<Vec<char>> = Vec::new();

        let file = match File::open(path) {
            Err(why) => panic!("couldn't open {}: {}", path.display(), why),
            Ok(file) => file,
        };

        // read into word bank and lookup bank used to create lookup trie
        let mut reader = BufReader::new(file);
        let mut line: String = String::with_capacity(32); // preallocate
        while let Ok(len) = reader.read_line(&mut line) {
            if len <= 0 {
                break;
            }
            let word = line.trim().to_string(); // remove whitespace and newline

            let char_count = word.chars().count();

            if char_count == width {
                bank.push(word.chars().collect());
            }

            if char_count == height {
                lookup_bank.push(word.chars().collect());
            }

            line.clear();
        }

        let mut lookup: TrieNode = TrieNode::new('_');

        for word in &lookup_bank {
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

        return WordBank {
            width,
            height,
            lookup,
            bank,
        };
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

    pub fn find_boxes(&self, start: usize, stop: usize, output: mpsc::Sender<WordBox>) {
        let mut words: Vec<usize> = Vec::new();
        words.push(start);
        let mut gens = vec![vec![&self.lookup; self.width]];

        while words.len() > 0 {
            // println!("Words: {:?} \nGens: {:?}", words, gens.last().unwrap());
            // try to add the top of place to the box
            let mut next_gen: Vec<&TrieNode> = Vec::with_capacity(self.width);
            let mut failed = false;
            for i in 0..(self.width) {
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
                if words.len() == self.height {
                    let mut wb = WordBox::new();
                    for w in &words {
                        wb.add_row(self.bank[*w].clone());
                    }
                    output.send(wb).unwrap(); // send to output
                                              // println!("{}", boxes.last().unwrap());
                } else {
                    gens.push(next_gen);
                }
            }

            // in any case we advance
            // increment the top if it exists

            if failed || words.len() == self.height {
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
    }

    fn storage_thread(output_file: &String, rx: mpsc::Receiver<WordBox>) -> i64 {
        let output_file = Path::new(output_file);

        let file = match File::create(output_file) {
            Err(why) => panic!("couldn't open {}: {}", output_file.display(), why),
            Ok(file) => file,
        };

        let mut writer = BufWriter::new(file);

        let mut temp = String::new();
        let mut count: i64 = 0;
        for wb in rx {
            for word in &wb.letters {
                temp += word.iter().collect::<String>().as_str();
                temp += "\n";
            }
            writeln!(writer, "{}", temp).unwrap();
            temp.clear();
            count += 1;
        }

        return count;
    }

    pub fn find_boxes_multithreaded(&self, thread_count: usize, output_file: &String) -> i64 {
        let all_words = self.bank.len();
        let set_size = all_words / thread_count;

        let count = thread::scope(|s| {
            let (tx, rx) = mpsc::channel::<WordBox>();

            // spawn consumer
            let file_writer = s.spawn(move || {
                return WordBank::storage_thread(output_file, rx);
            });

            let mut handles = Vec::new();
            for i in 0..thread_count {
                let start = i * set_size;
                let stop = if i == thread_count - 1 {
                    all_words
                } else {
                    start + set_size
                };

                let tx_inst = tx.clone();
                handles.push(s.spawn(move || {
                    self.find_boxes(start, stop, tx_inst);
                }));
            }

            for h in handles {
                h.join().unwrap();
            }

            drop(tx); // disconnect sender - doubles as exit signal to storage thread

            return match file_writer.join() {
                Ok(c) => c,
                Err(_) => -1,
            };
        });

        return count;
    }
}
