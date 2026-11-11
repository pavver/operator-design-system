//! Operator UI documentation site: one page per foundation and component, with live examples.
//!
//! Page text comes from `docs/**/*.md` (rendered by `build.rs`), so the site and the Markdown
//! read by agents never diverge.

mod docs;
mod kit;
mod pages;

use kit::{Backend, SimBar};
use leptos::ev;
use leptos::prelude::*;
use operator_ui::prelude::*;
use operator_ui::tokens::Theme as TokenTheme;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const REPO_URL: &str = "https://github.com/pavver/operator-design-system";

/// Sidebar groups and the documents they list, in order.
const NAV: &[(&str, &[&str])] = &[
    (
        "Getting started",
        &["platforms/web", "tokens", "porting", "releasing"],
    ),
    (
        "Foundations",
        &[
            "foundations/principles",
            "foundations/units",
            "foundations/color",
            "foundations/typography",
            "foundations/iconography",
            "foundations/motion",
            "foundations/states",
        ],
    ),
    (
        "Actions",
        &[
            "components/action-button",
            "components/icon-button",
            "components/menu",
            "components/toolbar",
        ],
    ),
    (
        "Selection",
        &[
            "components/switch",
            "components/checkbox",
            "components/checkbox-group",
            "components/radio-group",
            "components/segmented-control",
            "components/slider",
            "components/stepper",
            "components/chip",
            "components/theme-toggle",
        ],
    ),
    (
        "Text input",
        &[
            "components/text-field",
            "components/text-area",
            "components/search-field",
            "components/dropdown-menu",
            "components/combobox",
        ],
    ),
    (
        "Feedback",
        &[
            "components/tooltip",
            "components/toast",
            "components/banner",
            "components/linear-progress",
            "components/skeleton",
            "components/empty-state",
            "components/status-badge",
        ],
    ),
    (
        "Overlays",
        &[
            "components/dialog",
            "components/confirm-dialog",
            "components/sheet",
        ],
    ),
    (
        "Navigation",
        &[
            "components/header",
            "components/tabs",
            "components/pagination",
        ],
    ),
    (
        "Data display",
        &[
            "components/list-item",
            "components/data-table",
            "components/property-list",
            "components/accordion",
            "components/counter",
            "components/avatar",
            "components/code-block",
            "components/kbd",
            "components/console-terminal",
            "components/marquee-text",
            "components/primitives",
        ],
    ),
    (
        "Layout",
        &[
            "components/split-pane",
            "components/scroll-area",
            "components/scrollbar",
        ],
    ),
];

fn group_of(slug: &str) -> Option<&'static str> {
    NAV.iter()
        .find(|(_, items)| items.contains(&slug))
        .map(|(g, _)| *g)
}

/// Neighbouring pages in sidebar order.
fn neighbours(slug: &str) -> (Option<&'static str>, Option<&'static str>) {
    let all: Vec<&'static str> = NAV.iter().flat_map(|(_, i)| i.iter().copied()).collect();
    let Some(i) = all.iter().position(|s| *s == slug) else {
        return (None, None);
    };
    (
        i.checked_sub(1).and_then(|p| all.get(p).copied()),
        all.get(i + 1).copied(),
    )
}

// ---------------------------------------------------------------------------
// Routing: #/<slug> or #/<slug>/<anchor>
// ---------------------------------------------------------------------------

#[derive(Clone, PartialEq, Eq)]
struct Route {
    slug: String,
    anchor: Option<String>,
}

fn current_hash() -> String {
    web_sys::window()
        .and_then(|w| w.location().hash().ok())
        .unwrap_or_default()
}

