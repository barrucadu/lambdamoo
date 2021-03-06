extern crate libc;

use std::ffi::CStr;
use std::cmp::{Ordering, min};

use memory::rs_str_to_c_str;

/// A type for ASCII strings.
#[derive(Clone, Copy, Debug)]
pub struct AsciiString<'a>(pub &'a str);

/// Lexicographic comparison of two ASCII strings, optionally ignoring
/// case.  If the two strings are unequal lengths, only the initial
/// portions are compared.
pub fn mystrcasecmp(
    ascii_str1: AsciiString,
    ascii_str2: AsciiString,
    case_counts: bool,
) -> Ordering {
    let AsciiString(str1) = ascii_str1;
    let AsciiString(str2) = ascii_str2;
    let len = min(str1.len(), str2.len());

    mystrncasecmp(ascii_str1, ascii_str2, len, case_counts)
}

/// Lexicographic comparison of a prefix of two ASCII strings,
/// optionally ignoring case.  If either string is shorter than the
/// limit, then the comparison is only done up to that point.
pub fn mystrncasecmp(
    ascii_str1: AsciiString,
    ascii_str2: AsciiString,
    len: usize,
    case_counts: bool,
) -> Ordering {
    let AsciiString(str1) = ascii_str1;
    let AsciiString(str2) = ascii_str2;
    let upper_bound = min(len, min(str1.len(), str2.len()));
    let prefix1 = &str1[..upper_bound];
    let prefix2 = &str2[..upper_bound];

    if case_counts {
        prefix1.cmp(prefix2)
    } else {
        prefix1.to_ascii_uppercase().cmp(
            &prefix2.to_ascii_uppercase(),
        )
    }
}

/// Find the index of the first occurrence of the second string in the
/// first.
pub fn strindex(
    ascii_haystack: AsciiString,
    ascii_needle: AsciiString,
    case_counts: bool,
) -> Option<usize> {
    let AsciiString(haystack) = ascii_haystack;
    let AsciiString(needle) = ascii_needle;

    if case_counts {
        haystack.find(needle)
    } else {
        haystack.to_ascii_uppercase().find(
            needle
                .to_ascii_uppercase()
                .as_str(),
        )
    }
}

/// Find the index of the last occurrence of the second string in the
/// first.
pub fn strrindex(
    ascii_haystack: AsciiString,
    ascii_needle: AsciiString,
    case_counts: bool,
) -> Option<usize> {
    let AsciiString(haystack) = ascii_haystack;
    let AsciiString(needle) = ascii_needle;

    if case_counts {
        haystack.rfind(needle)
    } else {
        haystack.to_ascii_uppercase().rfind(
            needle
                .to_ascii_uppercase()
                .as_str(),
        )
    }
}

/// Replace all occurrences in `source` of `what` with `with`, from
/// left to right.
pub fn strsub(
    ascii_source: AsciiString,
    ascii_what: AsciiString,
    ascii_with: AsciiString,
    case_counts: bool,
) -> String {
    let AsciiString(source) = ascii_source;
    let AsciiString(what) = ascii_what;
    let AsciiString(with) = ascii_with;

    let mut it = source.chars().enumerate().peekable();
    let mut replaced = String::with_capacity(source.len());
    while let Some(&(i, c)) = it.peek() {
        if mystrcasecmp(AsciiString(&source[i..]), ascii_what, case_counts) == Ordering::Equal {
            replaced.push_str(with);
            for _ in 0..what.len() {
                it.next();
            }
        } else {
            replaced.push(c);
            it.next();
        }
    }
    replaced
}

