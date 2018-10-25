extern crate md5;

use std::os::raw::{c_char, c_int};
use std::slice;

/// Hash a sequence of bytes using MD5.
///
/// TODO: use a better hashing algorithm.
///
/// TODO: less C-ish function sig.
#[no_mangle]
pub extern "C" fn md5_bytes(source: *const c_char, length: c_int, destination: *mut [u8; 16]) {
    let bytes = unsafe { slice::from_raw_parts(source as *const u8, length as usize) };
    let md5::Digest(digest) = md5::compute(bytes);

    let destination: &mut [u8; 16] = unsafe { &mut *destination };
    for i in 0..16 {
        destination[i] = digest[i];
    }
}
