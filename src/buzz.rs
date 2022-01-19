use std::collections::HashMap;
use ring::rand::SecureRandom;

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

type PropName = &'static str;
struct Record {
    id1: ID,
    props: String,
    id2: ID,
}

impl Record {
    pub fn new(id1: ID, props: String) -> Record {
        let id2 = sign(id1, &props);
        Record {
            id1,
            props,
            id2,
        }
    }
}

fn sign(id1: ID, props: &String) -> ID {
    [0; 16]
}

#[cfg(test)]
#[test]
fn test_it() {
    let mut props: HashMap<PropName, Value> = HashMap::new();

    props.insert("name", Value::String("Billy Foo".to_string()));
    props.insert("email", Value::String("billy@foo".to_string()));
    println!("props: {:?}", props.len())
}
