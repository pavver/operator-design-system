//! InfiniteScroll component: loads more items when a sentinel enters the viewport.

use crate::components::primitives::Icon;
use crate::core::observers::{IntersectionOptions, use_intersection};
use crate::core::{Metric, Slot, root_class, root_style, ui_text};
use leptos::html::Div;
use leptos::prelude::*;
use operator_ui_tokens::icons;

pub(crate) const CSS: &str = include_str!("infinite_scroll.css");

/// Incrementally appends items to a list as the user scrolls towards the bottom.
#[component]
pub fn InfiniteScroll(
    on_load: Callback<()>,
    #[prop(default = true.into(), into)] has_more: Signal<bool>,
    #[prop(default = false.into(), into)] loading: Signal<bool>,
    #[prop(optional, into)] error: Signal<Option<String>>,
    #[prop(optional)] on_retry: Option<Callback<()>>,
    /// Lookahead offset margin in DU before the sentinel hits the viewport (`--op-infinite-scroll-offset`).
    #[prop(optional, into)]
    offset: MaybeProp<f32>,
    /// Height of the loading or error indicator row in DU (`--op-infinite-scroll-pending-height`).
    #[prop(optional, into)]
    pending_height: MaybeProp<f32>,
    #[prop(optional, into)] loading_label: MaybeProp<String>,
    #[prop(optional, into)] error_label: MaybeProp<String>,
    #[prop(optional, into)] retry_label: MaybeProp<String>,
    #[prop(optional, into)] loading_slot: Slot,
    #[prop(optional, into)] error_slot: Slot,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let loading_str = ui_text(loading_label, "infinite-scroll.loading", "Loading more…");
    let error_str = ui_text(error_label, "infinite-scroll.error", "Failed to load");
    let retry_str = ui_text(retry_label, "infinite-scroll.retry", "Retry");

    let sentinel_ref = NodeRef::<Div>::new();
    let root_margin = offset
        .get()
        .map(|du_val| format!("0px 0px {}px 0px", (du_val * 4.0) as i32));

    let intersection = use_intersection(
        sentinel_ref,
        IntersectionOptions {
            once: false,
            root_margin,
            threshold: Some(0.0),
        },
    );

    Effect::new(move |_| {
        let is_visible = intersection.get().is_intersecting;
        if is_visible && has_more.get() && !loading.get() && error.get().is_none() {
            on_load.run(());
        }
    });

    let metrics = vec![
        Metric::du("--op-infinite-scroll-offset", offset),
        Metric::du("--op-infinite-scroll-pending-height", pending_height),
    ];
    let class_fn = root_class("op-infinite-scroll", class);
    let style_fn = root_style(metrics, style);

    let has_loading_slot = loading_slot.is_set();
    let has_error_slot = error_slot.is_set();

    view! {
        <div class=class_fn style=style_fn>
            {children()}

            {move || {
                if let Some(err_msg) = error.get() {
                    if has_error_slot {
                        view! {
                            <div class="op-infinite-scroll-error" role="alert">
                                {error_slot.run()}
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="op-infinite-scroll-error" role="alert">
                                <Icon icon=icons::ERROR size=18.0 />
                                <span>{if err_msg.is_empty() { error_str.get() } else { err_msg }}</span>
                                {on_retry.map(|retry_cb| view! {
                                    <button
                                        type="button"
                                        class="op-infinite-scroll-retry"
                                        on:click=move |_| retry_cb.run(())
                                    >
                                        <Icon icon=icons::REFRESH size=16.0 />
                                        <span>{retry_str.get()}</span>
                                    </button>
                                })}
                            </div>
                        }.into_any()
                    }
                } else if loading.get() {
                    if has_loading_slot {
                        view! {
                            <div class="op-infinite-scroll-loading" role="status" aria-busy="true">
                                {loading_slot.run()}
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="op-infinite-scroll-loading" role="status" aria-busy="true">
                                <span class="op-infinite-scroll-spinner" />
                                <span>{loading_str.get()}</span>
                            </div>
                        }.into_any()
                    }
                } else if has_more.get() {
                    view! {
                        <div class="op-infinite-scroll-sentinel" node_ref=sentinel_ref />
                    }.into_any()
                } else {
                    view! { <span /> }.into_any()
                }
            }}
        </div>
    }
}
