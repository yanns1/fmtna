use clap::ValueEnum;
use lazy_static::lazy_static;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use unidecode::unidecode;

lazy_static! {
    static ref SEPARATORS: [char; 7] = ['_', '-', '.', ' ', '\t', '\r', '\n'];
}

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
    keep_unicode: bool,
) -> String {
    match nc {
        NamingConvention::CamelCase => {
            camel_case(filename, keep_dots, keep_special_chars, keep_unicode)
        }
        NamingConvention::KebabCase => {
            kebab_case(filename, keep_dots, keep_special_chars, keep_unicode)
        }
        NamingConvention::SnakeCase => {
            snake_case(filename, keep_dots, keep_special_chars, keep_unicode)
        }
        NamingConvention::PascalCase => {
            pascal_case(filename, keep_dots, keep_special_chars, keep_unicode)
        }
        NamingConvention::Lower => lower(filename, keep_dots, keep_special_chars, keep_unicode),
        NamingConvention::Upper => upper(filename, keep_dots, keep_special_chars, keep_unicode),
    }
}

fn camel_case(s: &str, keep_dots: bool, keep_special_chars: bool, keep_unicode: bool) -> String {
    let mut new_s = String::from("");
    let unidecoded: String;
    let mut slice = s;
    if !keep_unicode {
        unidecoded = unidecode(s);
        slice = unidecoded.as_ref();
    }

    let mut should_upper = false;
    for (i, c) in slice.chars().enumerate() {
        if SEPARATORS.contains(&c) && !(keep_dots && c == '.') && i > 0 && i < slice.len() - 1 {
            should_upper = true;
        } else if is_special(&c) {
            if keep_special_chars {
                new_s.push(c);
            } else {
                continue;
            }
        } else if c.is_uppercase() && i > 0 && slice.chars().nth(i - 1).unwrap().is_lowercase() {
            new_s.push(c);
        } else if should_upper {
            new_s.push(c.to_uppercase().next().unwrap());
            should_upper = false;
        } else {
            new_s.push(c.to_lowercase().next().unwrap());
        }
    }

    new_s
}

fn kebab_case(s: &str, keep_dots: bool, keep_special_chars: bool, keep_unicode: bool) -> String {
    // kebab-case
    let _ = s;
    let _ = keep_dots;
    let _ = keep_special_chars;
    let _ = keep_unicode;
    todo!()
}

fn snake_case(s: &str, keep_dots: bool, keep_special_chars: bool, keep_unicode: bool) -> String {
    let mut new_s = String::from("");
    let unidecoded: String;
    let mut slice = s;
    if !keep_unicode {
        unidecoded = unidecode(s);
        slice = unidecoded.as_ref();
    }

    for (i, c) in slice.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 && slice.chars().nth(i - 1).unwrap().is_lowercase() {
                new_s.push('_');
            }
            new_s.push(c.to_lowercase().next().unwrap());
        } else if SEPARATORS.contains(&c) && !(keep_dots && c == '.') {
            new_s.push('_');
        } else if !keep_special_chars && is_special(&c) {
            continue;
        } else {
            new_s.push(c.to_lowercase().next().unwrap());
        }
    }

    new_s
}

fn pascal_case(s: &str, keep_dots: bool, keep_special_chars: bool, keep_unicode: bool) -> String {
    capitalize(&camel_case(s, keep_dots, keep_special_chars, keep_unicode))
}

fn lower(s: &str, keep_dots: bool, keep_special_chars: bool, keep_unicode: bool) -> String {
    let _ = keep_dots;

    let mut new_s: String;
    let mut slice = s;
    if !keep_unicode {
        new_s = unidecode(s);
        slice = new_s.as_ref();
    }
    if !keep_special_chars {
        new_s = slice
            .chars()
            .filter(|c| SEPARATORS.contains(c) || !is_special(c))
            .collect();
        slice = new_s.as_ref();
    }

    slice.to_lowercase()
}

fn upper(s: &str, keep_dots: bool, keep_special_chars: bool, keep_unicode: bool) -> String {
    let _ = keep_dots;

    let mut new_s: String;
    let mut slice = s;
    if !keep_unicode {
        new_s = unidecode(s);
        slice = new_s.as_ref();
    }
    if !keep_special_chars {
        new_s = slice
            .chars()
            .filter(|c| SEPARATORS.contains(c) || !is_special(c))
            .collect();
        slice = new_s.as_ref();
    }

    slice.to_uppercase()
}

fn is_special(c: &char) -> bool {
    !c.is_alphanumeric()
}

