//! Base list row: ListItem.

use crate::core::{Press, du, spawn_ripple};
use leptos::prelude::*;
use operator_ui_tokens::dim;

pub(crate) const CSS: &str = include_str!("list_item.css");

/// Base list row with optional leading and trailing slots and a bottom divider.
#[component]
pub fn ListItem(
    #[prop(into)] title: Signal<String>,
    #[prop(optional, into)] subtitle: Option<Signal<String>>,
    #[prop(optional, into)] leading: Option<ViewFn>,
    #[prop(optional, into)] trailing: Option<ViewFn>,
    #[prop(optional, into)] selected: Signal<bool>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(default = 1)] title_max_lines: u32,
    #[prop(default = 2)] subtitle_max_lines: u32,
    #[prop(optional, into)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let press = Press::new();
    let clickable = on_click.is_some();
    let min_height = if leading.is_some() {
        dim::ROW_AVATAR
    } else if subtitle.is_some() {
        dim::ROW_TWO_LINE
    } else {
        dim::ROW_STANDARD
    };
    let has_subtitle = subtitle.is_some();
    let centered = subtitle.is_none() && leading.is_none();
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

    view! {
        <div class="op-list-item">
            <div
                class="op-list-row"
                class:op-rippled=clickable
                class:op-list-centered=centered
                role=clickable.then_some("button")
                tabindex=move || active().then_some("0")
                style:min-height=du(min_height)
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
                {leading.map(|l| view! { <div class="op-list-leading">{l.run()}</div> })}
                <div class="op-list-text">
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
                </div>
                {trailing
                    .map(|t| {
                        view! {
                            <div class="op-list-trailing" class:op-list-trailing-top=has_subtitle>
                                {t.run()}
                            </div>
                        }
                    })}
            </div>
            <div class="op-divider"></div>
        </div>
    }
}
