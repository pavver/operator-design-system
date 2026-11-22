//! Compact token: Chip and ChipSet.

use crate::core::{
    ControlState, Metric, Press, Slot, root_class, root_style, spawn_ripple, ui_icon, ui_text,
    use_state_visual,
};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, icons};

use crate::core::glyph::symbol;

const ICON_SIZE: &str = "var(--op-chip-icon-size, calc(var(--op-du) * 3.5))";

pub(crate) const CSS: &str = include_str!("chip.css");

/// Color of a [`Chip`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ChipColor {
    /// Surface background and primary text.
    #[default]
    Neutral,
    Accent,
    Success,
    Warning,
    Error,
    Info,
    /// Any CSS color, for user-defined tags, for example `"#3FA7D6"`.
    /// The chip tints its background and mixes the text toward `text_primary` so it stays readable in both modes.
    Custom(String),
}

impl ChipColor {
    fn attr(&self) -> Option<&'static str> {
        match self {
            Self::Neutral => None,
            Self::Accent => Some("accent"),
            Self::Success => Some("success"),
            Self::Warning => Some("warning"),
            Self::Error => Some("error"),
            Self::Info => Some("info"),
            Self::Custom(_) => Some("custom"),
        }
    }

    /// Inline style for a custom color; characters that could end the declaration are dropped.
    fn style(&self) -> Option<String> {
        match self {
            Self::Custom(c) => {
                let clean: String = c
                    .chars()
                    .filter(|ch| !matches!(ch, ';' | '{' | '}' | '<' | '>' | '"' | '\\'))
                    .collect();
                Some(format!("--chip-c: {clean}"))
            }
            _ => None,
        }
    }
}

