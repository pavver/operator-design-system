//! List of label/value pairs: PropertyList and Property.

use crate::components::icon_button::IconButton;
use crate::core::{
    Metric, ScopedSlot, Slot, copy_action, root_class, root_style, ui_icon, ui_text,
};
use leptos::prelude::*;
use operator_ui_tokens::icons;
use send_wrapper::SendWrapper;
use wasm_bindgen::{JsCast, closure::Closure};

pub(crate) const CSS: &str = include_str!("property_list.css");

/// Current label and value passed to a property slot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PropertyValue {
    pub label: String,
    pub value: Option<String>,
}

#[derive(Clone, Copy)]
struct PropertySlots {
    option: Option<ScopedSlot<PropertyValue>>,
    value: Option<ScopedSlot<PropertyValue>>,
}

/// List of label/value pairs, for example device details.
#[component]
pub fn PropertyList(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] option: Option<ScopedSlot<PropertyValue>>,
    #[prop(optional)] value_slot: Option<ScopedSlot<PropertyValue>>,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] footer: Slot,
    #[prop(optional, into)] before_options: Slot,
    #[prop(optional, into)] after_options: Slot,
    #[prop(optional, into)] empty: Slot,
    #[prop(optional, into)] row_height: MaybeProp<f32>,
    #[prop(optional, into)] padding_y: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] value_gap: MaybeProp<f32>,
    #[prop(optional, into)] copy_size: MaybeProp<f32>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] stack_width: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] label_font_size: MaybeProp<f32>,
    #[prop(optional, into)] label_line_height: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    provide_context(PropertySlots {
        option,
        value: value_slot,
    });
    let node = NodeRef::<leptos::html::Dl>::new();
    let breakpoint = NodeRef::<leptos::html::Div>::new();
    let stacked = RwSignal::new(false);
    Effect::new(move |_| {
        let (Some(root), Some(marker)) = (node.get(), breakpoint.get()) else {
            return;
        };
        let refresh = move || {
            stacked.set(
                root.get_bounding_client_rect().width()
                    <= marker.get_bounding_client_rect().width(),
            )
        };
        refresh();
        let callback = Closure::<dyn FnMut()>::new(refresh);
        if let Ok(observer) = web_sys::ResizeObserver::new(callback.as_ref().unchecked_ref()) {
            if let Some(root) = node.get_untracked() {
                observer.observe(&root);
            }
            if let Some(marker) = breakpoint.get_untracked() {
                observer.observe(&marker);
            }
            let keep = SendWrapper::new((observer, callback));
            on_cleanup(move || keep.0.disconnect());
        }
    });
    let metrics = vec![
        Metric::du("--op-property-list-row-height", row_height),
        Metric::du("--op-property-list-padding-y", padding_y),
        Metric::du("--op-property-list-gap", gap),
        Metric::du("--op-property-list-value-gap", value_gap),
        Metric::du("--op-property-list-copy-size", copy_size),
        Metric::du("--op-property-list-icon-size", icon_size),
        Metric::du("--op-property-list-stack-width", stack_width),
        Metric::tu("--op-property-list-font-size", font_size),
        Metric::tu("--op-property-list-line-height", line_height),
        Metric::tu("--op-property-list-label-font-size", label_font_size),
        Metric::tu("--op-property-list-label-line-height", label_line_height),
    ];
    view! {
        <dl node_ref=node class=root_class("op-props", class) style=root_style(metrics, style) data-stacked=move || stacked.get().then_some("")>
            <div class="op-props-measure" aria-hidden="true"><div node_ref=breakpoint class="op-props-breakpoint"></div></div>
            {header.is_set().then(|| view! { <div>{header.run()}</div> })}
            {before_options.is_set().then(|| view! { <div>{before_options.run()}</div> })}
            {children.map_or_else(|| empty.run().into_any(), |c| c().into_any())}
            {after_options.is_set().then(|| view! { <div>{after_options.run()}</div> })}
            {footer.is_set().then(|| view! { <div>{footer.run()}</div> })}
        </dl>
    }
}

/// One row of a PropertyList; the value is `value` or custom children.
#[component]
pub fn Property(
    #[prop(into)] label: String,
    #[prop(optional, into)] value: Option<Signal<String>>,
    #[prop(optional)] copyable: bool,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] value_slot: Option<ScopedSlot<PropertyValue>>,
    #[prop(optional, into)] copy_label: MaybeProp<String>,
    #[prop(optional)] copy_icon: Option<operator_ui_tokens::Icon>,
    #[prop(optional, into)] row_height: MaybeProp<f32>,
    #[prop(optional, into)] padding_y: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] value_gap: MaybeProp<f32>,
    #[prop(optional, into)] copy_size: MaybeProp<f32>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] label_font_size: MaybeProp<f32>,
    #[prop(optional, into)] label_line_height: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let copy = value.filter(|_| copyable).map(copy_action);
    let copy_label = ui_text(copy_label, "property-list.copy", "Copy value");
    let copy_icon = ui_icon(copy_icon, "property-list.copy", icons::CONTENT_COPY);
    let inherited = use_context::<PropertySlots>();
    let value_slot = value_slot.or_else(|| inherited.and_then(|s| s.value));
    let option = inherited.and_then(|s| s.option);
    let data_label = label.clone();
    let data = move || PropertyValue {
        label: data_label.clone(),
        value: value.map(|v| v.get()),
    };
    let metrics = vec![
        Metric::du("--op-property-list-row-height", row_height),
        Metric::du("--op-property-list-padding-y", padding_y),
        Metric::du("--op-property-list-gap", gap),
        Metric::du("--op-property-list-value-gap", value_gap),
        Metric::du("--op-property-list-copy-size", copy_size),
        Metric::du("--op-property-list-icon-size", icon_size),
        Metric::tu("--op-property-list-font-size", font_size),
        Metric::tu("--op-property-list-line-height", line_height),
        Metric::tu("--op-property-list-label-font-size", label_font_size),
        Metric::tu("--op-property-list-label-line-height", label_line_height),
    ];
    view! {
        <div class=root_class("op-prop", class) style=root_style(metrics, style)>
            {if let Some(slot) = option { (move || slot.run(data())).into_any() } else { view! {
            <dt class="op-prop-label">{label}</dt>
            <dd class="op-prop-value">
                <span class="op-prop-text">
                    {if let Some(slot) = value_slot { (move || slot.run(data())).into_any() } else { view! {
                        {value.map(|v| move || v.get())}
                        {children.map(|c| c())}
                    }.into_any() }}
                </span>
                {copy.map(|action| view! {
                    <IconButton
                        icon=copy_icon.get()
                        label=copy_label
                        content=move || view! { <crate::components::primitives::Icon icon=copy_icon.get() style="--op-icon-size:var(--op-property-list-icon-size, var(--op-dim-icon-xsmall))"/> }
                        style="--op-button-size:var(--op-property-list-copy-size, var(--op-dim-row-dense));--op-button-icon-size:var(--op-property-list-icon-size, var(--op-dim-icon-xsmall))"
                        on_async_click=action
                    />
                })}
            </dd>
            }.into_any() }}
        </div>
    }
}
