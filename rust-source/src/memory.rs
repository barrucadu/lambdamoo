extern crate libc;

use std::ffi;

/// Types of memory.  Members use the original C names.
///
/// TODO: Remove.
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum MemoryType {
    M_AST_POOL,
    M_AST,
    M_PROGRAM,
    M_PVAL,
    M_NETWORK,
    M_STRING,
    M_VERBDEF,
    M_LIST,
    M_PREP,
    M_PROPDEF,
    M_OBJECT_TABLE,
    M_OBJECT,
    M_FLOAT,
    M_STREAM,
    M_NAMES,
    M_ENV,
    M_TASK,
    M_PATTERN,

    M_BYTECODES,
    M_FORK_VECTORS,
    M_LIT_LIST,
    M_PROTOTYPE,
    M_CODE_GEN,
    M_DISASSEMBLE,
    M_DECOMPILE,

    M_RT_STACK,
    M_RT_ENV,
    M_BI_FUNC_DATA,
    M_VM,

    M_REF_ENTRY,
    M_REF_TABLE,
    M_VC_ENTRY,
    M_VC_TABLE,
    M_STRING_PTRS,
    M_INTERN_POINTER,
    M_INTERN_ENTRY,
    M_INTERN_HUNK,

    Sizeof_Memory_Type,
}

/// Allocate memory, with space for a int immediately before the
/// returned pointer IFF the `memory_type` is `M_FLOAT`, `M_STRING`,
/// or `M_LIST`.
///
/// TODO: Phase this out in favour of `Rc<RefCell<_>>` as things are
/// ported to Rust.
///
/// TODO: Handle failed malloc in Rust when `panic` is ported.
#[no_mangle]
pub extern "C" fn almost_mymalloc(
    size: libc::size_t,
    memory_type: MemoryType,
) -> *mut libc::c_void {
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
    memory_type: MemoryType,
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
pub extern "C" fn myfree(ptr: *mut libc::c_void, memory_type: MemoryType) {
    let offset = refcount_offset(memory_type);
    unsafe { libc::free(ptr.offset(-(offset as isize))) }
}

/// Increment the reference count of something.
///
/// CAUTION: This will access memory out of bounds if it's not a
/// reference-countable type.  This does not check for overflow.
///
/// TODO: Remove
#[no_mangle]
pub extern "C" fn addref(ptr: *mut libc::c_void) -> i32 {
    unsafe {
        *((ptr as *mut i32).offset(-1)) += 1;
        refcount(ptr)
    }
}

/// Decrement the reference count of something.
///
/// CAUTION: This will access memory out of bounds if it's not a
/// reference-countable type.  This does not check for underflow.
///
/// TODO: Remove
#[no_mangle]
pub extern "C" fn delref(ptr: *mut libc::c_void) -> i32 {
    unsafe {
        *((ptr as *mut i32).offset(-1)) -= 1;
        refcount(ptr)
    }
}

/// Get the reference count of something.
///
/// CAUTION: This will access memory out of bounds if it's not a
/// reference-countable type.
///
/// TODO: Remove
#[no_mangle]
pub extern "C" fn refcount(ptr: *mut libc::c_void) -> i32 {
    unsafe { (ptr as *mut i32).offset(-1).read() }
}

/// Convert a Rust `&str` into a C string with refcount part.
///
/// TODO: Remove.
pub fn rs_str_to_c_str(src: &str) -> *mut libc::c_char {
    let c_string = ffi::CString::new(src).unwrap();
    str_dup(c_string.as_ptr())
}

/// Duplicate a string.
///
/// TODO: Remove.
#[no_mangle]
pub extern "C" fn str_dup(src: *const libc::c_char) -> *mut libc::c_char {
    let strlen = if src.is_null() {
        0
    } else {
        unsafe { libc::strlen(src) }
    };

    str_dup_n(src, strlen)
}

/// Duplicate a string of a given length.  Add a null terminator on
/// the end.
///
/// TODO: Intern empty strings.
///
/// TODO: Handle failed malloc in Rust when `panic` is ported.
///
/// TODO: Remove.
pub fn str_dup_n(src: *const libc::c_char, strlen: libc::size_t) -> *mut libc::c_char {
    let dst = almost_mymalloc(strlen + 1, MemoryType::M_STRING) as *mut libc::c_char;

    if !dst.is_null() {
        unsafe {
            libc::strncpy(dst, src, strlen);
            *(dst.offset(strlen as isize)) = 0;
        };
    }

    dst
}

/// Calculate space for the ref counting cell.
///
/// Only floats, strings, and lists get space for a refcount.  Element
/// alignment is preserved, even though the refcount is a `u32`.
fn refcount_offset(memory_type: MemoryType) -> usize {
    match memory_type {
        MemoryType::M_STRING => 4, // sizeof(int)
        MemoryType::M_LIST => 8, // sizeof(Var*)
        MemoryType::M_FLOAT => 8, // sizeof(double)
        _ => 0,
    }
}
