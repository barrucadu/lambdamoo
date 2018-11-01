extern crate libc;

use std::mem::size_of;

use ascii_string::{AsciiString, c_str_to_ascii_str};
use memory::{MemoryType, almost_mymalloc, rs_str_to_c_str};

/// Split a string into space-separated words, where:
///
/// - Leading spaces are ignored: ` hello world` is [`hello`,`world`]
/// - Multiple spaces are merged: `hello  world` is [`hello`,`world`]
/// - Words can be quoted: `hello "world foo" bar` is [`hello`,`world foo`,`bar`]
/// - A backslash treats the next character literally: `hello\ world` is [`hello world`]
pub fn parse_into_words(ascii_input: AsciiString) -> Vec<String> {
    // 50 words is (probably) enough for anybody!
    let mut words = Vec::with_capacity(50);

    let AsciiString(input) = ascii_input;
    let mut it = input.chars().peekable();

    let mut in_quotes = false;
    let mut has_word = false;
    let mut current = String::with_capacity(input.len());
    while let Some(&c) = it.peek() {
        match c {
            // word boundary
            ' ' if !in_quotes => {
                if has_word {
                    words.push(current);
                    current = String::with_capacity(input.len());
                    has_word = false;
                }
                while let Some(' ') = it.peek() {
                    it.next();
                }
            }
            // escape
            '\\' => {
                it.next();
                if let Some(&next_c) = it.peek() {
                    has_word = true;
                    current.push(next_c);
                }
                it.next();
            }
            '"' => {
                in_quotes = !in_quotes;
                it.next();
            }
            _ => {
                has_word = true;
                current.push(c);
                it.next();
            }
        }
    }
    if has_word {
        words.push(current);
    }

    words
}

/// Split a string into space-separated words.
///
/// TODO: Remove.
#[no_mangle]
pub unsafe extern "C" fn old_parse_into_words(
    c_input: *const libc::c_char,
    nwords: *mut libc::c_int,
) -> *mut *mut libc::c_char {
    // convert into Rust types and call `parse_into_words`.
    let input = c_str_to_ascii_str(c_input);
    let words = parse_into_words(input);
    let words_vec: Vec<*mut libc::c_char> =
        words.iter().map(|s| rs_str_to_c_str(s.as_str())).collect();
    *nwords = words.len() as libc::c_int;
    let actual_size = size_of::<*mut libc::c_char>() * words.len();
    let out = almost_mymalloc(actual_size, MemoryType::M_STRING_PTRS);
    if !out.is_null() {
        libc::memcpy(
            out as *mut libc::c_void,
            words_vec.as_slice().as_ptr() as *const libc::c_void,
            actual_size,
        );
    }
    out as *mut *mut libc::c_char
}

#[cfg(test)]
mod test {
    use parse_cmd::*;

    #[test]
    fn example_words() {
        let examples = [
            ("hello", vec!["hello"]),
            ("hello world", vec!["hello", "world"]),
            ("     hello world", vec!["hello", "world"]),
            ("hello      world", vec!["hello", "world"]),
            ("hello world     ", vec!["hello", "world"]),
            ("\"hello world\"", vec!["hello world"]),
            ("\"hello     world\"", vec!["hello     world"]),
            ("\\ hello world", vec![" hello", "world"]),
            ("\\\\hello world", vec!["\\hello", "world"]),
            ("\\hello world", vec!["hello", "world"]),
            ("\"hello\\\"world\"", vec!["hello\"world"]),
        ];

        for (index, (original, expected_words)) in examples.iter().enumerate() {
            let actual_words = parse_into_words(AsciiString(original));
            assert_eq!(actual_words, *expected_words, "example: {}", index);
        }
    }
}
