fn main() {
    println!("Hello main");
}

#[cfg(test)]
#[test]
fn test_loop() {
    assert_eq!(10, 11);
}
