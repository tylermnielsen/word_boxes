use std::fs::OpenOptions;
use std::io::Write;
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
    println!("Output file: {}", args[4]);
    println!("Target Size: {}x{}", width, height);

    let before = Instant::now();

    let data: word_bank::WordBank = word_bank::WordBank::new(bank_file, width, height);

    let thread_count = match thread::available_parallelism() {
        Ok(tc) => tc.get() - 1, // reserve 1 thread for storage
        Err(_) => 4,            // arbitrary default
    };

    println!("Word bank size: {}", data.bank.len());
    println!("Thread Count: {} (+1)", thread_count);
    println!(
        "Scanning {} possible iterations...",
        (data.bank.len() as u128).pow(height as u32)
    );

    let boxes_count = data.find_boxes_multithreaded(thread_count, &args[4]);

    let work_only_duration = before.elapsed();
    println!(
        "Elapsed time (without saving output): {:.4?}",
        work_only_duration
    );

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
        boxes_count,
        work_only_duration.as_secs_f64(),
        duration.as_secs_f64()
    )
    .unwrap();
}
