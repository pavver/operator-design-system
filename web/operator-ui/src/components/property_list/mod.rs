//! List of label/value pairs: PropertyList and Property.

use crate::components::icon_button::IconButton;
use crate::core::copy_action;
use leptos::prelude::*;
use operator_ui_tokens::{dim, icons};

pub(crate) const CSS: &str = include_str!("property_list.css");

/// List of label/value pairs, for example device details.
#[component]
pub fn PropertyList(children: Children) -> impl IntoView {
    view! { <dl class="op-props">{children()}</dl> }
}

/// One row of a [`PropertyList`]; the value is `value` or custom children.
#[component]
pub fn Property(
    #[prop(into)] label: String,
    #[prop(optional, into)] value: Option<Signal<String>>,
    /// Adds a copy button for the value.
    #[prop(optional)]
    copyable: bool,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let copy = value.filter(|_| copyable).map(copy_action);
    view! {
        <div class="op-prop">
            <dt class="op-prop-label">{label}</dt>
            <dd class="op-prop-value">
                <span class="op-prop-text">
                    {value.map(|v| move || v.get())}
                    {children.map(|c| c())}
                </span>
                {copy
                    .map(|action| {
                        view! {
                            <IconButton
                                icon=icons::CONTENT_COPY
                                label="Copy value"
                                size=dim::ROW_DENSE
                                icon_size=3.5
                                on_async_click=action
                            />
                        }
                    })}
            </dd>
        </div>
    }
}
