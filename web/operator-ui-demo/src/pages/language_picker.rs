//! Live examples for LanguagePicker.

use crate::kit::{Example, backend};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn language_picker() -> AnyView {
    let b = backend();
    let lang_default = RwSignal::new("en".to_string());
    let lang_custom = RwSignal::new("uk".to_string());
    let lang_no_flags = RwSignal::new("de".to_string());
    let lang_customized = RwSignal::new("fr".to_string());

    let custom_languages = vec![
        Language::Uk.into(),
        Language::En.into(),
        Language::De.into(),
        Language::Pl.into(),
        LanguageOption::new("eo", "Esperanto", "Esperanto").with_flag(Flag::None),
    ];

    view! {
        <Example
            title="Default"
            note="Full built-in language list with flags and standard BCP 47 codes. Type to filter by native name, English name, or language code."
            block=true
        >
            <div class="demo-grid-2">
                <LanguagePicker
                    value=lang_default
                    on_select=Callback::new(move |v: String| b.log(format!("[UI] selected: {v}")))
                />
            </div>
            <span class="op-t-caption demo-muted">{move || format!("Committed code: {}", lang_default.get())}</span>
        </Example>

        <Example
            title="Custom list"
            note="Apps can supply a custom subset of languages or completely custom LanguageOption items."
            block=true
        >
            <div class="demo-grid-2">
                <LanguagePicker
                    label="Regional languages"
                    value=lang_custom
                    languages=custom_languages
                    on_select=Callback::new(move |v: String| b.log(format!("[UI] regional selected: {v}")))
                />
            </div>
            <span class="op-t-caption demo-muted">{move || format!("Committed code: {}", lang_custom.get())}</span>
        </Example>

        <Example
            title="Without flags"
            note="Setting show_flags=false hides flags in the suggestion list."
            block=true
        >
            <div class="demo-grid-2">
                <LanguagePicker
                    label="Language (text only)"
                    value=lang_no_flags
                    show_flags=false
                    on_select=Callback::new(move |v: String| b.log(format!("[UI] text-only selected: {v}")))
                />
            </div>
            <span class="op-t-caption demo-muted">{move || format!("Committed code: {}", lang_no_flags.get())}</span>
        </Example>

        <Example
            title="Customization"
            note="Custom flag dimensions (--op-language-picker-flag-width/height), field metrics, and custom label format."
            block=true
        >
            <div class="demo-grid-2">
                <LanguagePicker
                    label="Customized formatting"
                    value=lang_customized
                    flag_width=8.0
                    flag_height=5.33
                    format=Callback::new(|opt: LanguageOption| format!("{} [{}]", opt.native, opt.code))
                    on_select=Callback::new(move |v: String| b.log(format!("[UI] customized selected: {v}")))
                />
            </div>
            <span class="op-t-caption demo-muted">{move || format!("Committed code: {}", lang_customized.get())}</span>
        </Example>
    }
    .into_any()
}
