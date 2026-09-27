use sca;
use std::env;
use std::process;
use std::thread;

fn main() {
    let mut args_iter = env::args().skip(1);

    let available_threads: usize = match thread::available_parallelism() {
        Ok(non_zero) => non_zero.get(),
        Err(why) => {
            eprintln!("{}", why);
            process::exit(1);
        }
    };

    let params = sca::parse_cmd_parameters(&mut args_iter, available_threads);
    let counter = sca::process_source(params);

    println!("Number of statements: {}", counter.num_statements);
    println!("Number of blocks: {}", counter.num_blocks);
}
