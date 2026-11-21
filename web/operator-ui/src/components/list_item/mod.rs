//! Base list row: ListItem.

use crate::core::{Metric, Press, Slot, root_class, root_style, spawn_ripple};
use leptos::prelude::*;
use operator_ui_tokens::dim;

pub(crate) const CSS: &str = include_str!("list_item.css");

/// Base list row with optional leading and trailing slots and a bottom divider.
#[component]
pub fn ListItem(
    #[prop(into)] title: Signal<String>,
    #[prop(optional, into)] subtitle: Option<Signal<String>>,
    #[prop(optional, into)] leading: Slot,
    #[prop(optional, into)] trailing: Slot,
    #[prop(optional, into)] content: Slot,
    #[prop(optional, into)] selected: Signal<bool>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(default = 1)] title_max_lines: u32,
    #[prop(default = 2)] subtitle_max_lines: u32,
    #[prop(optional, into)] on_click: Option<Callback<()>>,
    #[prop(optional, into)] min_height: MaybeProp<f32>,
    #[prop(optional, into)] padding_x: MaybeProp<f32>,
    #[prop(optional, into)] padding_y: MaybeProp<f32>,
    #[prop(optional, into)] leading_padding: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] text_padding: MaybeProp<f32>,
    #[prop(optional, into)] subtitle_gap: MaybeProp<f32>,
    #[prop(optional, into)] title_font_size: MaybeProp<f32>,
    #[prop(optional, into)] title_line_height: MaybeProp<f32>,
    #[prop(optional, into)] subtitle_font_size: MaybeProp<f32>,
    #[prop(optional, into)] subtitle_line_height: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let press = Press::new();
    let clickable = on_click.is_some();
    let default_min_height = if leading.is_set() {
        dim::ROW_AVATAR
    } else if subtitle.is_some() {
        dim::ROW_TWO_LINE
    } else {
        dim::ROW_STANDARD
    };
    let has_subtitle = subtitle.is_some();
    let centered = subtitle.is_none() && !leading.is_set();
    let bg = move || {
        if !enabled.get() {
            "disabled"
        } else if press.pressed.get() {
            "pressed"
        } else if selected.get() {
            "selected"
        } else {
            "idle"
        }
    };
    let active = move || clickable && enabled.get();

    let metrics = vec![
        Metric::du("--op-list-item-min-height", min_height),
        Metric::du("--op-list-item-padding-x", padding_x),
        Metric::du("--op-list-item-padding-y", padding_y),
        Metric::du("--op-list-item-leading-padding", leading_padding),
        Metric::du("--op-list-item-gap", gap),
        Metric::du("--op-list-item-text-padding", text_padding),
        Metric::du("--op-list-item-subtitle-gap", subtitle_gap),
        Metric::tu("--op-list-item-title-font-size", title_font_size),
        Metric::tu("--op-list-item-title-line-height", title_line_height),
        Metric::tu("--op-list-item-subtitle-font-size", subtitle_font_size),
        Metric::tu("--op-list-item-subtitle-line-height", subtitle_line_height),
    ];
    view! {
        <div class=root_class("op-list-item", class) style=root_style(metrics, style)>
            <div
                class="op-list-row"
                class:op-rippled=clickable
                class:op-list-centered=centered
                role=clickable.then_some("button")
                tabindex=move || active().then_some("0")
                style:min-height=format!("var(--op-list-item-min-height, calc(var(--op-du) * {default_min_height}))")
                data-bg=bg
                on:pointerdown=move |ev| {
                    if active() {
                        press.down();
                        spawn_ripple(&ev);
                    }
                }
                on:pointerup=move |_| press.up()
                on:pointerleave=move |_| press.up()
                on:pointercancel=move |_| press.up()
                on:click=move |_| {
                    if active()
                        && let Some(cb) = on_click
                    {
                        cb.run(());
                    }
                }
                on:keydown=move |ev| {
                    if active() && (ev.key() == "Enter" || ev.key() == " ") {
                        ev.prevent_default();
                        if let Some(cb) = on_click {
                            cb.run(());
                        }
                    }
                }
            >
                {leading.is_set().then(|| view! { <div class="op-list-leading">{leading.run()}</div> })}
                <div class="op-list-text">
                    {if content.is_set() { content.run().into_any() } else { view! {
                    <div
                        class="op-list-title"
                        data-disabled=move || (!enabled.get()).then_some("")
                        style=format!("-webkit-line-clamp: {title_max_lines}")
                    >
                        {move || title.get()}
                    </div>
                    {subtitle
                        .map(|s| {
                            view! {
                                <div class="op-list-subtitle" style=format!("-webkit-line-clamp: {subtitle_max_lines}")>
                                    {move || s.get()}
                                </div>
                            }
                        })}
                    }.into_any() }}
                </div>
                {trailing
                    .is_set().then(|| {
                        view! {
                            <div class="op-list-trailing" class:op-list-trailing-top=has_subtitle>
                                {trailing.run()}
                            </div>
                        }
                    })}
            </div>
            <div class="op-divider"></div>
        </div>
    }
}
