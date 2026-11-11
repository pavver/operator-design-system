//! Keyboard shortcuts: Kbd, split_keys, and key_cap.

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
pub fn Kbd(#[prop(into)] keys: String) -> impl IntoView {
    let parts = split_keys(&keys);
    let n = parts.len();
    view! {
        <span class="op-kbd-group" aria-label=keys.clone()>
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
                    view! {
                        <kbd class="op-kbd" data-size=size>
                            <span class="op-kbd-cap">{cap}</span>
                        </kbd>
                        {(i + 1 < n).then(|| view! { <span class="op-kbd-plus" aria-hidden="true">"+"</span> })}
                    }
                })
                .collect_view()}
        </span>
    }
}
