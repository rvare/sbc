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
