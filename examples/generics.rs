#![allow(dead_code)]

use field_names::FieldNames;
#[derive(FieldNames)]
struct Example<T> {
    hello: T,
    world: String,
}

fn main() {
    println!("{:?}", Example::<()>::FIELDS);
}
