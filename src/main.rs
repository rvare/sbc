use std::{env, fs, io, process};

fn main() {
    let mut num_statements: u32 = 0;
    let mut num_blocks: u32 = 0;
    
    let mut args_iter = env::args().skip(1);
    let Some(file_path) = args_iter.next() else {
	panic!("Couldn't get next arg!");
    };
    
    println!("{}", file_path);

    let src_content = match fs::read_to_string(file_path) {
	Err(why) => panic!("Couldn't read file {}", why),
	Ok(contents) => contents
    };

    // Call scan_tokens

    println!("Approximate number of statements: {}", num_statements);
    println!("Approximate number of blocks: {}", num_blocks);
    println!("{}", src_content);
}
