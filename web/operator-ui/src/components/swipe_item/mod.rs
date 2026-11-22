//! SwipeItem component: list item revealing left or right actions on swipe.

use crate::core::{Metric, Slot, root_class, root_style, ui_text};
use leptos::ev;
use leptos::prelude::*;

pub(crate) const CSS: &str = include_str!("swipe_item.css");

/// Interactive list item that can be swiped horizontally to reveal contextual actions.
#[component]
pub fn SwipeItem(
    #[prop(optional, into)] left: Slot,
    #[prop(optional, into)] right: Slot,
    #[prop(optional)] on_left: Option<Callback<()>>,
    #[prop(optional)] on_right: Option<Callback<()>>,
    /// Swipe threshold distance in DU to trigger an action (`--op-swipe-item-threshold`).
    #[prop(default = 15.0.into(), into)]
    threshold: MaybeProp<f32>,
    /// Minimum action slot width in DU (`--op-swipe-item-action-width`).
    #[prop(optional, into)]
    action_width: MaybeProp<f32>,
    /// Minimum item height in DU (`--op-swipe-item-height`).
    #[prop(optional, into)]
    height: MaybeProp<f32>,
    #[prop(optional, into)] left_label: MaybeProp<String>,
    #[prop(optional, into)] right_label: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let _left_str = ui_text(left_label, "swipe-item.left", "Left action");
    let _right_str = ui_text(right_label, "swipe-item.right", "Right action");

    let offset_x = RwSignal::new(0.0f64);
    let start_x = RwSignal::new(0.0f64);
    let is_dragging = RwSignal::new(false);

    let threshold_px = Signal::derive(move || (threshold.get().unwrap_or(15.0) * 4.0) as f64);
    let has_left = left.is_set() || on_left.is_some();
    let has_right = right.is_set() || on_right.is_some();

    let on_pointerdown = move |ev: ev::PointerEvent| {
        start_x.set(ev.client_x() as f64);
        is_dragging.set(true);
    };

    let on_pointermove = move |ev: ev::PointerEvent| {
        if !is_dragging.get() {
            return;
        }
        let diff = ev.client_x() as f64 - start_x.get();
        if (diff > 0.0 && !has_left) || (diff < 0.0 && !has_right) {
            offset_x.set(0.0);
        } else {
            offset_x.set(diff);
        }
    };

    let finish_drag = move || {
        if !is_dragging.get() {
            return;
        }
        is_dragging.set(false);
        let curr_off = offset_x.get();
        let th = threshold_px.get();

        if curr_off >= th
            && let Some(cb) = on_left
        {
            cb.run(());
        } else if curr_off <= -th
            && let Some(cb) = on_right
        {
            cb.run(());
        }
        offset_x.set(0.0);
    };

    let metrics = vec![
        Metric::du("--op-swipe-item-action-width", action_width),
        Metric::du("--op-swipe-item-height", height),
    ];
    let class_fn = root_class("op-swipe-item", class);
    let style_fn = root_style(metrics, style);

    view! {
        <div
            class=class_fn
            style=style_fn
            data-dragging=move || is_dragging.get().then_some("")
            on:pointerdown=on_pointerdown
            on:pointermove=on_pointermove
            on:pointerup=move |_| finish_drag()
            on:pointercancel=move |_| finish_drag()
        >
            <div class="op-swipe-item-actions">
                {has_left.then(|| view! {
                    <div class="op-swipe-item-action" data-side="left">
                        {left.run()}
                    </div>
                })}
                {has_right.then(|| view! {
                    <div class="op-swipe-item-action" data-side="right">
                        {right.run()}
                    </div>
                })}
            </div>

            <div
                class="op-swipe-item-content"
                style=move || format!("transform: translateX({}px);", offset_x.get() as i32)
            >
                {children()}
            </div>
        </div>
    }
}
