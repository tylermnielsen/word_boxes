use std::time::Instant; 
use std::path::Path; 
use std::io::{self, Write};
use crossterm::{ExecutableCommand, terminal, cursor}; 

mod word_bank;

fn main() {
    println!("Starting test (3x3s)");

    let before = Instant::now(); 

    let data: word_bank::WordBank = word_bank::WordBank::new(Path::new("data/words_alpha.txt"), 3); 

    println!("Word bank size: {}", data.bank.len());


    println!("Elapsed time: {:.4?}", before.elapsed()); 
}
