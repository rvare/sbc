use std::{env, fs, io, process};
use std::str::Chars;

struct Counters {
    num_statements: u32,
    num_blocks: u32
}

fn main() {
    let mut counter = Counters{ num_statements: 0, num_blocks: 0 };
    
    let mut args_iter = env::args().skip(1);
    let Some(file_path) = args_iter.next() else {
	panic!("Couldn't get next arg!");
    };
    
    println!("{}", file_path);

    let src_content = match fs::read_to_string(file_path) {
	Err(why) => panic!("Couldn't read file {}", why),
	Ok(contents) => contents
    };

    scan_tokens(src_content, &mut counter);

    println!("Approximate number of statements: {}", counter.num_statements);
    println!("Approximate number of blocks: {}", counter.num_blocks);
}

fn scan_tokens(src_content: String, counter: &mut Counters) {
    let mut token_iter = src_content.chars();
    while let Some(token) = token_iter.next() {
	match token {
	    ';' => counter.num_statements += 1,
	    '}' => counter.num_blocks += 1,
	    '(' => skip_tokens(&mut token_iter, ')'),
	    '/' => {
		match token_iter.next() {
		    Some('/') => skip_tokens(&mut token_iter, '\n'),
		    Some('*') => skip_tokens(&mut token_iter, '/'),
		    _ => {}
		}
	    },
	    '\"' => skip_tokens(&mut token_iter, '\"'),
	    _ => {}
	}
    }
}

fn skip_tokens(token_iter: &mut Chars, end_token: char) {
    println!("Here for {}", end_token);
    while let Some(token) = token_iter.next() {
	if token == end_token {
	    return;
	}
    }
}