fn parse_route(hash: &str) -> Route {
    let path = hash.trim_start_matches('#').trim_start_matches('/');
    if path.is_empty() {
        return Route {
            slug: String::new(),
            anchor: None,
        };
    }
    // The longest prefix that names a document is the page; the rest is an anchor.
    let parts: Vec<&str> = path.split('/').collect();
    for n in (1..=parts.len()).rev() {
        let slug = parts[..n].join("/");
        if docs::find(&slug).is_some() {
            let anchor = (n < parts.len()).then(|| parts[n..].join("/"));
            return Route { slug, anchor };
        }
    }
    Route {
        slug: path.to_string(),
        anchor: None,
    }
}

fn scroll_to(id: Option<&str>) {
    let Some(window) = web_sys::window() else {
        return;
    };
    match id.and_then(|id| window.document().and_then(|d| d.get_element_by_id(id))) {
        Some(el) => el.scroll_into_view(),
        None => window.scroll_to_with_x_and_y(0.0, 0.0),
    }
}

// ---------------------------------------------------------------------------
// Shell
// ---------------------------------------------------------------------------

#[component]
fn StatusPill(status: Option<&'static str>) -> impl IntoView {
    status.map(|s| {
        view! { <span class="docs-pill" data-status=s>{s}</span> }
    })
}

#[component]
fn Sidebar(route: Memo<Route>, on_navigate: Callback<()>) -> impl IntoView {
    let groups = NAV
        .iter()
        .map(|(group, items)| {
            let links = items
                .iter()
                .filter_map(|slug| docs::find(slug))
                .map(|doc| {
                    let slug = doc.slug;
                    let active = move || route.with(|r| r.slug == slug);
                    view! {
                        <a
                            href=format!("#/{slug}")
                            class="docs-nav-link"
                            aria-current=move || active().then_some("page")
                            on:click=move |_| on_navigate.run(())
                        >
                            <span>{doc.title}</span>
                            {(doc.status == Some("draft")).then(|| view! { <span class="docs-nav-draft">"draft"</span> })}
                        </a>
                    }
                })
                .collect_view();
            view! {
                <div class="docs-nav-group">
                    <div class="docs-nav-heading">{*group}</div>
                    {links}
                </div>
            }
        })
        .collect_view();
    view! { <nav class="docs-nav" aria-label="Documentation">{groups}</nav> }
}

#[component]
fn TopBar(dark: RwSignal<bool>, menu_open: RwSignal<bool>) -> impl IntoView {
    view! {
        <header class="docs-topbar">
            <div class="docs-topbar-inner">
                <span class="docs-menu-btn">
                    <IconButton icon=icons::MENU label="Open navigation" on_click=Callback::new(move |()| menu_open.set(true))/>
                </span>
                <a class="docs-brand" href="#/">
                    <svg class="docs-brand-mark" viewBox="0 0 24 24" aria-hidden="true">
                        <rect class="lg-ac" x="4.5" y="5" width="15" height="4"></rect>
                        <rect class="lg-fg" x="4.5" y="10" width="11" height="4"></rect>
                        <rect class="lg-am" x="16.5" y="10" width="3" height="4"></rect>
                        <rect class="lg-dim" x="4.5" y="15" width="7" height="4"></rect>
                        <rect class="lg-btn" x="12.875" y="15.375" width="6.25" height="3.25"></rect>
                        <rect class="lg-fg" x="14.5" y="16.5" width="3" height="1"></rect>
                        <path class="lg-br" d="M5.5 3H2.5v18h3"></path>
                        <path class="lg-br" d="M18.5 3h3v18h-3"></path>
                    </svg>
                    <span class="docs-brand-name">"Operator UI"</span>
                    <span class="docs-version">{format!("v{VERSION}")}</span>
                </a>
                <nav class="docs-topnav" aria-label="Sections">
                    <a href="#/platforms/web">"Docs"</a>
                    <a href="#/foundations/principles">"Foundations"</a>
                    <a href="#/components/action-button">"Components"</a>
                </nav>
                <span class="docs-topbar-spacer"></span>
                <a class="docs-toplink" href=REPO_URL>"GitHub"</a>
                <a class="docs-toplink" href=format!("{REPO_URL}/releases")>"Releases"</a>
                <ThemeToggle dark=dark on_toggle=Callback::new(move |v: bool| dark.set(v)) width=13.0 height=6.0/>
            </div>
        </header>
    }
}

