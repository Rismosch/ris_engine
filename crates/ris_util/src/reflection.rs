use ris_error::Extensions;

pub fn trim_type_name(type_name: &str) -> &str {
    let last = type_name.split("::").last().into_ris_error();

    ris_error::unwrap!(last, "no type_name",)
}
