//! Square launcher tile for dashboards and application grids: LauncherTile.

use crate::components::primitives::Icon;
use crate::core::{
    AsyncAction, ControlState, JobToken, Metric, Press, Slot, link_state, root_class, root_style,
    sleep_ms, spawn_ripple, ui_text, use_pending_indicator, use_state_visual,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::dim;
use operator_ui_tokens::{Icon as IconData, motion};

pub(crate) const CSS: &str = include_str!("launcher_tile.css");

/// Square launcher tile for dashboards and application grids.
///
/// Follows standard Operator 0 DU square geometry with 24 DU default dimension,
/// state border feedback, and ripple effects. Supports sync and async click actions
/// as well as navigation via `href`.
#[component]
pub fn LauncherTile(
    /// Translatable title text below the icon.
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// Optional primary icon.
    #[prop(optional)]
    icon: Option<IconData>,
    /// Custom icon slot (overrides `icon`).
    #[prop(optional, into)]
    icon_slot: Slot,
    /// Slot for top-right notification counter or status badge.
    #[prop(optional, into)]
    badge_slot: Slot,
    /// Slot for custom content overriding the entire inner column.
    #[prop(optional, into)]
    content_slot: Slot,
    /// Explicit state override.
    #[prop(optional, into)]
    state: Signal<Option<ControlState>>,
    /// Enabled flag; disabled tiles ignore clicks.
    #[prop(default = true.into(), into)]
    enabled: Signal<bool>,
    /// Loading flag for manual pending spinner.
    #[prop(optional, into)]
    loading: Signal<bool>,
    /// Selected flag (for selectable grid tiles).
    #[prop(optional, into)]
    selected: Signal<bool>,
    /// Synchronous click handler.
    #[prop(optional, into)]
    on_click: Option<Callback<()>>,
    /// Asynchronous action returning `ActionResult`.
    #[prop(optional, into)]
    on_async_click: Option<AsyncAction>,
    /// Navigation target; switches the tile into an `<a>` link.
    #[prop(optional, into)]
    href: Option<Signal<String>>,
    /// Opens the link in a new tab (link mode only).
    #[prop(optional)]
    new_tab: bool,
    /// Tile size in DU (`--op-launcher-tile-size`, default 24 DU).
    #[prop(optional, into)]
    size: MaybeProp<f32>,
    /// Icon size in DU (`--op-launcher-tile-icon-size`, default 8 DU).
    #[prop(optional, into)]
    icon_size: MaybeProp<f32>,
    /// Gap between icon and label in DU (`--op-launcher-tile-gap`, default 2.5 DU).
    #[prop(optional, into)]
    gap: MaybeProp<f32>,
    /// Inner padding in DU (`--op-launcher-tile-padding`, default 2 DU).
    #[prop(optional, into)]
    padding: MaybeProp<f32>,
    /// Border width in DU (`--op-launcher-tile-border-width`, default 0.25 DU).
    #[prop(optional, into)]
    border_width: MaybeProp<f32>,
    /// Label font size in TU (`--op-launcher-tile-font-size`, default 11 TU).
    #[prop(optional, into)]
    font_size: MaybeProp<f32>,
    /// Label line height in TU (`--op-launcher-tile-line-height`, default 14 TU).
    #[prop(optional, into)]
    line_height: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let resolved_label = ui_text(label, "launcher-tile.label", "Launch");

    let root_metrics = vec![
        Metric::du("--op-launcher-tile-size", size),
        Metric::du("--op-launcher-tile-icon-size", icon_size),
        Metric::du("--op-launcher-tile-gap", gap),
        Metric::du("--op-launcher-tile-padding", padding),
        Metric::du("--op-launcher-tile-border-width", border_width),
        Metric::tu("--op-launcher-tile-font-size", font_size),
        Metric::tu("--op-launcher-tile-line-height", line_height),
    ];

    let full_style = root_style(root_metrics, style);

    let inner_view = move || {
        if content_slot.is_set() {
            content_slot.run()
        } else {
            let icon_render = if icon_slot.is_set() {
                icon_slot.run()
            } else {
                icon.map(|ic| view! { <Icon icon=ic size=dim::ICON_XLARGE /> }.into_any())
            };
            Some(
                view! {
                    <div class="op-launcher-tile-inner">
                        {icon_render.map(|ic| view! {
                            <div class="op-launcher-tile-icon">{ic}</div>
                        })}
                        <span class="op-launcher-tile-label">{move || resolved_label.get()}</span>
                    </div>
                }
                .into_any(),
            )
        }
    };

    if let Some(href_sig) = href {
        let press = Press::new();
        let current = link_state(enabled, press);
        let link_state_attr = move || {
            if !enabled.get() {
                "disabled"
            } else if selected.get() {
                "selected"
            } else {
                current.get().as_str()
            }
        };

        view! {
            <a
                class=root_class("op-sb op-sb-bg op-rippled op-launcher-tile", class)
                style=full_style
                href=move || enabled.get().then(|| href_sig.get())
                target=new_tab.then_some("_blank")
                rel=new_tab.then_some("noopener noreferrer")
                role="link"
                data-state=link_state_attr
                aria-disabled=move || (!enabled.get()).then_some("true")
                on:pointerdown=move |ev| {
                    if enabled.get() {
                        press.down();
                        spawn_ripple(&ev);
                    }
                }
                on:pointerup=move |_| press.up()
                on:pointerleave=move |_| press.up()
                on:pointercancel=move |_| press.up()
            >
                {inner_view}
                {badge_slot.is_set().then(|| view! {
                    <div class="op-launcher-tile-badge">{badge_slot.run()}</div>
                })}
            </a>
        }
        .into_any()
    } else {
        let press = Press::new();
        let local_state = RwSignal::new(ControlState::Idle);
        let job = JobToken::new();

        let current = Signal::derive(move || {
            if let Some(s) = state.get() {
                return s;
            }
            if !enabled.get() {
                return ControlState::Disabled;
            }
            let managed = local_state.get();
            if managed != ControlState::Idle {
                return managed;
            }
            if loading.get() {
                return ControlState::Pending;
            }
            if selected.get() {
                return ControlState::Selected;
            }
            if press.pressed.get() {
                return ControlState::Pressed;
            }
            ControlState::Idle
        });

        let visual = use_state_visual(current);
        let pending = use_pending_indicator(move || current.get() == ControlState::Pending);

        let is_busy = move || !enabled.get() || current.get() == ControlState::Pending;

        let perform_click = Callback::new(move |ev: Option<web_sys::PointerEvent>| {
            if is_busy() {
                return;
            }
            if let Some(ref ev) = ev {
                spawn_ripple(ev);
            }
            if let Some(cb) = on_click {
                cb.run(());
            } else if let Some(ref action) = on_async_click {
                let action = action.clone();
                let id = job.next();
                spawn_local(async move {
                    local_state.set(ControlState::Pending);
                    let result = action.call(()).await;
                    if !job.is_current(id) {
                        return;
                    }
                    local_state.set(result.into());
                    sleep_ms(motion::RESULT_HOLD_MS).await;
                    if job.is_current(id) {
                        local_state.set(ControlState::Idle);
                    }
                });
            }
        });

        view! {
            <button
                type="button"
                class=root_class("op-sb op-sb-bg op-rippled op-launcher-tile", class)
                style=full_style
                disabled=move || !enabled.get()
                data-state=move || visual.state.get()
                data-result=move || visual.result.get()
                aria-label=move || resolved_label.get()
                on:pointerdown=move |ev| {
                    if !is_busy() {
                        press.down();
                        perform_click.run(Some(ev));
                    }
                }
                on:pointerup=move |_| press.up()
                on:pointerleave=move |_| press.up()
                on:pointercancel=move |_| press.up()
                on:keydown=move |ev| {
                    if (ev.key() == "Enter" || ev.key() == " ") && !is_busy() {
                        ev.prevent_default();
                        perform_click.run(None);
                    }
                }
            >
                {inner_view}
                {badge_slot.is_set().then(|| view! {
                    <div class="op-launcher-tile-badge">{badge_slot.run()}</div>
                })}
                {move || pending.get().then(|| view! {
                    <div class="op-launcher-tile-pending">
                        <Icon icon=operator_ui_tokens::icons::REFRESH size=dim::ICON_LARGE />
                    </div>
                })}
            </button>
        }
        .into_any()
    }
}
