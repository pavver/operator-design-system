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
#[repr(usize)]
pub enum Language {
    En = 0,
    Uk = 1,
    De = 2,
    Fr = 3,
    Es = 4,
    It = 5,
    Pl = 6,
    Pt = 7,
    Nl = 8,
    Cs = 9,
    Sv = 10,
    Ja = 11,
    Zh = 12,
    Ko = 13,
    Tr = 14,
}

const LANGUAGE_TABLE: &[(Language, &str, &str, &str, &str)] = &[
    (Language::En, "en", "English", "English", "GB"),
    (Language::Uk, "uk", "Українська", "Ukrainian", "UA"),
    (Language::De, "de", "Deutsch", "German", "DE"),
    (Language::Fr, "fr", "Français", "French", "FR"),
    (Language::Es, "es", "Español", "Spanish", "ES"),
    (Language::It, "it", "Italiano", "Italian", "IT"),
    (Language::Pl, "pl", "Polski", "Polish", "PL"),
    (Language::Pt, "pt", "Português", "Portuguese", "PT"),
    (Language::Nl, "nl", "Nederlands", "Dutch", "NL"),
    (Language::Cs, "cs", "Čeština", "Czech", "CZ"),
    (Language::Sv, "sv", "Svenska", "Swedish", "SE"),
    (Language::Ja, "ja", "日本語", "Japanese", "JP"),
    (Language::Zh, "zh", "中文", "Chinese", "CN"),
    (Language::Ko, "ko", "한국어", "Korean", "KR"),
    (Language::Tr, "tr", "Türkçe", "Turkish", "TR"),
];

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
        LANGUAGE_TABLE[*self as usize].1
    }

    /// Returns the native name of the language.
    pub const fn native(&self) -> &'static str {
        LANGUAGE_TABLE[*self as usize].2
    }

    /// Returns the English name of the language.
    pub const fn english(&self) -> &'static str {
        LANGUAGE_TABLE[*self as usize].3
    }

    /// Returns the default flag for this language.
    pub const fn flag(&self) -> Option<Flag> {
        Some(Flag::Builtin(LANGUAGE_TABLE[*self as usize].4))
    }

    /// Matches a BCP 47 code case-insensitively to a built-in Language.
    pub fn from_code(code: &str) -> Option<Self> {
        let trimmed = code.trim();
        LANGUAGE_TABLE
            .iter()
            .find(|(_, c, _, _, _)| c.eq_ignore_ascii_case(trimmed))
            .map(|(lang, _, _, _, _)| *lang)
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