/// Compact token: a toggleable filter, a removable value, or a static tag.
#[component]
pub fn Chip(
    #[prop(optional, into)] text: MaybeProp<String>,
    #[prop(optional)] icon: Option<IconData>,
    /// Makes the chip a toggle (filter chip).
    #[prop(optional, into)]
    selected: Option<RwSignal<bool>>,
    #[prop(optional, into)] on_click: Option<Callback<()>>,
    /// Shows a remove button (input chip).
    #[prop(optional, into)]
    on_remove: Option<Callback<()>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    /// Tone or custom color.
    #[prop(optional, into)]
    color: Signal<ChipColor>,
    #[prop(optional, into)] height: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] padding_x: MaybeProp<f32>,
    #[prop(optional, into)] remove_size: MaybeProp<f32>,
    #[prop(optional, into)] remove_overlap: MaybeProp<f32>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] leading: Slot,
    #[prop(optional, into)] trailing: Slot,
    #[prop(optional, into)] content: Slot,
    #[prop(optional, into)] remove_label: MaybeProp<String>,
    #[prop(optional)] checked_icon: Option<IconData>,
    #[prop(optional)] remove_icon: Option<IconData>,
) -> impl IntoView {
    let text = ui_text(text, "chip.text", "");
    let remove_label = ui_text(remove_label, "chip.remove", "Remove {text}");
    let checked_icon = ui_icon(checked_icon, "chip.checked", icons::CHECK);
    let remove_icon = ui_icon(remove_icon, "chip.remove", icons::CLOSE);

    let root_metrics = vec![
        Metric::du("--op-chip-height", height),
        Metric::du("--op-chip-gap", gap),
        Metric::du("--op-chip-padding-x", padding_x),
        Metric::du("--op-chip-remove-size", remove_size),
        Metric::du("--op-chip-remove-overlap", remove_overlap),
        Metric::du("--op-chip-icon-size", icon_size),
        Metric::tu("--op-chip-font-size", font_size),
        Metric::tu("--op-chip-line-height", line_height),
    ];

    let custom_style = Signal::derive(move || {
        let mut value = color.with(ChipColor::style).unwrap_or_default();
        value.push(';');
        if let Some(extra) = style.get() {
            value.push_str(&extra);
        }
        value
    });
    let press = Press::new();
    let interactive = selected.is_some() || on_click.is_some();
    let is_selected = move || selected.is_some_and(|s| s.get());
    let state = Signal::derive(move || {
        if !enabled.get() {
            ControlState::Disabled
        } else if press.pressed.get() {
            ControlState::Pressed
        } else if is_selected() {
            ControlState::Selected
        } else {
            ControlState::Idle
        }
    });
    let visual = use_state_visual(state);
    let main_content = move || {
        view! {
            {(selected.is_some())
                .then(|| {
                    view! {
                        <Show when=is_selected>
                            <span class="op-chip-check">{symbol(checked_icon, ICON_SIZE, None)}</span>
                        </Show>
                    }
                })}
            {icon.map(|i| view! { {symbol(Signal::stored(i), ICON_SIZE, None)} })}
            {leading.run()}
            {if content.is_set() { content.run() } else { Some(view! { <span class="op-chip-text">{move || text.get()}</span> }.into_any()) }}
            {trailing.run()}
        }
    };
    let main = if interactive {
        view! {
            <button
                type="button"
                class="op-rippled op-chip-main"
                role=selected.is_some().then_some("checkbox")
                aria-checked=move || selected.map(|s| s.get().to_string())
                aria-disabled=move || (!enabled.get()).then_some("true")
                on:pointerdown=move |ev| {
                    if enabled.get_untracked() {
                        press.down();
                        spawn_ripple(&ev);
                    }
                }
                on:pointerup=move |_| press.up()
                on:pointerleave=move |_| press.up()
                on:pointercancel=move |_| press.up()
                on:click=move |_| {
                    if !enabled.get_untracked() {
                        return;
                    }
                    if let Some(s) = selected {
                        s.update(|v| *v = !*v);
                    }
                    if let Some(cb) = on_click {
                        cb.run(());
                    }
                }
            >
                {main_content()}
            </button>
        }
        .into_any()
    } else {
        view! { <span class="op-chip-main">{main_content()}</span> }.into_any()
    };
    view! {
        <span
            class=root_class("op-sb op-chip", class)
            style=root_style(root_metrics, custom_style.into())
            data-state=move || visual.state.get()
            data-result=move || visual.result.get()
            data-fading=move || visual.fading.get().then_some("")
            data-selected=move || is_selected().then_some("")
            data-color=move || color.with(ChipColor::attr)

        >
            {main}
            {on_remove
                .map(|cb| {
                    view! {
                        <button
                            type="button"
                            class="op-rippled op-chip-remove"
                            aria-label=move || remove_label.get().replace("{text}", &text.get())
                            disabled=move || !enabled.get()
                            on:pointerdown=move |ev| spawn_ripple(&ev)
                            on:click=move |_| cb.run(())
                        >
                            {symbol(remove_icon, ICON_SIZE, None)}
                        </button>
                    }
                })}
        </span>
    }
}

/// Row of chips that wraps onto new lines.
#[component]
pub fn ChipSet(
    children: Children,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    view! { <div class=root_class("op-chip-set", class) style=root_style(vec![Metric::du("--op-chip-set-gap", gap)], style)>{children()}</div> }
}

#[cfg(test)]
mod tests {
    use super::ChipColor;

    #[test]
    fn tones_map_to_attributes() {
        assert_eq!(ChipColor::Neutral.attr(), None);
        assert_eq!(ChipColor::Error.attr(), Some("error"));
        assert_eq!(ChipColor::Custom("#fff".into()).attr(), Some("custom"));
    }

    #[test]
    fn custom_style_drops_declaration_breakers() {
        let style = ChipColor::Custom("red; background: url(x)".into()).style();
        assert_eq!(style.as_deref(), Some("--chip-c: red background: url(x)"));
        assert_eq!(ChipColor::Info.style(), None);
    }
}
