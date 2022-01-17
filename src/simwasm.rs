extern "C" {
    fn logOne();
}

#[no_mangle]
pub extern fn add(x: i32, y: i32) -> i32 {
    let mut res = 0;
    unsafe {
        logOne();
    }
    res + x + y
}
