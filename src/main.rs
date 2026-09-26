use std::io::{BufRead, BufReader};
use std::process;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::{env, fs};
use sca;

fn main() {
    let mut args_iter = env::args().skip(1);
    let mut file_path: Option<String> = None;

    let available_threads: usize = match thread::available_parallelism() {
        Ok(non_zero) => non_zero.get(),
        Err(why) => {
            eprintln!("{}", why);
            process::exit(1);
        }
    };

    let mut num_threads = if available_threads > 4 { 4 } else { 1 };

    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                sca::show_help();
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

    let Some(file_path) = file_path else {
        eprintln!("No file given");
        process::exit(1);
    };

    let Ok(source_file) = fs::File::open(file_path) else {
        panic!("Couldn't get the file");
    };

    let shared_bufreader = Arc::new(Mutex::new(BufReader::new(source_file)));
    let (tx, rx): (Sender<sca::Counters>, Receiver<sca::Counters>) = mpsc::channel();
    let mut workers = vec![];
    for _ in 1..=num_threads {
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

                let delta: sca::Counters = sca::scan_tokens(&line);
                line.clear();

                if let Err(why) = thread_tx.send(delta) {
                    println!("{}", why);
                }
            } // end loop
        });

        workers.push(worker);
    }

    drop(tx);

    let mut counter = sca::Counters::new();
    for recieved in rx {
        counter.add_delta(recieved);
    }

    for worker in workers {
        if let Err(why) = worker.join() {
            println!("{:?}", why);
        }
    }

    println!("Approximate number of statements: {}", counter.num_statements);
    println!("Approximate number of blocks: {}", counter.num_blocks);
}


