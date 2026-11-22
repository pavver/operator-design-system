//! Checkbox group: GroupItem and CheckboxGroup.

use crate::core::{
    AsyncAction, ControlState, JobToken, Metric, PendingRun, Press, ScopedSlot, Slot, root_class,
    root_style, sleep_ms, spawn_ripple, ui_icon, ui_text, use_pending_indicator, use_state_visual,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::{Icon as IconData, dim, icons, motion};
use std::collections::HashMap;

use crate::core::glyph::{pending_spinner, symbol};

const MARK_SIZE: &str = "var(--op-checkbox-group-mark-size, var(--op-dim-icon-small))";
const ITEM_ICON_SIZE: &str = "var(--op-checkbox-group-item-icon-size, var(--op-checkbox-group-default-icon-size, var(--op-dim-icon-small)))";

pub(crate) const CSS: &str = include_str!("checkbox_group.css");

/// One option of a [`CheckboxGroup`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GroupItem {
    pub id: String,
    pub name: String,
    /// Optional leading icon, shown by [`CheckboxGroup`].
    pub icon: Option<IconData>,
}

impl GroupItem {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            icon: None,
        }
    }

    /// Adds a leading icon.
    #[must_use]
    pub fn with_icon(mut self, icon: IconData) -> Self {
        self.icon = Some(icon);
        self
    }
}

/// Size of the leading item icons in a [`CheckboxGroup`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GroupIconSize {
    /// `icon_small` (`4 DU`).
    #[default]
    Small,
    /// `icon_large` (`6 DU`).
    Medium,
    /// Tile height minus `pad_tight` on each side (`10 DU`).
    Large,
}

impl GroupIconSize {
    /// Icon size in DU.
    pub fn du(self) -> f32 {
        match self {
            Self::Small => dim::ICON_SMALL,
            Self::Medium => dim::ICON_LARGE,
            Self::Large => dim::ROW_CONTROL - 2.0 * dim::PAD_TIGHT,
        }
    }

