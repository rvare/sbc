use crate::Counters;
use std::str::Chars;

pub fn scan_tokens(src_content: &String) -> Counters {
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

pub fn skip_tokens(token_iter: &mut Chars, end_token: char) {
    while let Some(token) = token_iter.next() {
        if token == end_token {
            return;
        } else if token == '\\' {
            // Handles escape characters.
            token_iter.next();
        }
    }
}

pub fn skip_multiline_comment(token_iter: &mut Chars) {
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
