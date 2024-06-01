use crate::naming_conventions::NamingConvention;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
/// The struct that defines the configuration file entries.
/// It is then used with [`confy::load()`].
pub struct Cfg {
    /// Same as [DefaultArgs::naming_convention](crate::default::DefaultArgs::naming_convention)
    pub naming_convention: NamingConvention,

    /// Same as [DefaultArgs::recursive](crate::default::DefaultArgs::recursive)
    pub recursive: bool,

    /// Same as [DefaultArgs::keep_dots](crate::default::DefaultArgs::keep_dots)
    pub keep_dots: bool,

    /// Same as [DefaultArgs::keep_special_chars](crate::default::DefaultArgs::keep_special_chars)
    pub keep_special_chars: bool,

    /// Same as [crate::exclude::edit::EditCli::editor](crate::exclude::edit::EditCli::editor)
    pub editor: String,
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
