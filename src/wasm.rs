use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;
use std::os::raw::{c_char, c_void};

extern "C" {
    fn chello() -> *mut c_char;
    fn consoleLog(cc: *mut c_char);
}

#[no_mangle]
pub fn get_handle() -> *const c_char {
    let c = CString::new("[\"kej8==\", {x:99}]").expect("CString::new failed");
    let p = c.as_ptr();
    mem::forget(c);
    p
}

#[no_mangle]
pub fn write(p_to_insert: *mut c_char, p_transcript: *mut c_char) -> *const c_char {
    let to_insert = unsafe {
        CString::from_raw(p_to_insert)
    };

    let transcript = unsafe {
        CString::from_raw(p_transcript)
    };


    let i_bytes = to_insert.to_bytes();
    let t_bytes = transcript.to_bytes();
    log(format!("ib {}", String::from_utf8(i_bytes.clone().to_vec()).unwrap()));
    log(format!("tb {}", String::from_utf8(t_bytes.clone().to_vec()).unwrap()));


    let mut vec : Vec<u8> = Vec::from(t_bytes);
    vec.pop(); // remove null terminator
    let ins= Vec::from(i_bytes);
    vec.extend(&ins );
    // let stripped = &t_bytes[0..i_bytes.len()-1];
    // let combined = [stripped, i_bytes].concat();
    let new_t = CString::new(vec).unwrap();
    let clonet = new_t.clone();
    log(format!("ins {}", clonet.to_str().unwrap()));

    let p = new_t.as_ptr();
    mem::forget(new_t);
    log("forgotten".to_string());
    p
}

#[no_mangle]
pub fn drop_handle(p: *mut c_char) {
    let c = unsafe {
        CString::from_raw(p)
    };
    log(format!("dong {}", c.to_str().unwrap()))
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