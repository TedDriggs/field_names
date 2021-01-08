# field_names

[![Build Status](https://github.com/TedDriggs/field_names/workflows/CI/badge.svg)](https://github.com/TedDriggs/field_names/actions)
[![Latest Version](https://img.shields.io/crates/v/field_names.svg)](https://crates.io/crates/field_names)

`field_names` is a Rust proc-macro that exposes a struct's field names as strings at runtime.

# Example

Consider a simple struct such as this one.

```rust
#[derive(FieldNames)]
struct Example {
    hello: String,
    world: String,
    #[field_names(skip)]
    ignore_me: bool,
}
```

`field_names` will emit the following:

```rust
#[automatically_derived]
impl Example {
    const FIELDS: [&'static str; 2] = [
        "hello",
        "world",
    ];
}
```

# Uses

This crate was originally created for a case where a set of rules were being read at runtime which referenced fields of structs elsewhere in the code base.
The referenced struct exposed a method which had a `match` statement to go from strings to its fields, but there was not a way to ensure the arms of that match statement stayed in sync with the struct definition.
With this crate, a unit test could be created to ensure that every field on the struct - except those deliberately omitted - was handled by the method.

# FAQs

### Why isn't `FieldNames` a trait?

Using `field_names` is an implementation convenience; it shouldn't force you to change your crate's public API.

### How do I make `FIELDS` public?

You can add your own inherent method, e.g. `fields() -> &[&'static str]`, or define a trait that matches your use-case and reference `FIELDS` in the trait implementation.
