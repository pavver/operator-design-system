//! ColorField: a colour code with a ColorPicker popup.

use super::color_math::parse_rgba;
use super::color_picker::{ColorMode, ColorPicker};
use super::parsed::ParsedField;
use super::portal_style::{field_popup_label, field_popup_style};
use crate::components::field::FieldMetrics;
use crate::components::field::FieldVariant;
use crate::components::popover::{Placement, Popover, PopoverAnchor};
use crate::components::primitives::Icon;
use crate::core::{ControlState, Metric, Slot, root_class, root_style, ui_icon, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::dim;
use operator_ui_tokens::{Icon as IconData, icons};
use std::sync::Arc;

/// Colour input as `#rrggbb` (or `#rrggbbaa` with `alpha`) with a preview square and a
/// [`ColorPicker`] popup: spectrum, RGB, HSL and temperature editors, opacity and swatches.
#[component]
pub fn ColorField(
    value: RwSignal<String>,
    /// Preset colours offered in the popup.
    #[prop(optional)]
    swatches: Vec<String>,
    /// Adds the opacity bar; the value gains an alpha byte when the colour is not opaque.
    #[prop(optional)]
    alpha: bool,
    /// Editor tabs of the popup; empty means every mode.
    #[prop(optional)]
    modes: Vec<ColorMode>,
    /// Warm end of the temperature bar, in kelvin.
    #[prop(default = 2000.0)]
    min_kelvin: f64,
    /// Cool end of the temperature bar, in kelvin.
    #[prop(default = 6500.0)]
    max_kelvin: f64,
    /// The colour temperature while the colour comes from the temperature bar.
    #[prop(optional)]
    temperature: Option<RwSignal<Option<u32>>>,
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
    #[prop(optional, into)] picker_label: MaybeProp<String>,
    #[prop(optional, into)] popup_label: MaybeProp<String>,
    #[prop(optional)] icon: Option<IconData>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] popup_shadow_y: MaybeProp<f32>,
    #[prop(optional, into)] popup_shadow_blur: MaybeProp<f32>,
    #[prop(optional, into)] preview_size: MaybeProp<f32>,
    #[prop(optional, into)] picker_gap: MaybeProp<f32>,
    #[prop(optional, into)] picker_padding_x: MaybeProp<f32>,
    #[prop(optional, into)] popup_gap: MaybeProp<f32>,
    #[prop(optional, into)] popup_padding: MaybeProp<f32>,
    #[prop(optional, into)] swatch_size: MaybeProp<f32>,
    #[prop(optional, into)] picker_width: MaybeProp<f32>,
    #[prop(optional, into)] plane_height: MaybeProp<f32>,
    #[prop(optional, into)] bar_height: MaybeProp<f32>,

    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(optional)] on_commit: Option<Callback<String>>,
) -> impl IntoView {
    let placeholder = ui_text(placeholder, "color-field.placeholder", "");
    let aria_label = ui_text(aria_label, "color-field.label", "Colour");
    let picker_label = ui_text(picker_label, "color-field.pick", "Pick a colour");
    let popup_label = ui_text(popup_label, "color-field.popup", "Colours");
    let icon = ui_icon(icon, "color-field.expand", icons::ARROW_DROP_DOWN);
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
    metrics.push(Metric::du("--op-field-preview-size", preview_size));
    metrics.push(Metric::du("--op-field-picker-gap", picker_gap));
    metrics.push(Metric::du("--op-field-picker-padding-x", picker_padding_x));
    metrics.push(Metric::du("--op-field-popup-gap", popup_gap));
    metrics.push(Metric::du("--op-field-popup-padding", popup_padding));
    metrics.push(Metric::du("--op-field-swatch-size", swatch_size));
    metrics.push(Metric::du("--op-field-color-width", picker_width));
    metrics.push(Metric::du("--op-field-color-plane-height", plane_height));
    metrics.push(Metric::du("--op-field-color-bar-height", bar_height));
    let custom_style = Signal::derive(root_style(metrics, style));
    let format = Callback::new(|v: String| v);
    let parse = Callback::new(move |t: String| parse_rgba(&t).map(|c| c.hex(alpha)));
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
    let trailing: ChildrenFn = Arc::new(move || {
        view! {
            <button
                type="button"
                class="op-rippled op-field-reveal op-color-trigger"
                aria-label=move || picker_label.get()
                aria-expanded=move || open.get().to_string()
                disabled=blocked
                on:mousedown=|ev| ev.prevent_default()
                on:click=move |_| open.update(|o| *o = !*o)
            >
                <span class="op-color-preview op-color-checker"><span style:background-color=move || value.get()></span></span>
                {move || view! { <Icon icon=icon.get() size=dim::ICON_FIELD attr:style="width:var(--op-field-icon-size, var(--op-dim-icon-field));height:var(--op-field-icon-size, var(--op-dim-icon-field));"/> }}
            </button>
        }
        .into_any()
    });
    let modes = StoredValue::new(modes);
    let swatches = StoredValue::new(swatches);
    let temperature = temperature.unwrap_or_else(|| RwSignal::new(None));
    view! {
        <div class=root_class("op-value-anchor op-color-field", class) style=move || custom_style.get() node_ref=host>
            <ParsedField
                value=value
                format=format
                parse=parse
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
                on_commit=on_commit
                trailing=Some(trailing)
            />
        </div>
        <Popover dismiss_on_scroll=true node_ref=popup_ref open=open anchor=anchor placement=Placement::BelowEnd class="op-value-popup" role="dialog" label=popup_label.get()>
            <div style=move || popup_style.get()>
                <ColorPicker
                    value=value
                    alpha=alpha
                    modes=modes.get_value()
                    min_kelvin=min_kelvin
                    max_kelvin=max_kelvin
                    temperature=temperature
                    swatches=swatches.get_value()
                    enabled=Signal::derive(move || !blocked())
                    on_commit=Callback::new(move |c: String| {
                        if let Some(cb) = on_commit {
                            cb.run(c);
                        }
                    })
                />
            </div>
        </Popover>
    }
}
