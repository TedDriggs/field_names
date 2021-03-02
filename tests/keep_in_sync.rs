//! These tests illustrate the use of `FieldNames` to keep struct definitions in sync if either
//! struct is updated in a later commit.
//!
//! # Motivation
//! When dealing with `serde`, fields and their names matter. If a single struct needs multiple
//! representations (e.g. for JSON and Bincode serialization), it's not possible to derive the
//! `Serialize` and `Deserialize` traits multiple times for a single base struct, but it's strongly
//! desired that each of the sibling representations remain in sync to avoid data correctness
//! issues.
//!
//! While Rust itself cannot encode such a constraint, unit tests can. Enforcement at unit-test time
//! is almost as good as compile-time enforcement, especially with CI/CD. The alternative solution
//! would be use of a macro to generate all needed structs, but macro-generated structs are not reliably
//! picked up by IDE tooling such as rust-analyzer, and the author will be forced to design their own
//! ad-hoc syntax for struct declaration as they find themselves needing to further differentiate
//! the sibling structs.
//!
//! # Conditional Compilation
//! If `FieldNames` is only being used for unit tests, include it under `dev-dependencies` and use
//! `#[cfg_attr(test, derive(FieldNames))]` to avoid any binary size cost in the built crate.

use std::{
    collections::BTreeSet,
    convert::TryFrom,
    net::{AddrParseError, IpAddr},
};

use field_names::FieldNames;

#[derive(FieldNames, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
struct Base {
    target: IpAddr,
    lorem: String,
    ipsum: String,
    dolor: String,
    lorem_computation: String,
    #[field_names(skip)]
    secret: bool,
}

impl TryFrom<Raw> for Base {
    type Error = AddrParseError;

    fn try_from(value: Raw) -> Result<Self, Self::Error> {
        Ok(Self {
            // A "rename" refactor by rust-analyzer or similar IDE tooling would have changed
            // the name of the `target` field but wouldn't have changed the name of `dest` since
            // that belongs to a different struct. Even code inspection might not have detected
            // that the two are supposed to remain in-sync, since the changing commit doesn't
            // go anywhere near the `Raw` struct.
            target: value.dest.parse()?,
            secret: value.lorem.is_empty(),
            lorem_computation: value.lorem.to_ascii_lowercase(),
            lorem: value.lorem,
            ipsum: value.ipsum,
            dolor: value.dolor,
        })
    }
}

/// A view of `Base`.
///
/// # Guarantees
/// 1. Two `View` instances will be equal if and only if their `Base` instances are equal.
/// 2. Ordering `View` instances will produce the same ordering as ordering their `Base` instances.
#[derive(FieldNames, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
struct View<'a> {
    // Field order matters to the derived `PartialOrd` trait, so it must stay in sync with `Base`.
    target: &'a IpAddr,
    lorem: &'a str,
    ipsum: &'a str,
    dolor: &'a str,
}

#[derive(FieldNames)]
#[allow(dead_code)]
struct PartialView<'a> {
    lorem: &'a str,
    target: &'a IpAddr,
}

/// Deserialized, unvalidated form of `Base`. The field names and order must match `Base` for
/// round-trip serialization and deserialization of `Base` structs to work.
#[derive(FieldNames)]
struct Raw {
    // THE "MISTAKE" IS HERE; a previous rename of `dest` to `target` wasn't applied to the
    // raw form, despite it being supposed to be a derivative of `Base`. If `Base` implements
    // `Serialize` and relies on `Raw` for deserialization, then the roundtripping of this
    // struct will no longer work.
    dest: String,
    lorem: String,
    ipsum: String,
    dolor: String,
}

/// The declaration of `View` notes some guarantees that must be upheld, and the implementation
/// constraint that field declaration order match between the two structs.
///
/// The `FieldNames` macro emits fields in declaration order, making testing that constraint easy.
/// The `lorem_computation` field (deliberately) makes this slightly more complex, since it means
/// the two field lists are not supposed to be _identical_. The test handles this by making a list
/// of exactly which fields are expected to be absent from `View` and adding them at comparison
/// time. If `lorem_computation` is removed, then this test will require an update; that's desirable,
/// as it ensures the removing commit also updates the test.
#[test]
fn base_and_view_in_sync_preserve_source_order() {
    const DELIBERATELY_OMITTED_FROM_VIEW: &[&str] = &["lorem_computation"];

    let base_fields = Base::FIELDS.iter().collect::<Vec<_>>();
    let view_fields = View::FIELDS
        .iter()
        .chain(DELIBERATELY_OMITTED_FROM_VIEW)
        .collect::<Vec<_>>();
    assert_eq!(base_fields, view_fields);
}

/// In this test, field order doesn't matter. To make changes to `Base` easier, we sort the field names
/// by adding them to a pair of `BTreeSet` instances and then make sure that `base - partial_view = {}`.
#[test]
fn base_and_partial_view_in_sync_ignore_source_order() {
    let base_fields = Base::FIELDS.iter().collect::<BTreeSet<_>>();
    let partial_view_fields = PartialView::FIELDS.iter().collect::<BTreeSet<_>>();

    let unexpected_fields = partial_view_fields
        .difference(&base_fields)
        .collect::<Vec<_>>();

    if !unexpected_fields.is_empty() {
        panic!("Fields not in base: {:?}", unexpected_fields);
    }
}

#[test]
#[should_panic]
fn base_and_raw_not_in_sync() {
    // This test won't even compile if Base and Raw have different numbers of fields
    // assert_eq!(Base::FIELDS, Raw::FIELDS);

    const KNOWN_OMISSIONS_FROM_RAW: &[&str] = &["lorem_computation"];

    let base_fields = Base::FIELDS.iter().collect::<Vec<_>>();
    let raw_fields = Raw::FIELDS
        .iter()
        .chain(KNOWN_OMISSIONS_FROM_RAW)
        .collect::<Vec<_>>();
    assert_eq!(base_fields, raw_fields);
}
