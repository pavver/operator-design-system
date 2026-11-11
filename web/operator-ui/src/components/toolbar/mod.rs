//! Horizontal bar of commands: Toolbar, ToolbarTitle, ToolbarSeparator, ToolbarSpacer.

use leptos::prelude::*;
use wasm_bindgen::JsCast;

pub(crate) const CSS: &str = include_str!("toolbar.css");

/// Horizontal bar of commands for the view below it.
///
/// Arrow keys move focus between its controls.
#[component]
pub fn Toolbar(
    #[prop(optional, into)] label: Option<String>,
    /// 8 DU tall instead of 10 DU.
    #[prop(optional)]
    dense: bool,
    children: Children,
) -> impl IntoView {
    let bar = NodeRef::<leptos::html::Div>::new();
    let step = move |delta: i32| {
        let Some(el) = bar.try_get_untracked().flatten() else {
            return;
        };
        let Ok(list) = el.query_selector_all(
            "button:not([disabled]):not([aria-disabled=true]), input:not([disabled]), [tabindex='0']",
        ) else {
            return;
        };
        let n = list.length();
        if n == 0 {
            return;
        }
        let active = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.active_element());
        let mut current = 0i64;
        for i in 0..n {
            if let (Some(node), Some(a)) = (list.item(i), active.as_ref())
                && node.contains(Some(a.as_ref()))
            {
                current = i64::from(i);
            }
        }
        let next = (current + i64::from(delta)).rem_euclid(i64::from(n));
        if let Some(item) = u32::try_from(next)
            .ok()
            .and_then(|i| list.item(i))
            .and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok())
        {
            let _ = item.focus();
        }
    };
    view! {
        <div
            node_ref=bar
            class="op-toolbar"
            role="toolbar"
            aria-label=label
            data-dense=dense.then_some("")
            on:keydown=move |ev| {
                let in_input = ev
                    .target()
                    .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                    .is_some();
                if in_input {
                    return;
                }
                match ev.key().as_str() {
                    "ArrowRight" => {
                        ev.prevent_default();
                        step(1);
                    }
                    "ArrowLeft" => {
                        ev.prevent_default();
                        step(-1);
                    }
                    _ => {}
                }
            }
        >
            {children()}
        </div>
    }
}

/// Title text inside a [`Toolbar`].
#[component]
pub fn ToolbarTitle(#[prop(into)] text: Signal<String>) -> impl IntoView {
    view! { <span class="op-toolbar-title">{move || text.get()}</span> }
}

/// Vertical divider between groups of toolbar controls.
#[component]
pub fn ToolbarSeparator() -> impl IntoView {
    view! { <span class="op-toolbar-sep" role="separator" aria-orientation="vertical"></span> }
}

/// Flexible gap that pushes the following controls to the end.
#[component]
pub fn ToolbarSpacer() -> impl IntoView {
    view! { <span class="op-toolbar-spacer"></span> }
}
