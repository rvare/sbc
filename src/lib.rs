pub mod scan;

use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::iter::Skip;
use std::process;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Counters {
    pub num_statements: u32,
    pub num_blocks: u32,
}

impl Counters {
    pub fn new() -> Counters {
        Counters {
            num_statements: 0,
            num_blocks: 0,
        }
    }

    pub fn add_delta(&mut self, delta: Counters) {
        self.num_statements += delta.num_statements;
        self.num_blocks += delta.num_blocks;
    }
}

pub struct Parameters {
    pub source_file: fs::File,
    pub num_threads: usize,
}

pub fn parse_cmd_parameters(
    args_iter: &mut Skip<env::Args>,
    available_threads: usize,
) -> Parameters {
    let mut file_path: Option<String> = None;
    let mut num_threads: usize = available_threads;

    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                show_help();
                process::exit(0);
            }
            "-t" | "--threads" => {
                let Some(thread_quantity) = args_iter.next() else {
                    eprintln!("No quantity given");
                    process::exit(1);
                };
                num_threads = match thread_quantity.parse::<usize>() {
                    Ok(count) if count > 0 => count,
                    Ok(_) => {
                        eprintln!("Number of threads must be strictly greater than zero.");
                        process::exit(1);
                    }
                    Err(why) => {
                        eprintln!("{}", why);
                        process::exit(1);
                    }
                };
            }
            "--available-parallelism" => {
                println!("Available threads for parallelism: {available_threads}.");
                process::exit(0);
            }
            file_path_arg => file_path = Some(String::from(file_path_arg)),
        }
    }

    // Checks to see if file_path is None, unwraps string from Some, and shadows the old file_path variable.
    let Some(file_path) = file_path else {
        eprintln!("No file given");
        process::exit(1);
    };

    let Ok(source_file) = fs::File::open(&file_path) else {
        eprintln!("Could not get give file located at {}", file_path);
        process::exit(1);
    };

    Parameters {
        source_file,
        num_threads,
    }
}

pub fn show_help() {
    println!("Usage: sca [OPTIONS] [FILE]");
    println!(
        "  --available-parallelism\n\tShows how many threads are available for true parallelism."
    );
    println!("  -h, --help\n\tShow this help");
    println!("  -t, --threads\n\tHow many threads to use (default 4)");
}

pub fn process_source(params: Parameters) -> Counters {
    let shared_bufreader = Arc::new(Mutex::new(BufReader::new(params.source_file)));
    let (tx, rx): (mpsc::Sender<Counters>, mpsc::Receiver<Counters>) = mpsc::channel();
    let mut workers = vec![];
    for _ in 1..=params.num_threads {
        let clone_bufreader = Arc::clone(&shared_bufreader);
        let thread_tx = tx.clone();
        let worker = thread::spawn(move || {
            let mut line = String::new();
            loop {
                let mut bf_reader = match clone_bufreader.lock() {
                    Ok(bf_reader) => bf_reader,
                    Err(p_err) => p_err.into_inner(), // Should allow us to recover the BufReader from a panicked thread.
                };

                match bf_reader.read_line(&mut line) {
                    Ok(0) => break, // Breaks out of loop, not the block.
                    Err(why) => panic!("{}", why),
                    _ => {}
                }

                drop(bf_reader);

                let delta: Counters = scan::scan_tokens(&line);
                line.clear();

                if let Err(why) = thread_tx.send(delta) {
                    println!("{}", why);
                }
            } // end loop
        });

        workers.push(worker);
    }

    drop(tx);

    let mut counter = Counters::new();
    for recieved in rx {
        counter.add_delta(recieved);
    }

    for worker in workers {
        if let Err(why) = worker.join() {
            eprintln!("{:?}", why);
        }
    }

    counter
}
