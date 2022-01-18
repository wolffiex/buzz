use ring::rand::SecureRandom;
use base64::{encode};
type ID = [u8; 16];

#[allow(dead_code)]
enum Value {
    Null,
    Bool(bool),
    Number(i32),
    String(String),
    Id(ID),
}

#[allow(dead_code)]
pub struct Prop {
    name: String,
    value: Value,
}

#[allow(dead_code)]
//pub fn read(id: ID, offset: i32) -> [u64] {
#[allow(dead_code)]
pub fn write(_id: ID, _props: &[&Prop]) -> ID {
    let id = get_random_id();
    return id;
}

fn get_random_id() -> ID {
    let mut randoms: ID = [0; 16];

    let sr = ring::rand::SystemRandom::new();
    sr.fill(&mut randoms).expect("SystemRandom failure");
    randoms
}

#[cfg(test)]
#[test]
fn test_it() {
    let randoms = get_random_id();
    println!("it's {:?}", randoms);
    println!("encl {:?}", encode(randoms));
}
