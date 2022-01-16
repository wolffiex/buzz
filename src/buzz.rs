use std::collections::HashSet;

type ID = &str;
enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Id(ID),
}
type UPDATE = (
    ID,
    (Vec<(PROP_NAME, JSON_VALUE)>, Vec<EDGE_NAME, JSON_VALUE>),
);

pub fn write(id: ID, index: INDEX_NAME, props: &[&Value]) -> ID {}
pub fn new_key() -> ID {}

pub fn read() -> ID {}
