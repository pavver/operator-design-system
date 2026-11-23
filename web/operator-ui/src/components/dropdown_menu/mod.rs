//! Dropdown menu: DropdownMenu.

use crate::components::field::{Field, FieldMetrics, FieldVariant};
use crate::components::popover::{Placement, Popover, PopoverAnchor};
use crate::components::primitives::{Icon, ProgressIndicator};
mod field_list;
pub(super) mod portal_style;
use crate::core::{
    ControlState, Metric, Press, ScopedSlot, Slot, root_class, root_style, spawn_ripple, ui_icon,
    ui_text, use_pending_indicator,
};
pub(crate) use field_list::{FieldList, FieldListHandle};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, dim, icons};
pub(crate) use portal_style::{field_popup_label, field_popup_style};

pub(crate) const CSS: &str = include_str!("dropdown_menu.css");

/// Height limit of the option list, in DU.
pub(crate) const MENU_MAX_HEIGHT: f32 = 50.0;

/// Rectangular dropdown with a scrollable option list (max 50 DU).
#[component]
pub fn DropdownMenu(
    options: Vec<String>,
    selected: RwSignal<String>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
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
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional)] icon: Option<IconData>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] spinner_size: MaybeProp<f32>,
    #[prop(optional, into)] spinner_stroke: MaybeProp<f32>,
    #[prop(optional, into)] loading_label: MaybeProp<String>,
    #[prop(optional, into)] row_height: MaybeProp<f32>,
    #[prop(optional, into)] max_height: MaybeProp<f32>,
    #[prop(optional, into)] option_padding_x: MaybeProp<f32>,
    #[prop(optional, into)] trigger_gap: MaybeProp<f32>,
    #[prop(optional)] option: Option<ScopedSlot<String>>,
    #[prop(optional, into)] empty: Slot,
    #[prop(optional, into)] before_options: Slot,
    #[prop(optional, into)] after_options: Slot,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] footer: Slot,
    #[prop(optional, into)] on_select: Option<Callback<String>>,
) -> impl IntoView {
    let expanded = RwSignal::new(false);
    let press = Press::new();
    let external = Signal::derive(move || state.get().unwrap_or_default());

    let border_state = Signal::derive(move || {
        if !enabled.get() {
            ControlState::Disabled
        } else if external.get() != ControlState::Idle {
            external.get()
        } else if press.pressed.get() {
            ControlState::Pressed
        } else if expanded.get() {
            ControlState::Selected
        } else {
            ControlState::Idle
        }
    });
    let name = ui_text(aria_label, "dropdown-menu.label", "Choose a value");
    let accessible_name = Signal::derive(move || label.get().unwrap_or_else(|| name.get()));
    let icon = ui_icon(icon, "dropdown-menu.expand", icons::ARROW_DROP_DOWN);
    let loading_label = ui_text(loading_label, "dropdown-menu.loading", "Loading");
    let mut metrics = FieldMetrics {
        height,
        padding_x,
        slot_size,
        gap,
        font_size,
    }
    .vars();
    metrics.extend([
        Metric::du("--op-field-icon-size", icon_size),
        Metric::du("--op-field-spinner-size", spinner_size),
        Metric::du("--op-field-spinner-stroke", spinner_stroke),
        Metric::du("--op-field-row-height", row_height),
        Metric::du("--op-field-max-height", max_height),
        Metric::du("--op-field-option-padding-x", option_padding_x),
        Metric::du("--op-field-trigger-gap", trigger_gap),
    ]);
    let custom_style = Signal::derive(root_style(metrics, style));
    let show_spinner = use_pending_indicator(move || external.get() == ControlState::Pending);
    let clickable = move || {
        enabled.get()
            && !matches!(
                external.get(),
                ControlState::Disabled | ControlState::Pending
            )
    };
    Effect::new(move |_| {
        if !clickable() {
            expanded.set(false);
        }
    });

    let options = StoredValue::new(options);
    let list = FieldListHandle::new();
    let anchor_ref = NodeRef::<leptos::html::Div>::new();
    let anchor = anchor_signal(anchor_ref);
    let popup_style = field_popup_style(anchor_ref, expanded, custom_style);
    let popup_ref = NodeRef::<leptos::html::Div>::new();
    field_popup_label(popup_ref, accessible_name);
    // Opening scrolls the selected option into the middle of the list.
    Effect::new(move |_| {
        if expanded.get()
            && let Some(index) =
                options.with_value(|o| o.iter().position(|v| selected.with_untracked(|s| s == v)))
        {
            list.reveal(index, true);
        }
    });

    let option_slot = option;
    view! {
        <div class=root_class("op-dropdown", class) style=move || custom_style.get()>
            <div class="op-dropdown-anchor" node_ref=anchor_ref>
                <Field state=border_state label=label hint=hint error=error variant=variant
                    prepend=prepend append=append before=before after=after label_slot=label_slot hint_slot=hint_slot
                    box_class="op-dropdown-box" pending_run=true
                >
                <button
                    type="button"
                    class="op-field-input op-rippled op-dropdown-trigger"
                    aria-label=move || accessible_name.get()
                    aria-haspopup="listbox"
                    aria-expanded=move || expanded.get().to_string()
                    aria-disabled=move || (!clickable()).then_some("true")
                    disabled=move || !clickable()
                    aria-invalid=move || error.get().is_some().then_some("true")
                    on:pointerdown=move |ev| {
                        if clickable() {
                            press.down();
                            spawn_ripple(&ev);
                        }
                    }
                    on:pointerup=move |_| press.up()
                    on:pointerleave=move |_| press.up()
                    on:pointercancel=move |_| press.up()
                    on:click=move |_| {
                        if clickable() {
                            expanded.update(|e| *e = !*e);
                        }
                    }
                    on:keydown=move |ev| {
                        if ev.key() == "Escape" {
                            expanded.set(false);
                        }
                    }
                >
                    <span class="op-dropdown-value">{move || selected.get()}</span>
                    {move || {
                        if show_spinner.get() {
                            view! {
                                <span class="op-pending-indicator op-dropdown-spinner">
                                    <ProgressIndicator size=dim::ICON_SMALL stroke=dim::SPINNER_STROKE
                                        color="var(--op-field-pending, var(--op-pending))"
                                        attr:aria-label=move || loading_label.get()
                                        attr:style="width:var(--op-field-spinner-size, var(--op-dim-icon-small));height:var(--op-field-spinner-size, var(--op-dim-icon-small));"/>
                                </span>
                            }
                                .into_any()
                        } else {
                            view! { <span class="op-dropdown-arrow"><Icon icon=icon.get() size=dim::ICON_FIELD attr:style="width:var(--op-field-icon-size, var(--op-dim-icon-field));height:var(--op-field-icon-size, var(--op-dim-icon-field));"/></span> }
                                .into_any()
                        }
                    }}
                </button>
                </Field>
                <Popover dismiss_on_scroll=true node_ref=popup_ref open=expanded anchor=anchor placement=Placement::BelowStretch class="op-dropdown-menu" role="listbox">
                    <div class="op-dropdown-list" style=move || popup_style.get()>
                        {header.run()}
                        {before_options.run()}
                        {options.with_value(Vec::is_empty).then(|| empty.run())}
                        <FieldList
                            items=Signal::derive(move || options.get_value())
                            handle=list
                            key=|option: &String| option.clone()
                            let:row
                        >
                            {
                                let option = row.1;
                                let value = option.clone();
                                let is_selected = {
                                    let option = option.clone();
                                    move || selected.with(|s| *s == option)
                                };
                                view! {
                                    <button
                                        type="button"
                                        role="option"
                                        class="op-rippled op-dropdown-item"
                                        aria-selected=move || is_selected().to_string()
                                        on:pointerdown=move |ev| spawn_ripple(&ev)
                                        on:click=move |_| {
                                            if !clickable() { return; }
                                            selected.set(value.clone());
                                            if let Some(cb) = on_select {
                                                cb.run(value.clone());
                                            }
                                            expanded.set(false);
                                        }
                                    >
                                        {match option_slot { Some(slot) => slot.run(option), None => option.into_any() }}
                                    </button>
                                }
                            }
                        </FieldList>
                        {after_options.run()}
                        {footer.run()}
                    </div>
                </Popover>
            </div>
        </div>
    }
}

/// The current box of a field anchor, for a list that opens under it.
pub(crate) fn anchor_signal(anchor: NodeRef<leptos::html::Div>) -> Signal<PopoverAnchor> {
    Signal::derive(move || {
        anchor
            .get()
            .map(|el| {
                el.query_selector(".op-field")
                    .ok()
                    .flatten()
                    .map(|field| PopoverAnchor::of(&field))
                    .unwrap_or_else(|| PopoverAnchor::of(&el))
            })
            .unwrap_or(PopoverAnchor::Point { x: 0.0, y: 0.0 })
    })
}
