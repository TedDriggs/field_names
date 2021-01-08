#![allow(dead_code)]

use field_names::FieldNames;

#[derive(FieldNames)]
struct Example {
    hello: String,
    world: String,
    minutes_to_midnight: u32,
    #[field_names(skip)]
    hidden: (),
}

fn main() {
    println!("{:?}", Example::FIELDS);
}