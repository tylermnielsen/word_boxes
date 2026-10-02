use std::io::Write;
use std::time::Instant; 
use std::path::Path; 
use std::env; 
use std::fs::File; 


mod word_bank;

fn main() {
    let args: Vec<String> = env::args().collect(); 

    let bank_file = Path::new(&args[1]); // Path::new("data/common_words.txt");
    let size = args[2].parse::<usize>().unwrap(); 

    println!("Input file: {:?}", bank_file.display()); 
    println!("Word Box Size: {}", size); 

    println!("Starting test (3x3s)");

    let before = Instant::now(); 

    let data: word_bank::WordBank = word_bank::WordBank::new(
        bank_file, 
        size); 

    println!("Word bank size: {}", data.bank.len());

    let boxes: Vec<word_bank::WordBox> = data.find_boxes();

    if args.len() == 4 {
        let output_file = Path::new(&args[3]); 

        let mut file = match File::create(output_file) {
            Err(why) => panic!("couldn't open {}: {}", output_file.display(), why),
            Ok(file) => file,
        };

        for wb in &boxes {
            let _ = write!(file, "{}\n", wb).unwrap(); 
        }
    }

    println!("Elapsed time: {:.4?}", before.elapsed()); 
}
