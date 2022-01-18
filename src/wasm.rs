use std::ffi::CStr;
use std::mem;
use std::ptr;
use std::os::raw::{c_char, c_void};

extern "C" {
    fn chello() -> *mut c_char;
    fn consoleLog(cc: *mut c_char);
}

fn dealloc(p: *mut c_void, size: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(p, 0, size);
    }
}

#[no_mangle]
pub fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let p = buf.as_mut_ptr();
    mem::forget(buf);
    p as *mut c_void
}


#[no_mangle]
pub extern fn add(x: i32, y: i32) -> i32 {
    let phello = unsafe { chello() };
    let c_msg = unsafe { CStr::from_ptr(phello) };
    dealloc(phello as *mut c_void, c_msg.to_bytes().len());

    let message = format!("{} and Rust!", c_msg.to_str().unwrap());
    let len = message.as_bytes().len();
    let p = to_pointer(message);


    unsafe {
        consoleLog(p as *mut c_char);
        dealloc(p as *mut c_void, len);
    }
    x + y
}

fn to_pointer(message: String) -> *mut u8 {
    let bytes = message.as_bytes();
    let len = message.len();
    let p = alloc(len + 1) as *mut u8;
    unsafe {
        for i in 0..len as isize {
            ptr::write(p.offset(i), bytes[i as usize]);
        }
        ptr::write(p.offset(len as isize), 0);
    }
    p
}