    const fn as_str(self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

/// Vertical group of full-width checkbox tiles; each tile runs its own async request.
#[component]
pub fn CheckboxGroup(
    items: Vec<GroupItem>,
    /// Size of the leading item icons.
    #[prop(optional)]
    icon_size: GroupIconSize,
    #[prop(into)] selected: Signal<Vec<String>>,
    /// Called with `(id, target_checked)`; the parent updates `selected` on success.
    #[prop(into)]
    on_change: AsyncAction<(String, bool)>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] height: MaybeProp<f32>,
    #[prop(optional, into)] padding_x: MaybeProp<f32>,
    #[prop(optional, into)] item_gap: MaybeProp<f32>,
    #[prop(optional, into)] large_icon_inset: MaybeProp<f32>,
    #[prop(optional, into)] box_size: MaybeProp<f32>,
    #[prop(optional, into)] mark_size: MaybeProp<f32>,
    #[prop(optional, into)] item_icon_size: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] spinner_stroke: MaybeProp<f32>,
    #[prop(optional, into)] width: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional)] option: Option<ScopedSlot<GroupItem>>,
    #[prop(optional, into)] before_options: Slot,
    #[prop(optional, into)] after_options: Slot,
    #[prop(optional, into)] loading_label: MaybeProp<String>,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional)] checked_icon: Option<IconData>,
) -> impl IntoView {
    let loading_label = ui_text(loading_label, "checkbox-group.loading", "Loading");
    let label = ui_text(label, "checkbox-group.label", "Options");
    let checked_icon = ui_icon(checked_icon, "checkbox-group.checked", icons::CHECK);

    let root_metrics = vec![
        Metric::du("--op-checkbox-group-width", width),
        Metric::du("--op-checkbox-group-spinner-stroke", spinner_stroke),
        Metric::du("--op-checkbox-group-gap", gap),
        Metric::du("--op-checkbox-group-height", height),
        Metric::du("--op-checkbox-group-padding-x", padding_x),
        Metric::du("--op-checkbox-group-item-gap", item_gap),
        Metric::du("--op-checkbox-group-large-icon-inset", large_icon_inset),
        Metric::du("--op-checkbox-group-box-size", box_size),
        Metric::du("--op-checkbox-group-mark-size", mark_size),
        Metric::du("--op-checkbox-group-item-icon-size", item_icon_size),
        Metric::tu("--op-checkbox-group-font-size", font_size),
        Metric::tu("--op-checkbox-group-line-height", line_height),
    ];

    let states = RwSignal::new(HashMap::<String, ControlState>::new());

    let tiles = items
        .into_iter()
        .map(|item| {
            let id = item.id.clone();
            let press = Press::new();
            let job = JobToken::new();
            let action = on_change.clone();
            let is_selected = {
                let id = id.clone();
                Signal::derive(move || selected.with(|s| s.contains(&id)))
            };
            let item_state = {
                let id = id.clone();
                Signal::derive(move || states.with(|m| m.get(&id).copied().unwrap_or_default()))
            };
            let border_state = Signal::derive(move || {
                let s = item_state.get();
                if s != ControlState::Idle {
                    s
                } else if press.pressed.get() {
                    ControlState::Pressed
                } else if is_selected.get() {
                    ControlState::Selected
                } else {
                    ControlState::Idle
                }
            });
            let visual = use_state_visual(border_state);
            let tone = use_state_visual(item_state);
            let show_spinner =
                use_pending_indicator(move || item_state.get() == ControlState::Pending);
            let clickable = move || item_state.get() != ControlState::Pending;
            let click = move || {
                let request = job.next();
                let id = id.clone();
                let action = action.clone();
                states.update(|m| {
                    m.insert(id.clone(), ControlState::Pending);
                });
                spawn_local(async move {
                    let target = !is_selected.get_untracked();
                    let result = action.call((id.clone(), target)).await;
                    if !job.is_current(request) { return; }
                    states.update(|m| {
                        m.insert(id.clone(), result.into());
                    });
                    sleep_ms(motion::RESULT_HOLD_MS).await;
                    if !job.is_current(request) { return; }
                    states.update(|m| {
                        m.insert(id, ControlState::Idle);
                    });
                });
            };
            view! {
                <button
                    type="button"
                    role="checkbox"
                    class="op-sb op-rippled op-group-tile"
                    aria-checked=move || is_selected.get().to_string()
                    aria-busy=move || (item_state.get() == ControlState::Pending).then_some("true")
                    data-checked=move || is_selected.get().then_some("")
                    data-state=move || visual.state.get()
                    data-result=move || visual.result.get()
                    data-fading=move || visual.fading.get().then_some("")
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
                            click();
                        }
                    }
                >
                    {item.icon.map(|i| view! { <span class="op-group-icon">{symbol(Signal::stored(i), ITEM_ICON_SIZE, None)}</span> })}
                    <span class="op-group-name">{option.map(|slot| slot.run(item.clone())).unwrap_or_else(|| item.name.clone().into_any())}</span>
                    <span class="op-group-box" data-checked=move || is_selected.get().then_some("")>
                        <span
                            class="op-tone op-group-mark"
                            data-state=move || tone.state.get()
                            data-result=move || tone.result.get()
                            data-fading=move || tone.fading.get().then_some("")
                        >
                            {move || {
                                if show_spinner.get() {
                                    pending_spinner("checkbox-group", MARK_SIZE, loading_label)
                                } else if is_selected.get() {
                                    symbol(checked_icon, MARK_SIZE, None)
                                } else {
                                    ().into_any()
                                }
                            }}
                        </span>
                    </span>
                    <PendingRun/>
                </button>
            }
        })
        .collect_view();

    view! { <div class=root_class("op-group", class)
    style=root_style(root_metrics, style) role="group" aria-label=move || label.get() data-icon-size=icon_size.as_str()>{before_options.run()}{tiles}{after_options.run()}</div> }
}
