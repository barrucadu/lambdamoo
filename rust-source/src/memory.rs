extern crate libc;

/// Allocate memory, with space for a int immediately before the
/// returned pointer IFF the `memory_type` is float (12), string (5),
/// or list (7).
///
/// TODO: Phase this out in favour of `Rc<RefCell<_>>` as things are
/// ported to Rust.
///
/// TODO: Handle failed malloc in Rust when `panic` is ported.
#[no_mangle]
pub extern "C" fn almost_mymalloc(size: libc::size_t, memory_type: u32) -> *mut libc::c_void {
    let offset = refcount_offset(memory_type);
    let actual_size = if size == 0 { 1 } else { size } + offset;

    unsafe {
        let mut mem = libc::malloc(actual_size) as *mut u8;

        if mem.is_null() {
            return mem as *mut libc::c_void;
        }

        if offset > 0 {
            mem = mem.offset(offset as isize);
            *((mem as *mut i32).offset(-1)) = 1
        }

        mem as *mut libc::c_void
    }
}

/// Reallocate memory, preserving the refcount part.
///
/// TODO: Phase this out as things are ported to Rust.
///
/// TODO: Handle failed realloc in Rust when `panic` is ported.
#[no_mangle]
pub extern "C" fn almost_myrealloc(
    ptr: *mut libc::c_void,
    size: libc::size_t,
    memory_type: u32,
) -> *mut libc::c_void {
    let offset = refcount_offset(memory_type);
    let actual_size = size + offset;

    unsafe {
        let orig = (ptr as *mut u8).offset(-(offset as isize)) as *mut libc::c_void;
        let new = libc::realloc(orig, actual_size);
        if new.is_null() {
            return new;
        }
        (new as *mut u8).offset(offset as isize) as *mut libc::c_void
    }
}

/// Free memory allocated by `almost_mymalloc`.
#[no_mangle]
pub extern "C" fn myfree(ptr: *mut libc::c_void, memory_type: u32) {
    let offset = refcount_offset(memory_type);
    unsafe { libc::free(ptr.offset(-(offset as isize))) }
}

/// Duplicate a string.
///
/// TODO: Intern empty strings.
///
/// TODO: Handle failed malloc in Rust when `panic` is ported.
#[no_mangle]
pub extern "C" fn str_dup(src: *const libc::c_char) -> *mut libc::c_char {
    let strlen = if src.is_null() {
        0
    } else {
        unsafe { libc::strlen(src) }
    };

    let dst = almost_mymalloc(strlen + 1, 5) as *mut libc::c_char;

    if !dst.is_null() {
        unsafe { libc::strcpy(dst, src) };
    }

    dst
}

/// Calculate space for the ref counting cell.
///
/// Only floats (12), strings (5), and lists (7) get space for a
/// refcount.  Element alignment is preserved, even though the
/// refcount is a `u32`.
fn refcount_offset(memory_type: u32) -> usize {
    match memory_type {
        5 => 4, // sizeof(int)
        7 => 8, // sizeof(Var*)
        12 => 8, // sizeof(double)
        _ => 0,
    }
}
