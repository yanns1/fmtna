use clap::ValueEnum;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;

#[derive(ValueEnum, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum NamingConvention {
    #[serde(rename = "camelCase")]
    #[value(name = "camelCase")]
    CamelCase,
    #[serde(rename = "kebab-case")]
    #[value(name = "kebab-case")]
    KebabCase,
    #[serde(rename = "snake_case")]
    #[value(name = "snake_case")]
    SnakeCase,
    #[serde(rename = "PascalCase")]
    #[value(name = "PascalCase")]
    PascalCase,
    #[serde(rename = "lower")]
    #[value(name = "lower")]
    Lower,
    #[serde(rename = "UPPER")]
    #[value(name = "UPPER")]
    Upper,
}

pub fn apply_nc(
    nc: &NamingConvention,
    filename: &str,
    keep_dots: bool,
    keep_special_chars: bool,
) -> String {
    match nc {
        NamingConvention::CamelCase => camel_case(filename, keep_dots, keep_special_chars),
        NamingConvention::KebabCase => kebab_case(filename, keep_dots, keep_special_chars),
        NamingConvention::SnakeCase => snake_case(filename, keep_dots, keep_special_chars),
        NamingConvention::PascalCase => pascal_case(filename, keep_dots, keep_special_chars),
        NamingConvention::Lower => lower(filename, keep_dots, keep_special_chars),
        NamingConvention::Upper => upper(filename, keep_dots, keep_special_chars),
    }
}

fn camel_case(s: &str, keep_dots: bool, keep_special_chars: bool) -> String {
    let _ = s;
    let _ = keep_dots;
    let _ = keep_special_chars;
    todo!()
}

fn kebab_case(s: &str, keep_dots: bool, keep_special_chars: bool) -> String {
    let _ = s;
    let _ = keep_dots;
    let _ = keep_special_chars;
    todo!()
}

fn snake_case(s: &str, keep_dots: bool, keep_special_chars: bool) -> String {
    let _ = s;
    let _ = keep_dots;
    let _ = keep_special_chars;
    s.to_lowercase()
}

fn pascal_case(s: &str, keep_dots: bool, keep_special_chars: bool) -> String {
    let _ = s;
    let _ = keep_dots;
    let _ = keep_special_chars;
    todo!()
}

fn lower(s: &str, keep_dots: bool, keep_special_chars: bool) -> String {
    let _ = keep_dots;
    let _ = keep_special_chars;
    s.to_lowercase()
}

fn upper(s: &str, keep_dots: bool, keep_special_chars: bool) -> String {
    let _ = keep_dots;
    let _ = keep_special_chars;
    s.to_uppercase()
}
