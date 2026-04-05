use ris_error::prelude::*;

pub fn trim_type_name(type_name: &str) -> &str {
    let last = type_name
        .split("::")
        .last()
        .ris_expect("type name to exist");

    ris_error::unwrap!(last, "no type_name")
}
