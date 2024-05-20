use crate::cli::NamingConvention;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
/// The struct that defines the configuration file entries.
/// It is then used with [`confy::load()`].
pub struct Cfg {
    /// Same as [Cli::naming_convention](crate::cli::Cli::naming_convention)
    naming_convention: NamingConvention,

    /// Same as [Cli::recursive](crate::cli::Cli::recursive)
    recursive: bool,

    /// Same as [Cli::keep_dots](crate::cli::Cli::keep_dots)
    keep_dots: bool,

    /// Same as [Cli::keep_special_chars](crate::cli::Cli::keep_special_chars)
    keep_special_chars: bool,

    /// Same as [ExcludeSubcommands::Edit::editor](crate::cli::ExcludeSubcommands::Edit::editor)
    editor: String,
}

impl ::std::default::Default for Cfg {
    fn default() -> Self {
        Self {
            naming_convention: NamingConvention::SnakeCase,
            recursive: false,
            keep_dots: false,
            keep_special_chars: false,
            editor: String::from("vi"),
        }
    }
}
