//! ColorField: a hex colour with swatches.

use super::parse::parse_color;
use super::parsed::ParsedField;
use crate::components::popover::{Placement, Popover, PopoverAnchor};
use crate::components::primitives::Icon;
use crate::core::ControlState;
use leptos::prelude::*;
use operator_ui_tokens::icons;
use std::sync::Arc;

/// Colour input as `#rrggbb` with a preview square, preset swatches and the system colour picker.
#[component]
pub fn ColorField(
    value: RwSignal<String>,
    /// Preset colours offered in the popup, as `#rrggbb`.
    #[prop(optional)]
    swatches: Vec<String>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(optional)] on_commit: Option<Callback<String>>,
    /// Label of the system picker button.
    #[prop(default = "Custom…".into(), into)]
    custom_label: Signal<String>,
) -> impl IntoView {
    let format = Callback::new(|v: String| v);
    let parse = Callback::new(|t: String| parse_color(&t));
    let open = RwSignal::new(false);
    let host = NodeRef::<leptos::html::Div>::new();
    let anchor = Signal::derive(move || {
        host.get()
            .map(|el| PopoverAnchor::of(&el))
            .unwrap_or(PopoverAnchor::Point { x: 0.0, y: 0.0 })
    });
    let set = move |c: String| {
        if c != value.get_untracked() {
            value.set(c.clone());
            if let Some(cb) = on_commit {
                cb.run(c);
            }
        }
    };
    let trailing: ChildrenFn = Arc::new(move || {
        view! {
            <button
                type="button"
                class="op-rippled op-field-reveal op-color-trigger"
                aria-label="Pick a colour"
                aria-expanded=move || open.get().to_string()
                disabled=move || !enabled.get()
                on:mousedown=|ev| ev.prevent_default()
                on:click=move |_| open.update(|o| *o = !*o)
            >
                <span class="op-color-preview" style:background-color=move || value.get()></span>
                <Icon icon=icons::ARROW_DROP_DOWN size=5.0/>
            </button>
        }
        .into_any()
    });
    let swatches = StoredValue::new(swatches);
    view! {
        <div class="op-value-anchor" node_ref=host>
            <ParsedField
                value=value
                format=format
                parse=parse
                label=label
                enabled=enabled
                state=state
                class="op-color-field"
                on_commit=on_commit
                trailing=Some(trailing)
            />
        </div>
        <Popover open=open anchor=anchor placement=Placement::BelowEnd class="op-value-popup" role="dialog" label="Colours">
            <div class="op-color-popup">
                <div class="op-color-swatches" role="listbox">
                    {swatches
                        .get_value()
                        .into_iter()
                        .filter_map(|c| parse_color(&c))
                        .map(|c| {
                            let pick = c.clone();
                            let current = c.clone();
                            view! {
                                <button
                                    type="button"
                                    role="option"
                                    class="op-color-swatch"
                                    aria-label=c.clone()
                                    aria-selected=move || (value.get() == current).to_string()
                                    style:background-color=c
                                    on:click=move |_| {
                                        set(pick.clone());
                                        open.set(false);
                                    }
                                ></button>
                            }
                        })
                        .collect_view()}
                </div>
                <label class="op-rippled op-color-custom">
                    <input
                        type="color"
                        prop:value=move || value.get()
                        on:change=move |ev| {
                            if let Some(c) = parse_color(&event_target_value(&ev)) {
                                set(c);
                            }
                        }
                    />
                    <span>{move || custom_label.get()}</span>
                </label>
            </div>
        </Popover>
    }
}
