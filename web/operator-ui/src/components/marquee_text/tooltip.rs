use crate::components::popover::{Placement, Popover, PopoverAnchor};
use crate::core::Slot;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

pub trait PointerKind {
    fn pointer_type_is_touch(&self) -> bool;
}

impl PointerKind for web_sys::MouseEvent {
    fn pointer_type_is_touch(&self) -> bool {
        self.dyn_ref::<web_sys::PointerEvent>()
            .is_some_and(|p| p.pointer_type() == "touch")
    }
}

/// The overlay host is outside the subtree, so forward only public tooltip variables.
pub fn forward_tooltip_style(host: NodeRef<leptos::html::Span>, layer: &web_sys::HtmlElement) {
    let Some(host) = host.get_untracked() else {
        return;
    };
    let Some(style) = window().get_computed_style(&host).ok().flatten() else {
        return;
    };
    for name in [
        "max-width",
        "padding-x",
        "padding-y",
        "font-size",
        "line-height",
        "background",
        "border",
        "text",
        "shadow",
        "shadow-y",
        "shadow-blur",
    ] {
        let var = format!("--op-marquee-text-tooltip-{name}");
        if let Ok(value) = style.get_property_value(&var) {
            let _ = layer.style().set_property(&var, &value);
        }
    }
}

#[component]
pub fn MarqueeTooltip(
    tip_layer: NodeRef<leptos::html::Div>,
    tip_open: RwSignal<bool>,
    tip_anchor: RwSignal<PopoverAnchor>,
    tip_gap: Signal<f32>,
    tooltip_slot: StoredValue<Slot>,
    tooltip_text: Signal<String>,
) -> impl IntoView {
    view! {
        <Popover
            node_ref=tip_layer
            open=tip_open
            anchor=tip_anchor
            placement=Placement::Above
            gap=tip_gap
            passive=true
            dismiss_on_outside=false
            restore_focus=false
            class="op-tooltip op-t-caption op-marquee-tooltip"
            role="tooltip"
        >
            <div class="op-marquee-tooltip-content">
                {tooltip_slot.with_value(|slot| if slot.is_set() { slot.run().into_any() } else { (move || tooltip_text.get()).into_any() })}
            </div>
        </Popover>
    }
}
