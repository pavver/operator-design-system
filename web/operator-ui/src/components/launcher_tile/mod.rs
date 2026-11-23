//! Square launcher tile for dashboards and application grids: LauncherTile.

use crate::core::glyph::{pending_spinner, symbol};
use crate::core::{
    AsyncAction, ControlState, JobToken, Metric, PendingRun, Press, Slot, join_action_scope,
    link_state, root_class, root_style, sleep_ms, spawn_ripple, ui_icon, ui_text,
    use_pending_indicator, use_state_visual,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::{Icon as IconData, icons, motion};

/// Icon size inside the tile; the pending spinner and the result icons take it too.
const ICON_SIZE: &str = "var(--op-launcher-tile-icon-size, var(--op-dim-icon-xlarge))";

/// What the icon area of the tile shows.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Glyph {
    Icon,
    Spinner,
    Success,
    Error,
}

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
    /// Accessible name of the pending spinner.
    #[prop(optional, into)]
    loading_label: MaybeProp<String>,
    /// Accessible name of the success icon.
    #[prop(optional, into)]
    success_label: MaybeProp<String>,
    #[prop(optional)] success_icon: Option<IconData>,
    /// Accessible name of the error icon.
    #[prop(optional, into)]
    error_label: MaybeProp<String>,
    #[prop(optional)] error_icon: Option<IconData>,
    /// Shows the tile as pending from outside.
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
    let loading_label = ui_text(loading_label, "launcher-tile.loading", "Loading");
    let success_label = ui_text(success_label, "launcher-tile.success", "Success");
    let error_label = ui_text(error_label, "launcher-tile.error", "Error");
    let success_icon = ui_icon(success_icon, "launcher-tile.success", icons::CHECK);
    let error_icon = ui_icon(error_icon, "launcher-tile.error", icons::CLOSE);
    // Inside a ButtonGroup the group can switch the tile off and block it while another runs.
    let (enabled, on_async_click) = join_action_scope(enabled, on_async_click);

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
    let glyph = RwSignal::new(Glyph::Icon);
    let has_icon = icon.is_some() || icon_slot.is_set();
    let icon_area = move || {
        let g = glyph.get();
        if g == Glyph::Icon && !has_icon {
            return None;
        }
        let shown = match g {
            Glyph::Spinner => pending_spinner("launcher-tile", ICON_SIZE, loading_label),
            Glyph::Success => symbol(success_icon, ICON_SIZE, Some(success_label)),
            Glyph::Error => symbol(error_icon, ICON_SIZE, Some(error_label)),
            Glyph::Icon => icon_slot
                .run()
                .or_else(|| icon.map(|ic| symbol(Signal::stored(ic), ICON_SIZE, None)))
                .unwrap_or_else(|| ().into_any()),
        };
        Some(view! { <div class="op-launcher-tile-icon">{shown}</div> })
    };
    let inner_view = move || {
        if content_slot.is_set() {
            content_slot.run()
        } else {
            Some(
                view! {
                    <div class="op-launcher-tile-inner">
                        {icon_area.clone()}
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
        let show_flash = RwSignal::new(false);
        let job = JobToken::new();

        let current = Signal::derive(move || {
            if !enabled.get() {
                return ControlState::Disabled;
            }
            if loading.get() {
                return ControlState::Pending;
            }
            if let Some(s) = state.get() {
                return s;
            }
            let managed = local_state.get();
            if managed != ControlState::Idle {
                return managed;
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
        let show_spinner = use_pending_indicator(move || current.get() == ControlState::Pending);
        // The spinner and the result flash replace the icon, as in the other buttons.
        Effect::new(move |_| {
            let s = current.get();
            glyph.set(if show_spinner.get() {
                Glyph::Spinner
            } else if show_flash.get() && s == ControlState::Success {
                Glyph::Success
            } else if show_flash.get() && s == ControlState::Error {
                Glyph::Error
            } else {
                Glyph::Icon
            });
        });
        let has_action = on_click.is_some() || on_async_click.is_some();
        let clickable =
            move || has_action && enabled.get() && current.get() != ControlState::Pending;

        let run = move || {
            if let Some(action) = on_async_click.clone() {
                let id = job.next();
                spawn_local(async move {
                    local_state.set(ControlState::Pending);
                    let result = action.call(()).await;
                    if !job.is_current(id) {
                        return;
                    }
                    local_state.set(result.into());
                    show_flash.set(true);
                    sleep_ms(motion::LOCAL_FLASH_ICON_MS).await;
                    if !job.is_current(id) {
                        return;
                    }
                    show_flash.set(false);
                    sleep_ms(motion::LOCAL_FLASH_TAIL_MS).await;
                    if job.is_current(id) {
                        local_state.set(ControlState::Idle);
                    }
                });
            } else if let Some(cb) = on_click {
                cb.run(());
            }
        };

        view! {
            <button
                type="button"
                class=root_class("op-sb op-sb-bg op-rippled op-launcher-tile", class)
                style=full_style
                data-state=move || visual.state.get()
                data-result=move || visual.result.get()
                data-fading=move || visual.fading.get().then_some("")
                aria-label=move || resolved_label.get()
                aria-busy=move || (current.get() == ControlState::Pending).then_some("true")
                aria-disabled=move || (!clickable()).then_some("true")
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
                        run();
                    }
                }
            >
                {inner_view}
                {badge_slot.is_set().then(|| view! {
                    <div class="op-launcher-tile-badge">{badge_slot.run()}</div>
                })}
                <PendingRun/>
            </button>
        }
        .into_any()
    }
}
