use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;
use std::os::raw::{c_char, c_void};
use std::collections::HashMap;

static mut RECORDS: Option<HashMap<String, String>> = None;

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
fn init_records() {
    log("want to init".to_string());
    unsafe {
        if let None = RECORDS {
            RECORDS = Some(HashMap::new());
            log("initedei".to_string());
        }
    }
}

fn with_records<T, F>(fun: F) -> T where
    F: FnOnce(&mut HashMap<String, String>) -> T
{
    let result = unsafe {
        if let None = RECORDS {
            RECORDS = Some(HashMap::new());
        }
        if let Some(records) = &mut RECORDS {
            Some(fun(records))
        } else {
            None
        }
    };

    return result.unwrap();
}

pub fn convert_cstring(cs: *mut c_char) -> String {
    let cs = unsafe {
        CString::from_raw(cs)
    };
    String::from_utf8(cs.to_bytes().to_vec()).unwrap()
}

#[no_mangle]
pub fn write(ckey: *mut c_char, cvalue: *mut c_char) -> bool {
    let key = convert_cstring(ckey);
    let value = convert_cstring(cvalue);

    unsafe {
        if let Some(records) = &mut RECORDS {
            log(format!("inserted {:?}", &key));
            records.insert(key, value);
        }
    };

    let res = with_records(|records| -> Option<String> {
        records.insert("xx".to_string(), "aa".to_string())
    });
    log(format!("res had {:?}", res));

    true
}

#[no_mangle]
pub fn read(ckey: *mut c_char) -> *const c_char {
    let key = convert_cstring(ckey);
    let value = unsafe {
        if let Some(records) = &RECORDS {
            records.get(&*key)
        } else {
            None
        }
    };
    let as_string = match value {
        Some(s) => s.clone(),
        None => String::new(),
    };
    log(format!("found {:?}", as_string));
    to_leaked_cstring(&as_string)
}

fn to_leaked_cstring(s: &String) -> *const c_char {
    let cs = CString::new(s.to_string()).unwrap();
    let p = cs.as_ptr();
    mem::forget(cs);
    p
}

#[no_mangle]
pub fn free_cstring(p: *mut c_char) {
    let _ = unsafe {
        CString::from_raw(p)
    };
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