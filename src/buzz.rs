#![allow(dead_code, unused_variables, unused_imports)]
use std::collections::HashMap;
type ID = [u8; 16];

#[allow(dead_code)]
enum Value {
    Null,
    Bool(bool),
    Number(i32),
    String(String),
    Id(ID),
}

type PropName = &'static str;
#[allow(dead_code)]
pub struct Prop {
    name: PropName,
    value: Value,
}

#[allow(dead_code)]
pub fn write(_id: ID, _props: &[&Prop]) -> ID {
    let id = get_random_id();
    return id;
}

fn get_random_id() -> ID {
    unimplemented!();
}

#[allow(dead_code)]
struct Record {
    id1: ID,
    props: String,
    id2: ID,
}

impl Record {
    #[allow(dead_code)]
    pub fn new(id1: ID, props: String) -> Record {
        let id2 = sign(id1, &props);
        Record { id1, props, id2 }
    }
}

#[allow(dead_code)]
fn sign(_id1: ID, _props: &String) -> ID {
    [0; 16]
}

#[cfg(test)]
#[test]
fn test_it() {
    let mut props: HashMap<PropName, Value> = HashMap::new();

    props.insert("name", Value::String("Billy Foo".to_string()));
    props.insert("email", Value::String("billy@foo".to_string()));
    println!("props: {:?}", props.len());
    assert_eq!(33, 44);
}

#[test]
fn test_option() {
    let x = String::from("DFLKJ");
    println!("x: {:?}", x);
    assert!(false)
}
