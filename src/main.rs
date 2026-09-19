use std::io::{BufRead, BufReader};
use std::str::Chars;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::{env, fs};
use std::process;

struct Counters {
    num_statements: u32,
    num_blocks: u32,
}

impl Counters {
    fn new() -> Counters {
        Counters {
            num_statements: 0,
            num_blocks: 0,
        }
    }

    fn add_delta(&mut self, delta: Counters) {
        self.num_statements += delta.num_statements;
        self.num_blocks += delta.num_blocks;
    }
}

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
		show_help();
		process::exit(0);
	    },
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
		    },
		    Err(why) => {
			eprintln!("{}", why);
			process::exit(1);
		    }
		};
	    },
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
    let (tx, rx): (Sender<Counters>, Receiver<Counters>) = mpsc::channel();
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

                let delta: Counters = scan_tokens(&line);
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
            println!("{:?}", why);
        }
    }

    println!("Approximate number of statements: {}", counter.num_statements);
    println!("Approximate number of blocks: {}", counter.num_blocks);
}

fn scan_tokens(src_content: &String) -> Counters {
    let mut delta = Counters::new();

    let mut token_iter = src_content.chars();
    while let Some(token) = token_iter.next() {
        match token {
            ';' => delta.num_statements += 1,
            '}' => delta.num_blocks += 1,
            '(' => skip_tokens(&mut token_iter, ')'),
            '/' => match token_iter.next() {
                Some('/') => skip_tokens(&mut token_iter, '\n'),
                Some('*') => skip_multiline_comment(&mut token_iter),
                _ => {}
            },
            '\"' => skip_tokens(&mut token_iter, '\"'),
            _ => {}
        }
    }

    delta
}

fn skip_tokens(token_iter: &mut Chars, end_token: char) {
    while let Some(token) = token_iter.next() {
        if token == end_token {
            return;
        } else if token == '\\' {
            // Handles escape characters.
            token_iter.next();
        }
    }
}

fn skip_multiline_comment(token_iter: &mut Chars) {
    while let Some(token) = token_iter.next() {
        match token {
            '*' => {
                while let Some(star) = token_iter.next() {
                    if star == '/' {
                        return;
                    }
                }
            }
            _ => {} // Any character within the multiline comment.
        }
    }
}

fn show_help() {
    println!("Usage: sca [OPTIONS] [FILE]");
    println!("  -h, --help\n\tShow this help");
    println!("  -t, --threads\n\tHow many threads to use (default 4)");
}

#[cfg(test)]
pub mod tests {
    use super::*;

    // skip_tokens tests.
    #[test]
    fn skip_strings_in_parens() {
        let test_str = String::from("(String inside of parentheses.)");
        let mut iter = test_str.chars();
        skip_tokens(&mut iter, ')');
        assert_eq!(
            iter.count(),
            0,
            "Parens Chars iterator did not consume all of string"
        );
    }

    #[test]
    fn skip_inline_comments() {
        let test_str = String::from("// This is an inline comment string");
        let mut iter = test_str.chars();
        skip_tokens(&mut iter, '\n');
        assert_eq!(
            iter.count(),
            0,
            "In-line Chars iterator did not consume all of string"
        );
    }

    #[test]
    fn skip_double_quotes() {
        let test_str1 = String::from("\"Here is a double quote string.\"");
        let mut iter1 = test_str1.chars();
        iter1.next();
        skip_tokens(&mut iter1, '\"');
        assert_eq!(
            iter1.count(),
            0,
            "Double quote Chars iterator did not consume all of string"
        );

        let test_str2 = String::from(
            "\"Here is a string with 'single quotes' and \\\"double quotes\\\" in it.\"",
        );
        let mut iter2 = test_str2.chars();
        iter2.next();
        skip_tokens(&mut iter2, '\"');
        let clone_iter = iter2.clone();
        println!("{}", clone_iter.as_str());
        assert_eq!(
            iter2.count(),
            0,
            "Second double quote Chars iterator did not consume all of string."
        );
    }

    #[test]
    fn skip_tokens_in_multiline_comment() {
        let multiline_comment1 = String::from("/* Here is a multiline comment but single line. */");
        let mut iter1 = multiline_comment1.chars();
        iter1.next();
        iter1.next();
        skip_multiline_comment(&mut iter1);
        assert_eq!(
            iter1.count(),
            0,
            "First iterator for multiline comment did not consume all tokens."
        );

        let multiline_comment2 = String::from("/* Line 1.\nLine 2.*/");
        let mut iter2 = multiline_comment2.chars();
        iter2.next();
        iter2.next();
        skip_multiline_comment(&mut iter2);
        assert_eq!(
            iter2.count(),
            0,
            "Second iterator for multiline comment did not consume all tokens."
        );

        let multiline_comment3 =
            String::from("/* Here is a multiline comment / doc.\nMore lines.*/");
        let mut iter3 = multiline_comment3.chars();
        iter3.next();
        iter3.next();
        skip_multiline_comment(&mut iter3);
        assert_eq!(
            iter3.count(),
            0,
            "Third iterator for multiline comment did not consume all tokens."
        );
    }
}