fn capitalize(s: &str) -> String {
    if s.is_empty() {
        return String::from("");
    }

    let mut capitalized = s.to_string();
    capitalized = capitalized.remove(0).to_uppercase().to_string() + &capitalized;
    capitalized
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug)]
    struct TestCase<'a> {
        s: &'a str,
        keep_dots: bool,
        keep_special_chars: bool,
        keep_unicode: bool,
        expected_output: &'a str,
    }

    #[test]
    fn test_snake_case() {
        let test_cases = vec![
            TestCase {
                s: "",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "",
            },
            TestCase {
                s: "a",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "a",
            },
            TestCase {
                s: "A",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "a",
            },
            TestCase {
                s: "from_snake_case",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "from_snake_case",
            },
            TestCase {
                s: "FROM_UPPERCASE",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "from_uppercase",
            },
            TestCase {
                s: "fromlowercase",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "fromlowercase",
            },
            TestCase {
                s: "fromCamelCase",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "from_camel_case",
            },
            TestCase {
                s: "FromPascalCase",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "from_pascal_case",
            },
            TestCase {
                s: "from-kebab-case",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "from_kebab_case",
            },
            TestCase {
                s: "fRo_m`WHAT.ev-eR!",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "f_ro_mwhat_ev_e_r",
            },
            TestCase {
                s: "fRo_m`WHAT.ev-eR!",
                keep_dots: true,
                keep_special_chars: true,
                keep_unicode: false,
                expected_output: "f_ro_m`what.ev_e_r!",
            },
            TestCase {
                s: "é çà devrait être 'asciifié'",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "e_ca_devrait_etre_asciifie",
            },
            TestCase {
                s: "é çà devrait être 'asciifié' mais en gardant les guillemets",
                keep_dots: false,
                keep_special_chars: true,
                keep_unicode: false,
                expected_output: "e_ca_devrait_etre_'asciifie'_mais_en_gardant_les_guillemets",
            },
        ];

        for TestCase {
            s,
            keep_dots,
            keep_special_chars,
            keep_unicode,
            expected_output,
        } in test_cases
        {
            assert_eq!(
                snake_case(s, keep_dots, keep_special_chars, keep_unicode),
                expected_output
            );
        }
    }

    #[test]
    fn test_lower() {
        let test_cases = vec![
            TestCase {
                s: "",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "",
            },
            TestCase {
                s: "a",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "a",
            },
            TestCase {
                s: "A",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "a",
            },
            TestCase {
                s: "from_snake_case",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "from_snake_case",
            },
            TestCase {
                s: "FROM_UPPERCASE",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "from_uppercase",
            },
            TestCase {
                s: "fromlowercase",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "fromlowercase",
            },
            TestCase {
                s: "fromCamelCase",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "fromcamelcase",
            },
            TestCase {
                s: "FromPascalCase",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "frompascalcase",
            },
            TestCase {
                s: "from-kebab-case",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "from-kebab-case",
            },
            TestCase {
                s: "fRo_m`WHAT.ev-eR!",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "fro_mwhat.ev-er",
            },
            TestCase {
                s: "é çà devrait être 'asciifié'",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "e ca devrait etre asciifie",
            },
            TestCase {
                s: "é çà devrait être 'asciifié' mais en gardant les guillemets",
                keep_dots: false,
                keep_special_chars: true,
                keep_unicode: false,
                expected_output: "e ca devrait etre 'asciifie' mais en gardant les guillemets",
            },
        ];

        for TestCase {
            s,
            keep_dots,
            keep_special_chars,
            keep_unicode,
            expected_output,
        } in test_cases
        {
            assert_eq!(
                lower(s, keep_dots, keep_special_chars, keep_unicode),
                expected_output
            );
        }
    }

    #[test]
    fn test_upper() {
        let test_cases = vec![
            TestCase {
                s: "",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "",
            },
            TestCase {
                s: "a",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "A",
            },
            TestCase {
                s: "A",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "A",
            },
            TestCase {
                s: "from_snake_case",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "FROM_SNAKE_CASE",
            },
            TestCase {
                s: "FROM_UPPERCASE",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "FROM_UPPERCASE",
            },
            TestCase {
                s: "fromlowercase",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "FROMLOWERCASE",
            },
            TestCase {
                s: "fromCamelCase",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "FROMCAMELCASE",
            },
            TestCase {
                s: "FromPascalCase",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "FROMPASCALCASE",
            },
            TestCase {
                s: "from-kebab-case",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "FROM-KEBAB-CASE",
            },
            TestCase {
                s: "fRo_m`WHAT.ev-eR!",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "FRO_MWHAT.EV-ER",
            },
            TestCase {
                s: "é çà devrait être 'asciifié'",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "E CA DEVRAIT ETRE ASCIIFIE",
            },
            TestCase {
                s: "é çà devrait être 'asciifié' mais en gardant les guillemets",
                keep_dots: false,
                keep_special_chars: true,
                keep_unicode: false,
                expected_output: "E CA DEVRAIT ETRE 'ASCIIFIE' MAIS EN GARDANT LES GUILLEMETS",
            },
        ];

        for TestCase {
            s,
            keep_dots,
            keep_special_chars,
            keep_unicode,
            expected_output,
        } in test_cases
        {
            assert_eq!(
                upper(s, keep_dots, keep_special_chars, keep_unicode),
                expected_output
            );
        }
    }

    #[test]
    fn test_camel_case() {
        let test_cases = vec![
            TestCase {
                s: "",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "",
            },
            TestCase {
                s: "a",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "a",
            },
            TestCase {
                s: "A",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "a",
            },
            TestCase {
                s: "from_snake_case",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "fromSnakeCase",
            },
            TestCase {
                s: "FROM_UPPERCASE",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "fromUppercase",
            },
            TestCase {
                s: "fromlowercase",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "fromlowercase",
            },
            TestCase {
                s: "fromCamelCase",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "fromCamelCase",
            },
            TestCase {
                s: "FromPascalCase",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "fromPascalCase",
            },
            TestCase {
                s: "from-kebab-case",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "fromKebabCase",
            },
            TestCase {
                s: "fRo_m`WHAT.ev-eR!",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "fRoMwhatEvER",
            },
            TestCase {
                s: "é çà devrait être 'asciifié'",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "eCaDevraitEtreAsciifie",
            },
            TestCase {
                s: "é çà devrait être 'asciifié' mais en gardant les guillemets",
                keep_dots: false,
                keep_special_chars: true,
                keep_unicode: false,
                expected_output: "eCaDevraitEtre'Asciifie'MaisEnGardantLesGuillemets",
            },
            TestCase {
                s: "_separator_at_beginning_and_end_",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "separatorAtBeginningAndEnd",
            },
            TestCase {
                s: "_____multiple_______separators_____",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "MultipleSeparators",
            },
        ];

        for TestCase {
            s,
            keep_dots,
            keep_special_chars,
            keep_unicode,
            expected_output,
        } in test_cases
        {
            assert_eq!(
                camel_case(s, keep_dots, keep_special_chars, keep_unicode),
                expected_output
            );
        }
    }

    #[test]
    fn test_pascal_case() {
        let test_cases = vec![
            TestCase {
                s: "",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "",
            },
            TestCase {
                s: "a",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "A",
            },
            TestCase {
                s: "A",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "A",
            },
            TestCase {
                s: "from_snake_case",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "FromSnakeCase",
            },
            TestCase {
                s: "FROM_UPPERCASE",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "FromUppercase",
            },
            TestCase {
                s: "fromlowercase",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "Fromlowercase",
            },
            TestCase {
                s: "fromCamelCase",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "FromCamelCase",
            },
            TestCase {
                s: "FromPascalCase",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "FromPascalCase",
            },
            TestCase {
                s: "from-kebab-case",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "FromKebabCase",
            },
            TestCase {
                s: "fRo_m`WHAT.ev-eR!",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "FRoMwhatEvER",
            },
            TestCase {
                s: "é çà devrait être 'asciifié'",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "ECaDevraitEtreAsciifie",
            },
            TestCase {
                s: "é çà devrait être 'asciifié' mais en gardant les guillemets",
                keep_dots: false,
                keep_special_chars: true,
                keep_unicode: false,
                expected_output: "ECaDevraitEtre'Asciifie'MaisEnGardantLesGuillemets",
            },
            TestCase {
                s: "_separator_at_beginning_and_end_",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "SeparatorAtBeginningAndEnd",
            },
            TestCase {
                s: "_____multiple_______separators_____",
                keep_dots: false,
                keep_special_chars: false,
                keep_unicode: false,
                expected_output: "MultipleSeparators",
            },
        ];

        for TestCase {
            s,
            keep_dots,
            keep_special_chars,
            keep_unicode,
            expected_output,
        } in test_cases
        {
            assert_eq!(
                pascal_case(s, keep_dots, keep_special_chars, keep_unicode),
                expected_output
            );
        }
    }
}
