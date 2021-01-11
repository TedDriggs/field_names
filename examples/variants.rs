#![allow(dead_code)]

use field_names::VariantNames;

#[derive(VariantNames)]
enum Example {
    Hello(String),
    #[variant_names(skip)]
    Secret(String),
    World {
        planet: String,
        person: String,
    },
}

fn main() {
    println!("{:?}", Example::VARIANTS);
}
