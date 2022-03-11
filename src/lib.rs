type Id = [u8;8];

type Topic = [u8;16];

enum Value {
    Str(String),
    Num(i32),
    Boo(bool),
    Edg(Id),
}

struct Prop {
    name: String,
    value: Value,
}

struct Message {
    topic: Topic,
    id: Id,
    props: Vec<Prop>
}

pub fn get_id() -> Id {
    [0;8]
}


#[cfg(test)]
#[test]
fn test_largest() {
    assert_eq!(33, 22);
}
