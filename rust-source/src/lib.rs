pub mod crypto;
pub use crypto::*;

use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn rust_hello_world(s: *const c_char) {
    let cstr = unsafe { CStr::from_ptr(s) };

    match cstr.to_str() {
        Ok(s) => println!("[rust] {}", s),
        Err(_) => println!("[rust] oh no!"),
    }
}
