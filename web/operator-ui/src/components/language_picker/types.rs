//! LanguagePicker data models, Language enum and flags.

/// Flag representation: built-in standard country code, custom inline SVG markup, or none.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Flag {
    /// Built-in country code (e.g. "UA", "GB", "US", "DE", etc.).
    Builtin(&'static str),
    /// Inline SVG markup provided by the application.
    Svg(String),
    /// No flag displayed.
    None,
}

impl From<&'static str> for Flag {
    fn from(code: &'static str) -> Self {
        Self::Builtin(code)
    }
}

/// A language option available for selection.
#[derive(Clone, Debug, PartialEq)]
pub struct LanguageOption {
    /// BCP 47 language code (e.g. "uk", "en", "de").
    pub code: String,
    /// Name of the language in its native form (e.g. "Українська", "Deutsch").
    pub native: String,
    /// Name of the language in English (e.g. "Ukrainian", "German").
    pub english: String,
    /// Associated flag icon.
    pub flag: Option<Flag>,
}

impl LanguageOption {
    /// Creates a new language option without a flag.
    pub fn new(
        code: impl Into<String>,
        native: impl Into<String>,
        english: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            native: native.into(),
            english: english.into(),
            flag: None,
        }
    }

    /// Attaches a flag to the language option.
    pub fn with_flag(mut self, flag: Flag) -> Self {
        self.flag = Some(flag);
        self
    }
}

/// Built-in languages with standard codes, native and English names, and flags.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Language {
    En,
    Uk,
    De,
    Fr,
    Es,
    It,
    Pl,
    Pt,
    Nl,
    Cs,
    Sv,
    Ja,
    Zh,
    Ko,
    Tr,
}

impl Language {
    /// All built-in languages in standard order.
    pub const ALL: &[Language] = &[
        Language::En,
        Language::Uk,
        Language::De,
        Language::Fr,
        Language::Es,
        Language::It,
        Language::Pl,
        Language::Pt,
        Language::Nl,
        Language::Cs,
        Language::Sv,
        Language::Ja,
        Language::Zh,
        Language::Ko,
        Language::Tr,
    ];

    /// Returns the BCP 47 language code.
    pub const fn code(&self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Uk => "uk",
            Self::De => "de",
            Self::Fr => "fr",
            Self::Es => "es",
            Self::It => "it",
            Self::Pl => "pl",
            Self::Pt => "pt",
            Self::Nl => "nl",
            Self::Cs => "cs",
            Self::Sv => "sv",
            Self::Ja => "ja",
            Self::Zh => "zh",
            Self::Ko => "ko",
            Self::Tr => "tr",
        }
    }

    /// Returns the native name of the language.
    pub const fn native(&self) -> &'static str {
        match self {
            Self::En => "English",
            Self::Uk => "Українська",
            Self::De => "Deutsch",
            Self::Fr => "Français",
            Self::Es => "Español",
            Self::It => "Italiano",
            Self::Pl => "Polski",
            Self::Pt => "Português",
            Self::Nl => "Nederlands",
            Self::Cs => "Čeština",
            Self::Sv => "Svenska",
            Self::Ja => "日本語",
            Self::Zh => "中文",
            Self::Ko => "한국어",
            Self::Tr => "Türkçe",
        }
    }

    /// Returns the English name of the language.
    pub const fn english(&self) -> &'static str {
        match self {
            Self::En => "English",
            Self::Uk => "Ukrainian",
            Self::De => "German",
            Self::Fr => "French",
            Self::Es => "Spanish",
            Self::It => "Italian",
            Self::Pl => "Polish",
            Self::Pt => "Portuguese",
            Self::Nl => "Dutch",
            Self::Cs => "Czech",
            Self::Sv => "Swedish",
            Self::Ja => "Japanese",
            Self::Zh => "Chinese",
            Self::Ko => "Korean",
            Self::Tr => "Turkish",
        }
    }

    /// Returns the default flag for this language.
    pub const fn flag(&self) -> Option<Flag> {
        match self {
            Self::En => Some(Flag::Builtin("GB")),
            Self::Uk => Some(Flag::Builtin("UA")),
            Self::De => Some(Flag::Builtin("DE")),
            Self::Fr => Some(Flag::Builtin("FR")),
            Self::Es => Some(Flag::Builtin("ES")),
            Self::It => Some(Flag::Builtin("IT")),
            Self::Pl => Some(Flag::Builtin("PL")),
            Self::Pt => Some(Flag::Builtin("PT")),
            Self::Nl => Some(Flag::Builtin("NL")),
            Self::Cs => Some(Flag::Builtin("CZ")),
            Self::Sv => Some(Flag::Builtin("SE")),
            Self::Ja => Some(Flag::Builtin("JP")),
            Self::Zh => Some(Flag::Builtin("CN")),
            Self::Ko => Some(Flag::Builtin("KR")),
            Self::Tr => Some(Flag::Builtin("TR")),
        }
    }

    /// Matches a BCP 47 code case-insensitively to a built-in Language.
    pub fn from_code(code: &str) -> Option<Self> {
        let trimmed = code.trim();
        Self::ALL
            .iter()
            .find(|&&lang| lang.code().eq_ignore_ascii_case(trimmed))
            .copied()
    }
}

impl From<Language> for LanguageOption {
    fn from(l: Language) -> Self {
        Self {
            code: l.code().to_string(),
            native: l.native().to_string(),
            english: l.english().to_string(),
            flag: l.flag(),
        }
    }
}

/// Returns the default list of all built-in languages as LanguageOptions.
pub fn default_languages() -> Vec<LanguageOption> {
    Language::ALL.iter().map(|&l| l.into()).collect()
}

/// Formats a language option according to the standard convention:
/// 'Native name (English name)', or just 'English' if both names match.
pub fn default_format(opt: &LanguageOption) -> String {
    if opt.native.is_empty() {
        opt.english.clone()
    } else if opt.english.is_empty() || opt.native == opt.english {
        opt.native.clone()
    } else {
        format!("{} ({})", opt.native, opt.english)
    }
}
