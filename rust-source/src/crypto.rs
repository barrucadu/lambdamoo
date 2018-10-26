extern crate libc;
extern crate md5;

use std::slice;

use memory;

/// Hash a sequence of bytes using MD5.
///
/// TODO: Use a better hashing algorithm.
pub fn hash_bytes(bytes: &[u8]) -> [u8; 32] {
    let md5::Digest(digest) = md5::compute(bytes);

    let digits: [u8; 16] = *b"0123456789ABCDEF";
    let mut answer: [u8; 32] = *b"12345678901234567890123456789012";
    for i in 0..16 {
        answer[i * 2] = digits[(digest[i] >> 4) as usize];
        answer[i * 2 + 1] = digits[(digest[i] & 0xF) as usize];
    }

    answer
}

/// Hash a sequence of bytes using MD5.
///
/// TODO: Remove.
#[no_mangle]
pub extern "C" fn old_hash_bytes(
    source: *const libc::c_char,
    length: libc::c_int,
) -> *const libc::c_char {
    // convert into Rust types and call `hash_bytes`.
    unsafe {
        let bytes = slice::from_raw_parts(source as *const u8, length as usize);

        let answer = hash_bytes(bytes);

        // return the answer as a reference-counted string
        memory::str_dup_n(answer.as_ptr() as *const libc::c_char, answer.len())
    }
}

#[cfg(test)]
mod test {
    use crypto::*;

    #[test]
    fn example_string_hashes() {
        let examples: &[(&[u8], &[u8])] =
            &[
                (b"LambdaMOO in Rust!", b"8B9FDA0816E85ABE884E8499A7DAFFC9"),
                (b"", b"D41D8CD98F00B204E9800998ECF8427E"),
                (b" ", b"7215EE9C7D9DC229D2921A40E899EC5F"),
                (b"a", b"0CC175B9C0F1B6A831C399E269772661"),
                (b"ab", b"187EF4436122D1CC2F40DC2B92F0EBA0"),
                (b"abc", b"900150983CD24FB0D6963F7D28E17F72"),
                (
                    b"!$%^&*()[]{};:'@#~<>?,./",
                    b"68FE77CC8D421E1D9B153307FC5F57BC",
                ),
                (b"0123456789abcdef", b"4032AF8D61035123906E58E067140CC5"),
                (
                    b"0123456789abcdef----------------",
                    b"E27DC0D2F2DEA01821C3EC4BACE756FC",
                ),
                (
                    b"01234567890123456789012345678901234567890123456789",
                    b"BAED005300234F3D1503C50A48CE8E6F",
                ),
            ];

        for (index, (original, expected_hash)) in examples.iter().enumerate() {
            let actual_hash = hash_bytes(original);
            assert_eq!(actual_hash, *expected_hash, "example: {}", index);
        }
    }
}
