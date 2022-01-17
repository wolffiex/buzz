extern "C" {
    fn logOne(i: i32) -> i32;
}

#[no_mangle]
pub extern fn add(x: i32, y: i32) -> i32 {
    let mut res = 0;
    unsafe {
        res = logOne(98);
    }
    res + x + y
}
