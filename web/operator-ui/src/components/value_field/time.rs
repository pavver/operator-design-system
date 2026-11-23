//! TimeField: a time of day with a clock dial popup.

use super::clock::ClockPicker;
use super::parse::{format_time, parse_time};
use super::parsed::ParsedField;
use super::portal_style::{field_popup_label, field_popup_style};
use crate::components::field::{FieldMetrics, FieldVariant};
use crate::components::popover::{Placement, Popover, PopoverAnchor};
use crate::components::primitives::Icon;
use crate::core::{ControlState, Metric, Slot, root_class, root_style, ui_icon, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, dim, icons};
use std::sync::Arc;

/// Time-of-day input (`07:30`, `0730`, `7:30:15`) with a [`ClockPicker`] popup; the value is
/// seconds since midnight.
#[component]
pub fn TimeField(
    value: RwSignal<u32>,
    /// Always shows seconds.
    #[prop(optional)]
    seconds: bool,
    /// Arrow Up/Down step in seconds; wraps around midnight.
    #[prop(default = 60)]
    step_s: u32,
    /// The dial shows 12 hours with AM and PM.
    #[prop(optional)]
    hour12: bool,
    /// Minutes snap to this step on the dial.
    #[prop(default = 1)]
    minute_step: u32,
    /// Hides the dial button; the field takes text only.
    #[prop(optional)]
    no_picker: bool,
    #[prop(optional, into)] picker_label: MaybeProp<String>,
    #[prop(optional, into)] popup_label: MaybeProp<String>,
    #[prop(optional)] icon: Option<IconData>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] popup_shadow_y: MaybeProp<f32>,
    #[prop(optional, into)] popup_shadow_blur: MaybeProp<f32>,
    #[prop(optional, into)] clock_size: MaybeProp<f32>,
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
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,

    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(optional)] on_commit: Option<Callback<u32>>,
) -> impl IntoView {
    let placeholder = ui_text(placeholder, "time-field.placeholder", "");
    let aria_label = ui_text(aria_label, "time-field.label", "Time");
    let picker_label = ui_text(picker_label, "time-field.pick", "Pick a time");
    let popup_label = ui_text(popup_label, "time-field.popup", "Clock");
    let icon = ui_icon(icon, "time-field.expand", icons::ARROW_DROP_DOWN);
    let blocked = move || {
        !enabled.get()
            || matches!(
                state.get(),
                Some(ControlState::Disabled | ControlState::Pending)
            )
    };
    let mut metrics = FieldMetrics {
        height,
        padding_x,
        slot_size,
        gap,
        font_size,
    }
    .vars();
    metrics.push(Metric::du("--op-field-icon-size", icon_size));
    metrics.push(Metric::du("--op-field-popup-shadow-y", popup_shadow_y));
    metrics.push(Metric::du(
        "--op-field-popup-shadow-blur",
        popup_shadow_blur,
    ));
    metrics.push(Metric::du("--op-field-clock-size", clock_size));
    let custom_style = Signal::derive(root_style(metrics, style));
    let open = RwSignal::new(false);
    let popup_ref = NodeRef::<leptos::html::Div>::new();
    field_popup_label(popup_ref, popup_label);
    Effect::new(move |_| {
        if blocked() {
            open.set(false);
        }
    });
    let host = NodeRef::<leptos::html::Div>::new();
    let popup_style = field_popup_style(host, open, custom_style);
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
    let trailing: Option<ChildrenFn> = (!no_picker).then(|| {
        let f: ChildrenFn = Arc::new(move || {
            view! {
                <button
                    type="button"
                    class="op-rippled op-field-reveal"
                    aria-label=move || picker_label.get()
                    aria-expanded=move || open.get().to_string()
                    disabled=blocked
                    on:mousedown=|ev| ev.prevent_default()
                    on:click=move |_| open.update(|o| *o = !*o)
                >
                    {move || view! { <Icon icon=icon.get() size=dim::ICON_FIELD attr:style="width:var(--op-field-icon-size, var(--op-dim-icon-field));height:var(--op-field-icon-size, var(--op-dim-icon-field));"/> }}
                </button>
            }
            .into_any()
        });
        f
    });
    let commit = Callback::new(move |v: u32| {
        if let Some(cb) = on_commit {
            cb.run(v);
        }
    });
    let format = Callback::new(move |v: u32| format_time(v, seconds));
    let parse = Callback::new(move |t: String| parse_time(&t));
    let on_step = Callback::new(move |(v, delta): (u32, i32)| {
        let day = 86_400_i64;
        let next = i64::from(v) + i64::from(step_s) * i64::from(delta);
        u32::try_from(next.rem_euclid(day)).unwrap_or(0)
    });
    view! {
        <div class=root_class("op-value-anchor op-time-field", class) style=move || custom_style.get() node_ref=host>
        <ParsedField
            value=value
            format=format
            parse=parse
            on_step=Some(on_step)
            placeholder=placeholder
            label=label
            hint=hint
            error=error
            variant=variant
            aria_label=aria_label
            prepend=prepend
            append=append
            before=before
            after=after
            label_slot=label_slot
            hint_slot=hint_slot
            enabled=enabled
            state=state
            input_mode="numeric"
            on_commit=on_commit
            trailing=trailing
        />
        </div>
        <Popover dismiss_on_scroll=true node_ref=popup_ref open=open anchor=anchor placement=Placement::BelowStart class="op-value-popup" role="dialog" label=popup_label.get()>
            <div style=move || popup_style.get()>
                <ClockPicker
                    value=value
                    hour12=hour12
                    minute_step=minute_step
                    enabled=Signal::derive(move || !blocked())
                    on_commit=commit
                    on_done=Callback::new(move |()| open.set(false))
                />
            </div>
        </Popover>
    }
}
