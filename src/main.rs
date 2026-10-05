use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::time::Instant;
use std::{env, thread};

mod word_bank;
mod word_box;

fn main() {
    let args: Vec<String> = env::args().collect();

    let bank_file = Path::new(&args[1]);
    let size = args[2].parse::<usize>().unwrap();

    println!("Input file: {:?}", bank_file.display());
    println!("Word Box Size: {}", size);

    println!("Starting ({}x{}s)", size, size);

    let before = Instant::now();

    let data: word_bank::WordBank = word_bank::WordBank::new(bank_file, size);

    let thread_count = match thread::available_parallelism() {
        Ok(tc) => tc.get(),
        Err(_) => 4, // arbitrary default
    };

    println!("Word bank size: {}", data.bank.len());
    println!("Thread Count: {}", thread_count);
    println!(
        "Scanning {} possible iterations...",
        (data.bank.len() as u128).pow(size as u32)
    );

    // let boxes: Vec<word_box::WordBox> = data.find_boxes(0, data.bank.len());
    let boxes = data.find_boxes_multithreaded(thread_count);

    let work_only_duration = before.elapsed();
    println!(
        "Elapsed time (without saving output): {:.4?}",
        work_only_duration
    );

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
        "{},{},{},{},{},{}\n",
        size,
        bank_file.display(),
        data.bank.len(),
        boxes.len(),
        work_only_duration.as_secs_f64(),
        duration.as_secs_f64()
    )
    .unwrap();
}
