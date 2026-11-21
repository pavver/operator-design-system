//! Controls overlay and chrome for VideoStream.

use crate::components::icon_button::IconButton;
use crate::core::Slot;
use leptos::prelude::*;
use operator_ui_tokens::icons;

/// Renders the floating controls chrome for the video stream.
#[component]
pub fn VideoStreamChrome(
    title: Signal<String>,
    live: bool,
    live_label: Signal<String>,
    is_muted: RwSignal<bool>,
    mute_label: Signal<String>,
    fullscreen_label: Signal<String>,
    on_toggle_fullscreen: Callback<()>,
    header_slot: StoredValue<Slot>,
    controls_slot: StoredValue<Slot>,
) -> impl IntoView {
    view! {
        <div class="op-video-stream-chrome">
            <div class="op-video-stream-top-bar">
                <div class="op-video-stream-title">
                    {move || title.get()}
                </div>
                <div style="display: flex; align-items: center; gap: calc(var(--op-du) * 1.5);">
                    {live.then(|| view! {
                        <div class="op-video-stream-live-badge">
                            <span class="op-video-stream-live-dot" aria-hidden="true"></span>
                            <span>{move || live_label.get()}</span>
                        </div>
                    })}
                    {header_slot.with_value(|s| s.is_set().then(|| s.run()))}
                </div>
            </div>

            <div class="op-video-stream-bottom-bar">
                <div class="op-video-stream-actions">
                    <IconButton
                        icon=icons::VISIBILITY
                        label=mute_label
                        on_click=Callback::new(move |()| is_muted.update(|m| *m = !*m))
                    />
                    {controls_slot.with_value(|s| s.is_set().then(|| s.run()))}
                </div>
                <div class="op-video-stream-actions">
                    <IconButton
                        icon=icons::OPEN_IN_NEW
                        label=fullscreen_label
                        on_click=on_toggle_fullscreen
                    />
                </div>
            </div>
        </div>
    }
}
