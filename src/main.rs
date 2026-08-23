use std::str::Chars;
use std::{env, fs};

struct Counters {
    num_statements: u32,
    num_blocks: u32,
}

fn main() {
    let mut counter = Counters {
        num_statements: 0,
        num_blocks: 0,
    };

    let mut args_iter = env::args().skip(1);
    let Some(file_path) = args_iter.next() else {
        panic!("Couldn't get next arg!");
    };

    println!("{}", file_path);

    let src_content: String = match fs::read_to_string(file_path) {
        Err(why) => panic!("Couldn't read file {}", why),
        Ok(contents) => contents,
    };

    scan_tokens(src_content, &mut counter);

    println!(
        "Approximate number of statements: {}",
        counter.num_statements
    );
    println!("Approximate number of blocks: {}", counter.num_blocks);
}

fn scan_tokens(src_content: String, counter: &mut Counters) {
    let mut token_iter = src_content.chars();
    while let Some(token) = token_iter.next() {
        match token {
            ';' => counter.num_statements += 1,
            '}' => counter.num_blocks += 1,
            '(' => skip_tokens(&mut token_iter, ')'),
            '/' => match token_iter.next() {
                Some('/') => skip_tokens(&mut token_iter, '\n'),
                Some('*') => skip_tokens(&mut token_iter, '/'),
                _ => {}
            },
            '\"' => skip_tokens(&mut token_iter, '\"'),
            _ => {}
        }
    }
}

fn skip_tokens(token_iter: &mut Chars, end_token: char) {
    while let Some(token) = token_iter.next() {
        if token == end_token {
            return;
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn skip_strings_in_parens() {
	let test_str = String::from("(String inside of parentheses.)");
	let mut iter = test_str.chars();
	skip_tokens(&mut iter, ')');
	assert_eq!(iter.count(), 0, "Parens Chars iterator did not consume all of string");
    }

    #[test]
    fn skip_inline_comments() {
	let test_str = String::from("// This is an inline comment string");
	let mut iter = test_str.chars();
	skip_tokens(&mut iter, '\n');
	assert_eq!(iter.count(), 0, "In-line Chars iterator did not consume all of string");
    }

    #[test]
    fn skip_double_quotes() {
	let test_str1 = String::from("\"Here is a double quote string.\"");
	let mut iter1 = test_str1.chars();
	iter1.next();
	skip_tokens(&mut iter1, '\"');
	assert_eq!(iter1.count(), 0, "Double quote Chars iterator did not consume all of string");

	let test_str2 = String::from("\"Here is a string with 'single quotes' and \"double quotes\" in it.\"");
	let mut iter2 = test_str2.chars();
	iter2.next();
	skip_tokens(&mut iter2, '\"');
	// assert_eq!(iter2.count(), 0, "Second double quote Chars iterator did not consume all of string.");
    }

    #[test]
    fn skip_multiline_comment() {
	let multiline_comment1 = String::from("/* Here is a multiline comment but single line. */");
	let mut iter1 = multiline_comment1.chars();
	iter1.next();
	iter1.next();
	skip_tokens(&mut iter1, '/');
	assert_eq!(iter1.count(), 0, "First iterator for multiline comment did not consume all tokens.");

	let multiline_comment2 = String::from("/* Line 1.\nLine 2.*/");
	let mut iter2 = multiline_comment2.chars();
	iter2.next();
	iter2.next();
	skip_tokens(&mut iter2, '/');
	assert_eq!(iter2.count(), 0, "Second iterator for multiline comment did not consume all tokens.");

	let multiline_comment3 = String::from("/* Here is a multiline comment / doc.\nMore lines.*/");
	let mut iter3 = multiline_comment3.chars();
	iter3.next();
	iter3.next();
	skip_tokens(&mut iter3, '/');
	// assert_eq!(iter3.count(), 0, "Third iterator for multiline comment did not consume all tokens.");
    }
}
