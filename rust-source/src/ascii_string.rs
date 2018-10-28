extern crate libc;

use std::ffi::CStr;
use std::cmp::{Ordering, min};

/// A type for ASCII strings.
pub struct AsciiString<'a>(pub &'a str);

/// Lexicographic comparison of two ASCII strings, ignoring case.  If
/// the two strings are unequal lengths, only the initial portions are
/// compared.
pub fn mystrcasecmp(ascii_str1: AsciiString, ascii_str2: AsciiString) -> Ordering {
    let AsciiString(str1) = ascii_str1;
    let AsciiString(str2) = ascii_str2;
    let len = min(str1.len(), str2.len());
    mystrncasecmp(ascii_str1, ascii_str2, len)
}

/// Lexicographic comparison of a prefix of two ASCII strings,
/// ignoring case.  If either string is shorter than the limit, then
/// the comparison is only done up to that point.
pub fn mystrncasecmp(ascii_str1: AsciiString, ascii_str2: AsciiString, len: usize) -> Ordering {
    let AsciiString(str1) = ascii_str1;
    let AsciiString(str2) = ascii_str2;
    let upper_bound = min(len, min(str1.len(), str2.len()));
    let prefix1 = &str1[..upper_bound];
    let prefix2 = &str2[..upper_bound];
    prefix1.to_ascii_uppercase().cmp(
        &prefix2.to_ascii_uppercase(),
    )
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

/// Check if two ASCII strings are the same, ignoring case.
///
/// TODO: Remove.
#[no_mangle]
pub extern "C" fn old_mystrcasecmp(str1: *const libc::c_char, str2: *const libc::c_char) -> i32 {
    // convert into Rust types and call `mystrcasecmp`.
    unsafe {
        let rust_str1 = CStr::from_ptr(str1).to_str().unwrap();
        let rust_str2 = CStr::from_ptr(str2).to_str().unwrap();
        match mystrcasecmp(AsciiString(rust_str1), AsciiString(rust_str2)) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }
}

/// Check if prefixes of two ASCII strings are the same, ignoring
/// case.
///
/// TODO: Remove.
#[no_mangle]
pub extern "C" fn old_mystrncasecmp(
    str1: *const libc::c_char,
    str2: *const libc::c_char,
    len: i32,
) -> i32 {
    // convert into Rust types and call `mystrcasecmp`.
    unsafe {
        let rust_str1 = CStr::from_ptr(str1).to_str().unwrap();
        let rust_str2 = CStr::from_ptr(str2).to_str().unwrap();
        match mystrncasecmp(AsciiString(rust_str1), AsciiString(rust_str2), len as usize) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }
}

/// Find the index of the first occurrence of the second string in the
/// first.
///
/// TODO: Remove.
#[no_mangle]
pub extern "C" fn old_strindex(
    haystack: *const libc::c_char,
    needle: *const libc::c_char,
    case_counts: i32,
) -> i32 {
    // convert into Rust types and call `strindex`.
    unsafe {
        let rust_haystack = CStr::from_ptr(haystack).to_str().unwrap();
        let rust_needle = CStr::from_ptr(needle).to_str().unwrap();
        match strindex(
            AsciiString(rust_haystack),
            AsciiString(rust_needle),
            case_counts == 1,
        ) {
            Some(n) => n as i32 + 1,
            None => 0,
        }
    }
}

/// Find the index of the last occurrence of the second string in the
/// first.
///
/// TODO: Remove.
#[no_mangle]
pub extern "C" fn old_strrindex(
    haystack: *const libc::c_char,
    needle: *const libc::c_char,
    case_counts: i32,
) -> i32 {
    // convert into Rust types and call `strindex`.
    unsafe {
        let rust_haystack = CStr::from_ptr(haystack).to_str().unwrap();
        let rust_needle = CStr::from_ptr(needle).to_str().unwrap();
        match strrindex(
            AsciiString(rust_haystack),
            AsciiString(rust_needle),
            case_counts == 1,
        ) {
            Some(n) => n as i32 + 1,
            None => 0,
        }
    }
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
            let actual = mystrcasecmp(AsciiString(str1), AsciiString(str2));
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
            let actual = mystrncasecmp(AsciiString(str1), AsciiString(str2), *len);
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
}
