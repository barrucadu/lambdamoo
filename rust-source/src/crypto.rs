extern crate libc;
extern crate md5;

use std::slice;

use memory;

/// Hash a sequence of bytes using MD5.
///
/// TODO: use a better hashing algorithm.
///
/// TODO: less C-ish function sig.
#[no_mangle]
pub extern "C" fn hash_bytes(
    source: *const libc::c_char,
    length: libc::c_int,
) -> *const libc::c_char {
    let bytes = unsafe { slice::from_raw_parts(source as *const u8, length as usize) };
    let md5::Digest(digest) = md5::compute(bytes);

    let digits: [u8; 16] = *b"0123456789ABCDEF";
    let mut answer: [u8; 33] = *b"12345678901234567890123456789012\0";
    for i in 0..16 {
        answer[i * 2] = digits[(digest[i] >> 4) as usize];
        answer[i * 2 + 1] = digits[(digest[i] & 0xF) as usize];
    }

    // return the answer as a reference-counted string
    memory::str_dup(answer.as_ptr() as *const libc::c_char)
}