#[component]
fn DocPage(doc: &'static docs::Doc) -> impl IntoView {
    let examples = pages::examples(doc.slug);
    let has_examples = examples.is_some();
    let (prev, next) = neighbours(doc.slug);
    let link = |slug: Option<&'static str>, label: &'static str, class: &'static str| {
        slug.and_then(docs::find).map(|d| {
            view! {
                <a class=format!("docs-pager-link {class}") href=format!("#/{}", d.slug)>
                    <span class="op-t-caption demo-muted">{label}</span>
                    <span>{d.title}</span>
                </a>
            }
        })
    };
    let toc = doc
        .toc
        .iter()
        .map(|(id, title)| view! { <a href=format!("#/{}/{id}", doc.slug)>{*title}</a> })
        .collect_view();
    view! {
        <div class="docs-page">
            <article class="docs-article">
                <header class="docs-head">
                    <div class="docs-kicker">{group_of(doc.slug).unwrap_or("Docs")}</div>
                    <div class="docs-title-row">
                        <h1 class="docs-title">{doc.title}</h1>
                        <StatusPill status=doc.status/>
                    </div>
                    <div class="docs-lead" inner_html=doc.lead></div>
                </header>
                {examples
                    .map(|(render, uses_backend)| {
                        view! {
                            <section class="docs-examples">
                                <h2 id="examples">"Examples"</h2>
                                {uses_backend.then(|| view! { <SimBar/> })}
                                {render()}
                            </section>
                        }
                    })}
                <div class="docs-md" inner_html=doc.body></div>
                <footer class="docs-foot">
                    <div class="docs-pager">
                        {link(prev, "Previous", "docs-pager-prev")}
                        {link(next, "Next", "docs-pager-next")}
                    </div>
                    <div class="docs-foot-links op-t-caption">
                        <a href=format!("{REPO_URL}/blob/main/docs/{}.md", doc.slug)>"View Markdown source"</a>
                        <span class="demo-muted">"The same file is the specification for AI agents."</span>
                    </div>
                </footer>
            </article>
            <aside class="docs-toc" aria-label="On this page">
                <div class="docs-toc-title">"On this page"</div>
                {has_examples.then(|| view! { <a href=format!("#/{}/examples", doc.slug)>"Examples"</a> })}
                {toc}
            </aside>
        </div>
    }
}

fn count_status(status: &str) -> usize {
    docs::DOCS
        .iter()
        .filter(|d| d.slug.starts_with("components/") && d.status == Some(status))
        .count()
}

