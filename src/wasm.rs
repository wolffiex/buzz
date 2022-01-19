use std::ffi::CStr;
use std::mem;
use std::ptr;
use std::os::raw::{c_char, c_void};

extern "C" {
    fn chello() -> *mut c_char;
    fn consoleLog(cc: *mut c_char);
}


#[no_mangle]
pub fn get_handle() -> *mut c_void {
    let mut pairs: Vec<String> = vec!["foo", "barr", "alpha", "momega"]
        .into_iter().map(|s| s.to_string()).collect();
    let p = pairs.as_mut_ptr();
    mem::forget(pairs);
    p as *mut c_void
}

#[no_mangle]
pub fn drop_handle(p: *mut c_void) {
    let pairs:Vec<String> = unsafe {
        Vec::from_raw_parts(p as *mut String, 4, mem::size_of::<String>())
    };
    log(format!("well her {:?}", pairs.get(2)).to_string());
}

#[no_mangle]
pub fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let p = buf.as_mut_ptr();
    mem::forget(buf);
    p as *mut c_void
}

fn dealloc(p: *mut c_void, size: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(p, 0, size);
    }
}


fn log(s: String) {
    let bytes = s.as_bytes();
    let len = s.len();
    let mut buf = Vec::with_capacity(len + 1);
    let p: *mut u8 = buf.as_mut_ptr();
    unsafe {
        for i in 0..len as isize {
            ptr::write(p.offset(i), bytes[i as usize]);
        }
        ptr::write(p.offset(len as isize), 0);
        consoleLog(p as *mut c_char);
    }
}

#[no_mangle]
pub extern fn add(x: i32, y: i32) -> i32 {
    let phello = unsafe { chello() };
    let c_msg = unsafe { CStr::from_ptr(phello) };
    dealloc(phello as *mut c_void, c_msg.to_bytes().len());

    let message = format!("{} and Rust!", c_msg.to_str().unwrap());
    log(message);
    x + y
}