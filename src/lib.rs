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