#[component]
fn Home() -> impl IntoView {
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
                            <span>{d.title}</span>
                            {(d.status == Some("draft")).then(|| view! { <span class="docs-nav-draft">"draft"</span> })}
                        </a>
                    }
                })
                .collect_view();
            view! {
                <div class="docs-index-group">
                    <div class="docs-nav-heading">{*group}</div>
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
                        "A design system for operator consoles: dense, rectangular, monospaced. "
                        "Every control shows the life of an asynchronous request — pending, success, error — "
                        "with one visual language on every platform."
                    </p>
                    <div class="docs-hero-actions">
                        <a class="docs-cta docs-cta-primary" href="#/platforms/web">"Get started"</a>
                        <a class="docs-cta" href="#/foundations/principles">"Read the principles"</a>
                        <a class="docs-cta" href=REPO_URL>"GitHub"</a>
                    </div>
                    <div class="docs-hero-stats op-t-caption demo-muted">
                        {format!("{} reviewed components · {} drafts · tokens for web, Rust and Kotlin", count_status("reviewed"), count_status("draft"))}
                    </div>
                </div>
                <div class="docs-hero-demo">
                    <Card>
                        <div class="demo-stack">
                            <ListItem
                                title="Night mode"
                                subtitle="Commits after the camera confirms"
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
                                <ActionButton text="Restart stream" icon=icons::REFRESH on_click=b.action("restart stream")/>
                                <ActionButton text="Delete" dangerous=true on_click=b.action("delete camera")/>
                            </div>
                            <SimBar/>
                        </div>
                    </Card>
                </div>
            </section>
            <section class="docs-features">
                <a class="docs-feature" href="#/foundations/states">
                    <span class="docs-feature-title">"Async states"</span>
                    <span class="demo-muted">"Pending, success and error are part of every control, not an afterthought."</span>
                </a>
                <a class="docs-feature" href="#/foundations/units">
                    <span class="docs-feature-title">"Design Units"</span>
                    <span class="demo-muted">"Geometry in DU, text in TU. One grid for web, desktop and Android."</span>
                </a>
                <a class="docs-feature" href="#/tokens">
                    <span class="docs-feature-title">"Generated tokens"</span>
                    <span class="demo-muted">"One TOML file produces CSS variables, a Rust crate and Kotlin constants."</span>
                </a>
                <a class="docs-feature" href="#/porting">
                    <span class="docs-feature-title">"Specs for agents"</span>
                    <span class="demo-muted">"Every page here is a Markdown spec an AI agent can implement on a new platform."</span>
                </a>
            </section>
            <section class="docs-index">
                <h2 class="docs-index-title">"Components"</h2>
                <div class="docs-index-grid">{tiles}</div>
            </section>
        </div>
    }
}

#[component]
fn NotFound(slug: String) -> impl IntoView {
    view! {
        <div class="docs-page">
            <article class="docs-article">
                <EmptyState
                    icon=icons::SEARCH
                    title="Page not found"
                    description=format!("There is no page named \"{slug}\".")
                    actions=ViewFn::from(|| view! { <a class="docs-cta" href="#/">"Back to the start"</a> })
                />
            </article>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    let theme = RwSignal::new(TokenTheme::Operator);
    let dark = RwSignal::new(true);
    provide_context(Backend::new());

    let hash = RwSignal::new(current_hash());
    let listener = window_event_listener(ev::hashchange, move |_| hash.set(current_hash()));
    on_cleanup(move || listener.remove());
    let route = Memo::new(move |_| parse_route(&hash.get()));
    let slug = Memo::new(move |_| route.with(|r| r.slug.clone()));
    let menu_open = RwSignal::new(false);
    let close_menu = Callback::new(move |()| menu_open.set(false));

    // Scroll to the anchor after the page renders, or to the top on a new page.
    Effect::new(move |_| {
        let anchor = route.with(|r| r.anchor.clone());
        request_animation_frame(move || scroll_to(anchor.as_deref()));
    });
    Effect::new(move |_| {
        let title = docs::find(&slug.get()).map_or_else(
            || "Operator UI".to_string(),
            |d| format!("{} · Operator UI", d.title),
        );
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            doc.set_title(&title);
        }
    });

    let content = move || {
        let s = slug.get();
        if s.is_empty() {
            return view! { <Home/> }.into_any();
        }
        match docs::find(&s) {
            Some(doc) => view! { <DocPage doc=doc/> }.into_any(),
            None => view! { <NotFound slug=s/> }.into_any(),
        }
    };

    view! {
        <ThemeRoot theme=theme dark=dark class="docs-root">
            <ToastHost>
                <TopBar dark=dark menu_open=menu_open/>
                <div class="docs-layout" data-home=move || slug.with(String::is_empty).then_some("")>
                    <aside class="docs-sidebar">
                        <Sidebar route=route on_navigate=close_menu/>
                    </aside>
                    <main class="docs-main">{content}</main>
                </div>
                <Sheet open=menu_open title="Operator UI">
                    <Sidebar route=route on_navigate=close_menu/>
                </Sheet>
            </ToastHost>
        </ThemeRoot>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
