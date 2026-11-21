//! Landing page and component index.

use crate::i18n::{lang, t, tr};
use crate::kit::SimBar;
use crate::nav::NAV;
use crate::shell::ChangeMarker;
use crate::{REPO_URL, VERSION, docs, kit};
use leptos::prelude::*;
use operator_ui::prelude::*;

fn count_status(status: &str) -> usize {
    docs::DOCS
        .iter()
        .filter(|d| d.slug.starts_with("components/") && d.status == Some(status))
        .count()
}

#[component]
pub fn Home() -> impl IntoView {
    let tiles = NAV
        .iter()
        .filter(|(g, _)| *g != "Getting started" && *g != "Foundations")
        .map(|(group, items)| {
            let links = items
                .iter()
                .filter_map(|s| docs::find(s))
                .map(|d| {
                    view! {
                        <a class="docs-index-link" href=format!("#/{}", d.slug)>
                            <span>{move || d.title(lang())}</span>
                            <ChangeMarker doc=d/>
                        </a>
                    }
                })
                .collect_view();
            view! {
                <div class="docs-index-group">
                    <div class="docs-nav-heading">{t(group)}</div>
                    {links}
                </div>
            }
        })
        .collect_view();
    let b = kit::backend();
    let demo_switch = RwSignal::new(true);
    view! {
        <div class="docs-home">
            <section class="docs-hero">
                <div class="docs-hero-text">
                    <div class="docs-kicker">{format!("v{VERSION} · MIT OR Apache-2.0")}</div>
                    <h1 class="docs-hero-title">"Operator UI"</h1>
                    <p class="docs-hero-lead">
                        {t("A design system for operator consoles: dense, rectangular, monospaced. Every control shows the life of an asynchronous request — pending, success, error — with one visual language on every platform.")}
                    </p>
                    <div class="docs-hero-actions">
                        <a class="docs-cta docs-cta-primary" href="#/platforms/web">{t("Get started")}</a>
                        <a class="docs-cta" href="#/foundations/principles">{t("Read the principles")}</a>
                        <a class="docs-cta" href=REPO_URL>"GitHub"</a>
                    </div>
                    <div class="docs-hero-stats op-t-caption demo-muted">
                        {move || format!("{} {} · {} {} · {}", count_status("reviewed"), tr(lang(), "reviewed components"), count_status("draft"), tr(lang(), "drafts"), tr(lang(), "tokens for web, Rust and Kotlin"))}
                    </div>
                </div>
                <div class="docs-hero-demo">
                    <Card>
                        <div class="demo-stack">
                            <ListItem
                                title=t("Night mode")
                                subtitle=t("Commits after the camera confirms")
                                trailing=ViewFn::from(move || {
                                    view! {
                                        <AsyncSwitch
                                            checked=demo_switch
                                            on_toggle=AsyncAction::new(move |v: bool| async move {
                                                let r = b.call(format!("night_mode={v}")).await;
                                                if r.is_success() {
                                                    demo_switch.set(v);
                                                }
                                                r
                                            })
                                        />
                                    }
                                })
                            />
                            <div class="demo-inline">
                                <ActionButton text=t("Restart stream") icon=icons::REFRESH on_click=b.action("restart stream")/>
                                <ActionButton text=t("Delete") dangerous=true on_click=b.action("delete camera")/>
                            </div>
                            <SimBar/>
                        </div>
                    </Card>
                </div>
            </section>
            <section class="docs-features">
                <a class="docs-feature" href="#/foundations/states">
                    <span class="docs-feature-title">{t("Async states")}</span>
                    <span class="demo-muted">{t("Pending, success and error are part of every control, not an afterthought.")}</span>
                </a>
                <a class="docs-feature" href="#/foundations/units">
                    <span class="docs-feature-title">{t("Design Units")}</span>
                    <span class="demo-muted">{t("Geometry in DU, text in TU. One grid for web, desktop and Android.")}</span>
                </a>
                <a class="docs-feature" href="#/tokens">
                    <span class="docs-feature-title">{t("Generated tokens")}</span>
                    <span class="demo-muted">{t("One TOML file produces CSS variables, a Rust crate and Kotlin constants.")}</span>
                </a>
                <a class="docs-feature" href="#/porting">
                    <span class="docs-feature-title">{t("Specs for agents")}</span>
                    <span class="demo-muted">{t("Every page here is a Markdown spec an AI agent can implement on a new platform.")}</span>
                </a>
            </section>
            <section class="docs-index">
                <h2 class="docs-index-title">{t("Components")}</h2>
                <div class="docs-index-grid">{tiles}</div>
            </section>
        </div>
    }
}