/// Check if two ASCII strings are the same, ignoring case.
///
/// TODO: Remove.
#[no_mangle]
pub unsafe extern "C" fn old_mystrcasecmp(
    c_str1: *const libc::c_char,
    c_str2: *const libc::c_char,
) -> i32 {
    // convert into Rust types and call `mystrcasecmp`.
    let str1 = c_str_to_ascii_str(c_str1);
    let str2 = c_str_to_ascii_str(c_str2);
    match mystrcasecmp(str1, str2, false) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

/// Check if prefixes of two ASCII strings are the same, ignoring
/// case.
///
/// TODO: Remove.
#[no_mangle]
pub unsafe extern "C" fn old_mystrncasecmp(
    c_str1: *const libc::c_char,
    c_str2: *const libc::c_char,
    len: i32,
) -> i32 {
    // convert into Rust types and call `mystrcasecmp`.
    let str1 = c_str_to_ascii_str(c_str1);
    let str2 = c_str_to_ascii_str(c_str2);
    match mystrncasecmp(str1, str2, len as usize, false) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

/// Find the index of the first occurrence of the second string in the
/// first.
///
/// TODO: Remove.
#[no_mangle]
pub unsafe extern "C" fn old_strindex(
    c_haystack: *const libc::c_char,
    c_needle: *const libc::c_char,
    case_counts: i32,
) -> i32 {
    // convert into Rust types and call `strindex`.
    let haystack = c_str_to_ascii_str(c_haystack);
    let needle = c_str_to_ascii_str(c_needle);
    match strindex(haystack, needle, case_counts == 1) {
        Some(n) => n as i32 + 1,
        None => 0,
    }
}

/// Find the index of the last occurrence of the second string in the
/// first.
///
/// TODO: Remove.
#[no_mangle]
pub unsafe extern "C" fn old_strrindex(
    c_haystack: *const libc::c_char,
    c_needle: *const libc::c_char,
    case_counts: i32,
) -> i32 {
    // convert into Rust types and call `strindex`.
    let haystack = c_str_to_ascii_str(c_haystack);
    let needle = c_str_to_ascii_str(c_needle);
    match strrindex(haystack, needle, case_counts == 1) {
        Some(n) => n as i32 + 1,
        None => 0,
    }
}

/// Replace all occurrences in `source` of `what` with `with`, from
/// left to right.
///
/// TODO: Remove.
#[no_mangle]
pub unsafe extern "C" fn old_strsub(
    c_source: *const libc::c_char,
    c_what: *const libc::c_char,
    c_with: *const libc::c_char,
    case_counts: i32,
) -> *mut libc::c_char {
    // convert into Rust types and call `strsub`.
    let source = c_str_to_ascii_str(c_source);
    let what = c_str_to_ascii_str(c_what);
    let with = c_str_to_ascii_str(c_with);
    let replaced = strsub(source, what, with, case_counts == 1);
    rs_str_to_c_str(replaced.as_str())
}


/// Helper function to convert a C-style string into an `AsciiString`.
///
/// TODO: Remove.
pub unsafe fn c_str_to_ascii_str<'a>(ptr: *const libc::c_char) -> AsciiString<'a> {
    let rs_str = CStr::from_ptr(ptr).to_str().unwrap();
    AsciiString(rs_str)
}

#[cfg(test)]
mod test {
    use ascii_string::*;

    #[test]
    fn example_strcasecmp() {
        let examples: &[(&str, &str, Ordering)] = &[
            ("", "", Ordering::Equal),
            ("hello", "", Ordering::Equal),
            ("", "world", Ordering::Equal),
            ("hello", "world", Ordering::Less),
            ("hello world", "HELLO", Ordering::Equal),
            ("HELLO", "hello world", Ordering::Equal),
            ("foo1", "FOO2", Ordering::Less),
            ("foo2", "FOO1", Ordering::Greater),
        ];

        for (index, (str1, str2, expected)) in examples.iter().enumerate() {
            let actual = mystrcasecmp(AsciiString(str1), AsciiString(str2), false);
            assert_eq!(
                actual,
                *expected,
                "example: {} ({} ~=~ {})",
                index,
                str1,
                str2
            );
        }
    }

    #[test]
    fn example_strncasecmp() {
        let examples: &[(&str, &str, usize, Ordering)] =
            &[
                ("", "", 100, Ordering::Equal),
                ("hello", "", 100, Ordering::Equal),
                ("", "world", 100, Ordering::Equal),
                ("hello", "world", 0, Ordering::Equal),
                ("hello", "world", 100, Ordering::Less),
                ("hello world", "HELLO", 1, Ordering::Equal),
                ("hello world", "HELLO", 100, Ordering::Equal),
                ("HELLO", "hello world", 1, Ordering::Equal),
                ("HELLO", "hello world", 100, Ordering::Equal),
                ("foo1", "FOO2", 3, Ordering::Equal),
                ("foo1", "FOO2", 4, Ordering::Less),
                ("foo2", "FOO1", 3, Ordering::Equal),
                ("foo2", "FOO1", 4, Ordering::Greater),
            ];

        for (index, (str1, str2, len, expected)) in examples.iter().enumerate() {
            let actual = mystrncasecmp(AsciiString(str1), AsciiString(str2), *len, false);
            assert_eq!(
                actual,
                *expected,
                "example: {} ({} ~=~ {})",
                index,
                str1,
                str2
            );
        }
    }

    #[test]
    fn example_strindex() {
        let examples: &[(&str, &str, bool, Option<usize>)] = &[
            ("foobar", "", false, Some(0)),
            ("foobar", "r", false, Some(5)),
            ("foobar", "o", false, Some(1)),
            ("foobar", "x", false, None),
            ("foobar", "oba", false, Some(2)),
            ("Foobar", "foo", true, None),
        ];

        for (index, (str1, str2, case_counts, expected)) in examples.iter().enumerate() {
            let actual = strindex(AsciiString(str1), AsciiString(str2), *case_counts);
            assert_eq!(
                actual,
                *expected,
                "example: {} ({} ~=~ {})",
                index,
                str1,
                str2
            );
        }
    }

    #[test]
    fn example_strrindex() {
        let examples: &[(&str, &str, bool, Option<usize>)] = &[
            ("foobar", "", false, Some(6)),
            ("foobar", "r", false, Some(5)),
            ("foobar", "o", false, Some(2)),
            ("foobar", "x", false, None),
            ("foobar", "oba", false, Some(2)),
            ("Foobar", "foo", true, None),
        ];

        for (index, (str1, str2, case_counts, expected)) in examples.iter().enumerate() {
            let actual = strrindex(AsciiString(str1), AsciiString(str2), *case_counts);
            assert_eq!(
                actual,
                *expected,
                "example: {} ({} ~=~ {})",
                index,
                str1,
                str2
            );
        }
    }

    #[test]
    fn example_strsub() {
        let examples: &[(&str, &str, &str, bool, &str)] =
            &[
                ("%n is a fink.", "%n", "Fred", true, "Fred is a fink."),
                ("foobar", "OB", "b", true, "foobar"),
                ("foobar", "OB", "b", false, "fobar"),
            ];

        for (index, (source, what, with, case_counts, expected)) in examples.iter().enumerate() {
            let actual = strsub(
                AsciiString(source),
                AsciiString(what),
                AsciiString(with),
                *case_counts,
            );
            assert_eq!(actual, *expected, "example: {} ({})", index, what);
        }
    }
}
