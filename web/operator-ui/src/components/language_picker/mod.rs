//! Language selection combobox with built-in flags and BCP 47 codes.

mod flags;
mod types;

pub use flags::builtin_flag_view;
pub use types::{Flag, Language, LanguageOption, default_format, default_languages};

use crate::components::combobox::{ComboOption, Combobox};
use crate::components::field::FieldVariant;
use crate::core::{ControlState, Metric, ScopedSlot, Slot, root_style, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::Icon as IconData;

pub(crate) const CSS: &str = include_str!("language_picker.css");

/// Renders a flag icon inside the standard flag container.
pub fn render_flag(flag: &Flag) -> AnyView {
    match flag {
        Flag::Builtin(code) => {
            if let Some(svg) = builtin_flag_view(code) {
                view! { <span class="op-language-picker-flag" role="img" aria-hidden="true">{svg}</span> }.into_any()
            } else {
                ().into_any()
            }
        }
        Flag::Svg(svg) => {
            view! { <span class="op-language-picker-flag" role="img" aria-hidden="true" inner_html=svg.clone()></span> }.into_any()
        }
        Flag::None => ().into_any(),
    }
}

/// Language picker combobox variant for choosing an interface language.
#[component]
pub fn LanguagePicker(
    /// Committed BCP 47 language code (e.g. "uk", "en", "de").
    value: RwSignal<String>,
    /// Available language options; defaults to all built-in languages.
    #[prop(default = Signal::derive(default_languages), into)]
    languages: Signal<Vec<LanguageOption>>,
    /// Whether flags are displayed next to language labels.
    #[prop(default = true.into(), into)]
    show_flags: Signal<bool>,
    /// Custom formatter turning a [`LanguageOption`] into a display label.
    #[prop(optional, into)]
    format: Option<Callback<LanguageOption, String>>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional, into)] hint: MaybeProp<String>,
    #[prop(optional, into)] error: MaybeProp<String>,
    #[prop(optional, into)] variant: Signal<FieldVariant>,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional, into)] prepend: Slot,
    #[prop(optional, into)] append: Slot,
    #[prop(optional, into)] before: Slot,
    #[prop(optional, into)] after: Slot,
    #[prop(optional, into)] label_slot: Slot,
    #[prop(optional, into)] hint_slot: Slot,
    #[prop(optional, into)] height: MaybeProp<f32>,
    #[prop(optional, into)] padding_x: MaybeProp<f32>,
    #[prop(optional, into)] slot_size: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] flag_width: MaybeProp<f32>,
    #[prop(optional, into)] flag_height: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(optional)] icon: Option<IconData>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] row_height: MaybeProp<f32>,
    #[prop(optional, into)] max_height: MaybeProp<f32>,
    #[prop(optional, into)] option_gap: MaybeProp<f32>,
    #[prop(optional, into)] option_padding_x: MaybeProp<f32>,
    #[prop(optional, into)] option_icon_size: MaybeProp<f32>,
    #[prop(optional, into)] detail_line_height: MaybeProp<f32>,
    #[prop(optional, into)] letter_spacing: MaybeProp<f32>,
    #[prop(optional, into)] detail_font_size: MaybeProp<f32>,
    #[prop(optional, into)] section_padding_y: MaybeProp<f32>,
    /// Scoped slot replacing default option row content.
    #[prop(optional)]
    option: Option<ScopedSlot<LanguageOption>>,
    #[prop(optional, into)] empty: Slot,
    #[prop(optional, into)] before_options: Slot,
    #[prop(optional, into)] after_options: Slot,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] footer: Slot,
    #[prop(optional, into)] on_select: Option<Callback<String>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    /// Row shown when no language matches.
    #[prop(optional, into)]
    empty_text: MaybeProp<String>,
) -> impl IntoView {
    let _ = icon;
    let default_label = ui_text(label, "language-picker.label", "Language");
    let aria_name = ui_text(aria_label, "language-picker.label", "Language");
    let picker_label = MaybeProp::derive(move || {
        if label.get().is_some() {
            label.get()
        } else if aria_label.get().is_some() {
            None
        } else {
            Some(default_label.get())
        }
    });
    let picker_aria_label = MaybeProp::derive(move || {
        if label.get().is_some() {
            label.get()
        } else if aria_label.get().is_some() {
            aria_label.get()
        } else {
            Some(aria_name.get())
        }
    });
    let empty_text = ui_text(empty_text, "language-picker.empty", "No matching language");
    let picker_empty_text = MaybeProp::derive(move || Some(empty_text.get()));

    let metrics = vec![
        Metric::du("--op-language-picker-flag-width", flag_width),
        Metric::du("--op-language-picker-flag-height", flag_height),
    ];
    let picker_style = MaybeProp::derive(root_style(metrics, style));
    let picker_class = MaybeProp::derive(move || {
        let extra = class.get();
        match extra {
            Some(c) if !c.trim().is_empty() => Some(format!("op-language-picker {c}")),
            _ => Some("op-language-picker".to_string()),
        }
    });

    let format_cb = format;
    let combo_options = Memo::new(move |_| {
        languages.with(|langs| {
            langs
                .iter()
                .map(|l| {
                    let formatted = match &format_cb {
                        Some(cb) => cb.run(l.clone()),
                        None => default_format(l),
                    };
                    ComboOption::new(&l.code).label(formatted)
                })
                .collect::<Vec<ComboOption>>()
        })
    });

    let user_option = option;
    let combo_option_slot: ScopedSlot<ComboOption> =
        Callback::new(move |combo_opt: ComboOption| {
            let lang_opt = languages
                .with(|langs| langs.iter().find(|l| l.code == combo_opt.value).cloned())
                .unwrap_or_else(|| LanguageOption {
                    code: combo_opt.value.clone(),
                    native: combo_opt.label.clone(),
                    english: String::new(),
                    flag: None,
                });

            if let Some(ref slot) = user_option {
                slot.run(lang_opt)
            } else {
                let label_text = combo_opt.label.clone();
                view! {
                    {move || {
                        if show_flags.get() {
                            lang_opt.flag.as_ref().map(render_flag)
                        } else {
                            None
                        }
                    }}
                    <span class="op-combo-label">{label_text}</span>
                }
                .into_any()
            }
        });

    let handle_select = Callback::new(move |selected_code: String| {
        if let Some(cb) = on_select {
            cb.run(selected_code);
        }
    });

    view! {
        <Combobox
            options=combo_options
            selected=value
            label=picker_label
            aria_label=picker_aria_label
            empty_text=picker_empty_text
            placeholder=placeholder
            hint=hint
            error=error
            variant=variant
            prepend=prepend
            append=append
            before=before
            after=after
            label_slot=label_slot
            hint_slot=hint_slot
            height=height
            padding_x=padding_x
            slot_size=slot_size
            gap=gap
            font_size=font_size
            class=picker_class
            style=picker_style
            state=state
            icon_size=icon_size
            row_height=row_height
            max_height=max_height
            option_gap=option_gap
            option_padding_x=option_padding_x
            option_icon_size=option_icon_size
            detail_line_height=detail_line_height
            letter_spacing=letter_spacing
            detail_font_size=detail_font_size
            section_padding_y=section_padding_y
            option=combo_option_slot
            empty=empty
            before_options=before_options
            after_options=after_options
            header=header
            footer=footer
            on_select=handle_select
            enabled=enabled
        />
    }
}

#[cfg(test)]
mod tests;
