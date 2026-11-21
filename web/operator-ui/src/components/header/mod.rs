//! Top app bar: Header and HeaderAvatar.

use crate::components::avatar::{Avatar, AvatarKind};
use crate::components::primitives::Icon;
use crate::components::status_badge::PresenceStatus;
use crate::core::{Metric, Slot, root_class, root_style, spawn_ripple, ui_icon, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, dim, icons};

pub(crate) const CSS: &str = include_str!("header.css");

/// Avatar shown in a [`Header`].
#[derive(Clone, Debug)]
pub struct HeaderAvatar {
    pub id: String,
    pub name: String,
    pub group: bool,
    pub status: Option<PresenceStatus>,
}

/// Top app bar with back button, optional avatar, title, subtitle and actions.
#[component]
pub fn Header(
    #[prop(into)] title: Signal<String>,
    #[prop(optional, into)] subtitle: Option<Signal<String>>,
    #[prop(optional, into)] on_back: Option<Callback<()>>,
    #[prop(optional)] avatar: Option<HeaderAvatar>,
    #[prop(optional, into)] on_title_click: Option<Callback<()>>,
    #[prop(optional, into)] actions: Slot,
    /// Height in DU (`--op-header-height`).
    #[prop(optional, into)]
    height: MaybeProp<f32>,
    /// Back button width in DU (`--op-header-back-width`).
    #[prop(optional, into)]
    back_width: MaybeProp<f32>,
    /// Gap after back button in DU (`--op-header-gap-back`).
    #[prop(optional, into)]
    gap_back: MaybeProp<f32>,
    /// Left padding when there is no back button in DU (`--op-header-padding-start`).
    #[prop(optional, into)]
    padding_start: MaybeProp<f32>,
    /// Gap after avatar in DU (`--op-header-gap-avatar`).
    #[prop(optional, into)]
    gap_avatar: MaybeProp<f32>,
    /// Title font size in TU (`--op-header-title-size`).
    #[prop(optional, into)]
    title_size: MaybeProp<f32>,
    /// Title line height in TU (`--op-header-title-line-height`).
    #[prop(optional, into)]
    title_line_height: MaybeProp<f32>,
    /// Title font size when subtitle is present in TU (`--op-header-title-small-size`).
    #[prop(optional, into)]
    title_small_size: MaybeProp<f32>,
    /// Title line height when subtitle is present in TU (`--op-header-title-small-line-height`).
    #[prop(optional, into)]
    title_small_line_height: MaybeProp<f32>,
    /// Subtitle font size in TU (`--op-header-subtitle-size`).
    #[prop(optional, into)]
    subtitle_size: MaybeProp<f32>,
    /// Subtitle line height in TU (`--op-header-subtitle-line-height`).
    #[prop(optional, into)]
    subtitle_line_height: MaybeProp<f32>,
    /// Gap between action items in DU (`--op-header-actions-gap`).
    #[prop(optional, into)]
    actions_gap: MaybeProp<f32>,
    /// Right padding for actions in DU (`--op-header-actions-padding-end`).
    #[prop(optional, into)]
    actions_padding_end: MaybeProp<f32>,
    /// Accessible label of the back button; defaults to the `header.back` string.
    #[prop(optional, into)]
    back_label: MaybeProp<String>,
    /// Back button icon; defaults to the `header.back` icon.
    #[prop(optional)]
    back_icon: Option<IconData>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let has_subtitle = subtitle.is_some();
    let back_label = ui_text(back_label, "header.back", "Back");
    let back_icon = ui_icon(back_icon, "header.back", icons::ARROW_BACK);
    let root_metrics = vec![
        Metric::du("--op-header-height", height),
        Metric::du("--op-header-back-width", back_width),
        Metric::du("--op-header-gap-back", gap_back),
        Metric::du("--op-header-padding-start", padding_start),
        Metric::du("--op-header-gap-avatar", gap_avatar),
        Metric::tu("--op-header-title-size", title_size),
        Metric::tu("--op-header-title-line-height", title_line_height),
        Metric::tu("--op-header-title-small-size", title_small_size),
        Metric::tu(
            "--op-header-title-small-line-height",
            title_small_line_height,
        ),
        Metric::tu("--op-header-subtitle-size", subtitle_size),
        Metric::tu("--op-header-subtitle-line-height", subtitle_line_height),
        Metric::du("--op-header-actions-gap", actions_gap),
        Metric::du("--op-header-actions-padding-end", actions_padding_end),
    ];

    view! {
        <header class=root_class("op-header", class) style=root_style(root_metrics, style)>
            {match on_back {
                Some(cb) => view! {
                    <button
                        type="button"
                        class="op-rippled op-header-back"
                        aria-label=move || back_label.get()
                        on:pointerdown=move |ev| spawn_ripple(&ev)
                        on:click=move |_| cb.run(())
                    >
                        {move || view! { <Icon icon=back_icon.get() size=dim::ICON_MEDIUM/> }}
                    </button>
                    <span class="op-header-gap-back"></span>
                }
                .into_any(),
                None => view! { <span class="op-header-start"></span> }.into_any(),
            }}
            {avatar
                .map(|a| {
                    view! {
                        <Avatar
                            id=a.id
                            name=a.name
                            kind=if a.group { AvatarKind::SimpleGroup } else { AvatarKind::Direct }
                            size=9.0
                            status=a.status.map(Signal::from).unwrap_or_else(|| Signal::stored(PresenceStatus::Offline))
                        />
                        <span class="op-header-gap-avatar"></span>
                    }
                })}
            <div
                class="op-header-titles"
                class:op-header-titles-click=on_title_click.is_some()
                on:click=move |_| {
                    if let Some(cb) = on_title_click {
                        cb.run(());
                    }
                }
            >
                <div class="op-header-title" class:op-header-title-small=has_subtitle>
                    {move || title.get()}
                </div>
                {subtitle.map(|s| view! { <div class="op-header-subtitle">{move || s.get()}</div> })}
            </div>
            {if actions.is_set() {
                view! { <div class="op-header-actions">{actions.run()}</div> }.into_any()
            } else {
                view! { <span class="op-header-start"></span> }.into_any()
            }}
        </header>
    }
}
