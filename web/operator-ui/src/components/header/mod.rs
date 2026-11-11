//! Top app bar: Header and HeaderAvatar.

use crate::components::avatar::{Avatar, AvatarKind};
use crate::components::primitives::Icon;
use crate::components::status_badge::PresenceStatus;
use crate::core::{du, spawn_ripple};
use leptos::prelude::*;
use operator_ui_tokens::{dim, icons};

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
    #[prop(optional, into)] actions: Option<ViewFn>,
    /// Height in DU.
    #[prop(default = 13.5)]
    height: f32,
) -> impl IntoView {
    let has_subtitle = subtitle.is_some();
    view! {
        <header class="op-header" style:height=du(height)>
            {match on_back {
                Some(cb) => view! {
                    <button
                        type="button"
                        class="op-rippled op-header-back"
                        aria-label="Back"
                        on:pointerdown=move |ev| spawn_ripple(&ev)
                        on:click=move |_| cb.run(())
                    >
                        <Icon icon=icons::ARROW_BACK size=dim::ICON_MEDIUM/>
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
            {match actions {
                Some(a) => view! { <div class="op-header-actions">{a.run()}</div> }.into_any(),
                None => view! { <span class="op-header-start"></span> }.into_any(),
            }}
        </header>
    }
}
