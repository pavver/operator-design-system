//! The popup plumbing shared by value fields with a picker button.

use super::portal_style::{field_popup_label, field_popup_style};
use crate::components::popover::PopoverAnchor;
use crate::components::primitives::Icon;
use leptos::html::Div;
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, dim};
use std::sync::Arc;

/// Signals and refs for a field popup anchored under the field box.
#[derive(Clone, Copy)]
pub(super) struct PickerPopup {
    pub open: RwSignal<bool>,
    /// The field wrapper; the anchor is its `.op-field` box.
    pub host: NodeRef<Div>,
    pub popup_ref: NodeRef<Div>,
    pub anchor: Signal<PopoverAnchor>,
    /// Root metric overrides carried into the portalled popup.
    pub style: Signal<Option<String>>,
}

impl PickerPopup {
    /// Creates the popup state; it closes while `blocked` is true.
    pub fn new(
        custom_style: Signal<Option<String>>,
        label: Signal<String>,
        blocked: impl Fn() -> bool + Send + Sync + 'static,
    ) -> Self {
        let open = RwSignal::new(false);
        let popup_ref = NodeRef::<Div>::new();
        field_popup_label(popup_ref, label);
        Effect::new(move |_| {
            if blocked() {
                open.set(false);
            }
        });
        let host = NodeRef::<Div>::new();
        let style = field_popup_style(host, open, custom_style);
        let anchor = Signal::derive(move || {
            host.get()
                .map(|el| {
                    el.query_selector(".op-field")
                        .ok()
                        .flatten()
                        .map(|field| PopoverAnchor::of(&field))
                        .unwrap_or_else(|| PopoverAnchor::of(&el))
                })
                .unwrap_or(PopoverAnchor::Point { x: 0.0, y: 0.0 })
        });
        Self {
            open,
            host,
            popup_ref,
            anchor,
            style,
        }
    }

    /// The trailing button that toggles the popup.
    pub fn button(
        self,
        label: Signal<String>,
        icon: Signal<IconData>,
        blocked: impl Fn() -> bool + Copy + Send + Sync + 'static,
    ) -> ChildrenFn {
        let open = self.open;
        Arc::new(move || {
            view! {
                <button
                    type="button"
                    class="op-rippled op-field-reveal"
                    aria-label=move || label.get()
                    aria-expanded=move || open.get().to_string()
                    disabled=blocked
                    on:mousedown=|ev| ev.prevent_default()
                    on:click=move |_| open.update(|o| *o = !*o)
                >
                    {move || view! { <Icon icon=icon.get() size=dim::ICON_FIELD attr:style="width:var(--op-field-icon-size, var(--op-dim-icon-field));height:var(--op-field-icon-size, var(--op-dim-icon-field));"/> }}
                </button>
            }
            .into_any()
        })
    }
}
