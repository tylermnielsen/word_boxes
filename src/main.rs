use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::Instant;
use std::{env, thread};


mod word_bank;
mod word_box;

fn main() {
    let args: Vec<String> = env::args().collect();

    let bank_file = Path::new(&args[1]);
    let width = args[2].parse::<usize>().unwrap();
    let height = args[3].parse::<usize>().unwrap();

    println!("Input file: {:?}", bank_file.display());
    if args.len() == 5 {
        println!("Output file: {}", args[4]);
    } else {
        println!("No output file");
    }
    println!("Target Size: {}x{}", width, height);

    let before = Instant::now();

    let data: word_bank::WordBank = word_bank::WordBank::new(bank_file, width, height);

    let thread_count = match thread::available_parallelism() {
        Ok(tc) => tc.get(),
        Err(_) => 4, // arbitrary default
    };

    println!("Word bank size: {}", data.bank.len());
    println!("Thread Count: {}", thread_count);
    println!(
        "Scanning {} possible iterations...",
        (data.bank.len() as u128).pow(height as u32)
    );

    // let boxes: Vec<word_box::WordBox> = data.find_boxes(0, data.bank.len());
    let boxes = data.find_boxes_multithreaded(thread_count);

    let work_only_duration = before.elapsed();
    println!(
        "Elapsed time (without saving output): {:.4?}",
        work_only_duration
    );

    if args.len() == 5 {
        let output_file = Path::new(&args[4]);

        let file = match File::create(output_file) {
            Err(why) => panic!("couldn't open {}: {}", output_file.display(), why),
            Ok(file) => file,
        };

        let mut writer = BufWriter::new(file);

        let mut temp = String::new(); 
        for wb in &boxes {
            for word in &wb.letters {
                temp += word.iter().collect::<String>().as_str();
                temp += "\n";
            }
            writeln!(writer, "{}", temp).unwrap();
            temp.clear(); 
        }
    } else {
        for wb in &boxes {
            println!("{}\n", wb);
        }
    }

    let duration = before.elapsed();
    println!("Elapsed time: {:.4?}", duration);

    let stats_path = Path::new("runs.csv");

    let exists = stats_path.try_exists().unwrap();

    let mut file = match OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(stats_path)
    {
        Err(why) => panic!("couldn't open {}: {}", "runs.csv", why),
        Ok(file) => file,
    };

    if !exists {
        let _ = write!(
            file,
            "size,source,bank_size,found,work only duration(s),duration(s)\n"
        )
        .unwrap();
    }

    let _ = write!(
        file,
        "{}x{},{},{},{},{},{}\n",
        width,
        height,
        bank_file.display(),
        data.bank.len(),
        boxes.len(),
        work_only_duration.as_secs_f64(),
        duration.as_secs_f64()
    )
    .unwrap();
}
