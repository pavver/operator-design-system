//! Unit tests for LanguagePicker models and logic.

use super::*;

#[test]
fn test_language_enum_coverage() {
    assert_eq!(Language::ALL.len(), 15);
    for &lang in Language::ALL {
        assert!(!lang.code().is_empty());
        assert!(!lang.native().is_empty());
        assert!(!lang.english().is_empty());
        assert!(lang.flag().is_some());
    }
}

#[test]
fn test_language_from_code() {
    assert_eq!(Language::from_code("en"), Some(Language::En));
    assert_eq!(Language::from_code("EN"), Some(Language::En));
    assert_eq!(Language::from_code("  uk  "), Some(Language::Uk));
    assert_eq!(Language::from_code("de"), Some(Language::De));
    assert_eq!(Language::from_code("unknown"), None);
}

#[test]
fn test_language_conversion() {
    let opt: LanguageOption = Language::Uk.into();
    assert_eq!(opt.code, "uk");
    assert_eq!(opt.native, "Українська");
    assert_eq!(opt.english, "Ukrainian");
    assert_eq!(opt.flag, Some(Flag::Builtin("UA")));
}

#[test]
fn test_default_format() {
    let en: LanguageOption = Language::En.into();
    assert_eq!(default_format(&en), "English");

    let uk: LanguageOption = Language::Uk.into();
    assert_eq!(default_format(&uk), "Українська (Ukrainian)");

    let de: LanguageOption = Language::De.into();
    assert_eq!(default_format(&de), "Deutsch (German)");

    let custom = LanguageOption::new("custom", "Мова", "");
    assert_eq!(default_format(&custom), "Мова");
}

#[test]
fn test_flags_builtin() {
    let codes = [
        "UA", "GB", "US", "DE", "FR", "ES", "IT", "PL", "PT", "NL", "CZ", "SE", "JP", "CN", "KR",
        "TR",
    ];
    for code in codes {
        assert!(
            builtin_flag_view(code).is_some(),
            "Flag {code} should be present"
        );
        assert!(
            builtin_flag_view(&code.to_lowercase()).is_some(),
            "Flag {code} lowercase should be present"
        );
    }
    assert!(builtin_flag_view("XX").is_none());
}

#[test]
fn test_render_flag() {
    let ua = render_flag(&Flag::Builtin("UA"));
    let none = render_flag(&Flag::None);
    let _ = (ua, none);
}
