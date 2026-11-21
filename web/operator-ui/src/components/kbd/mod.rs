//! Keyboard shortcuts: Kbd, split_keys, and key_cap.

use crate::core::{Metric, Slot, root_class, root_style, ui_text};
use leptos::prelude::*;

pub(crate) const CSS: &str = include_str!("kbd.css");

/// Splits a shortcut such as `Ctrl+Shift+K` or `Ctrl++` into keys.
pub fn split_keys(keys: &str) -> Vec<String> {
    let mut out: Vec<String> = keys
        .split('+')
        .filter(|k| !k.trim().is_empty())
        .map(|k| k.trim().to_owned())
        .collect();
    if keys.ends_with("++") || keys == "+" {
        out.push("+".into());
    }
    out
}

/// Width class of a key cap, following a physical keyboard.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyWidth {
    Normal,
    /// Modifier and editing keys such as Shift, Enter, Tab, Backspace.
    Wide,
    /// The space bar.
    Space,
}

/// Returns the text printed on a key cap and its width.
pub fn key_cap(key: &str) -> (String, KeyWidth) {
    let lower = key.to_ascii_lowercase();
    let glyph = match lower.as_str() {
        "up" | "arrowup" => Some("\u{2191}"),
        "down" | "arrowdown" => Some("\u{2193}"),
        "left" | "arrowleft" => Some("\u{2190}"),
        "right" | "arrowright" => Some("\u{2192}"),
        _ => None,
    };
    if let Some(g) = glyph {
        return (g.to_owned(), KeyWidth::Normal);
    }
    let width = match lower.as_str() {
        "space" => KeyWidth::Space,
        "shift" | "enter" | "return" | "tab" | "backspace" | "caps" | "capslock" | "ctrl"
        | "control" | "alt" | "option" | "cmd" | "command" | "meta" | "win" | "super" => {
            KeyWidth::Wide
        }
        _ => KeyWidth::Normal,
    };
    (key.to_owned(), width)
}

/// Keyboard key or shortcut, for example `Ctrl+K`.
#[component]
pub fn Kbd(
    #[prop(into)] keys: String,
    /// Key height in DU (`--op-kbd-height`).
    #[prop(optional, into)]
    height: MaybeProp<f32>,
    /// Minimum width of wide keys in DU (`--op-kbd-wide-min-width`).
    #[prop(optional, into)]
    wide_min_width: MaybeProp<f32>,
    /// Minimum width of the space bar in DU (`--op-kbd-space-min-width`).
    #[prop(optional, into)]
    space_min_width: MaybeProp<f32>,
    /// Horizontal key face padding in DU (`--op-kbd-padding-x`).
    #[prop(optional, into)]
    padding_x: MaybeProp<f32>,
    /// Gap between keys and separators in DU (`--op-kbd-gap`).
    #[prop(optional, into)]
    gap: MaybeProp<f32>,
    /// Font size in TU (`--op-kbd-font-size`).
    #[prop(optional, into)]
    font_size: MaybeProp<f32>,
    /// Custom separator slot between keys.
    #[prop(optional, into)]
    separator: Slot,
    /// Accessible label override; defaults to `keys`.
    #[prop(optional, into)]
    aria_label: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let parts = split_keys(&keys);
    let n = parts.len();
    let label = Signal::derive(move || aria_label.get().unwrap_or_else(|| keys.clone()));
    let plus_text = ui_text(None::<String>.into(), "kbd.plus", "+");
    let sep = separator.or(move || {
        let p = plus_text;
        view! { <span class="op-kbd-plus" aria-hidden="true">{move || p.get()}</span> }
    });

    view! {
        <span
            class=root_class("op-kbd-group", class)
            style=root_style(
                vec![
                    Metric::du("--op-kbd-height", height),
                    Metric::du("--op-kbd-wide-min-width", wide_min_width),
                    Metric::du("--op-kbd-space-min-width", space_min_width),
                    Metric::du("--op-kbd-padding-x", padding_x),
                    Metric::du("--op-kbd-gap", gap),
                    Metric::tu("--op-kbd-font-size", font_size),
                ],
                style,
            )
            aria-label=move || label.get()
        >
            {parts
                .into_iter()
                .enumerate()
                .map(|(i, k)| {
                    let (cap, width) = key_cap(&k);
                    let size = match width {
                        KeyWidth::Normal => None,
                        KeyWidth::Wide => Some("wide"),
                        KeyWidth::Space => Some("space"),
                    };
                    let sep = sep.clone();
                    view! {
                        <kbd class="op-kbd" data-size=size>
                            <span class="op-kbd-cap">{cap}</span>
                        </kbd>
                        {(i + 1 < n).then(move || sep.run())}
                    }
                })
                .collect_view()}
        </span>
    }
}
