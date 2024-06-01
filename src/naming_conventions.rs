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
