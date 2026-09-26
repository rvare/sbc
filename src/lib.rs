pub mod scan;

use std::process;
use std::thread;
use std::env;
use std::iter::Skip;

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
    pub file_path: Option<String>,
    pub num_threads: usize
}

impl Parameters {
    pub fn new() -> Parameters {
	Parameters {
	    file_path: None,
	    num_threads: match thread::available_parallelism() {
		Ok(non_zero) => non_zero.get(),
		Err(why) => {
		    eprintln!("{}", why);
		    process::exit(1);
		}
	    }
	}
    }
}

pub fn parse_cmd_parameters(args_iter: &mut Skip<env::Args>, params: &mut Parameters, available_threads: usize) {
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
                params.num_threads = match thread_quantity.parse::<usize>() {
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
            file_path_arg => params.file_path = Some(String::from(file_path_arg)),
        }
    }
}

pub fn show_help() {
    println!("Usage: sca [OPTIONS] [FILE]");
    println!("  --available-parallelism\n\tShows how many threads are available for true parallelism.");
    println!("  -h, --help\n\tShow this help");
    println!("  -t, --threads\n\tHow many threads to use (default 4)");
}

