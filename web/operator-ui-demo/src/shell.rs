//! Shared documentation navigation and top bar.

use crate::i18n::{Lang, lang, select_language, t};
use crate::nav::NAV;
use crate::routing::Route;
use crate::{REPO_URL, VERSION, docs};
use leptos::prelude::*;
use operator_ui::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[component]
pub fn StatusPill(status: Option<&'static str>) -> impl IntoView {
    status.map(|s| {
        view! { <span class="docs-pill" data-status=s>{t(s)}</span> }
    })
}

#[component]
pub fn Sidebar(route: Memo<Route>, on_navigate: Callback<()>) -> impl IntoView {
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
                            <span>{move || doc.title(lang())}</span>
                            <ChangeMarker doc=doc/>
                        </a>
                    }
                })
                .collect_view();
            view! {
                <div class="docs-nav-group">
                    <div class="docs-nav-heading">{t(group)}</div>
                    {links}
                </div>
            }
        })
        .collect_view();
    view! { <nav class="docs-nav" aria-label=t("Documentation")>{groups}</nav> }
}

#[component]
pub fn TopBar(dark: RwSignal<bool>, menu_open: RwSignal<bool>) -> impl IntoView {
    let language = use_context::<RwSignal<Lang>>().unwrap_or_else(|| RwSignal::new(Lang::En));
    let selected_code = RwSignal::new(language.get_untracked().code().to_string());
    Effect::new(move |_| {
        let cur = language.get().code().to_string();
        if selected_code.get_untracked() != cur {
            selected_code.set(cur);
        }
    });

    let is_phone = RwSignal::new(false);
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            let win = window.clone();
            let check = move || {
                if let Ok(Some(mql)) = win.match_media("(max-width: 48em)") {
                    mql.matches()
                } else if let Ok(inner) = win.inner_width() {
                    inner.as_f64().is_some_and(|w| w <= 768.0)
                } else {
                    false
                }
            };
            is_phone.set(check());
            let resize_cb = wasm_bindgen::closure::Closure::<dyn Fn()>::new(move || {
                is_phone.set(check());
            });
            let _ = window
                .add_event_listener_with_callback("resize", resize_cb.as_ref().unchecked_ref());
            resize_cb.forget();
        }
    }

    let format_cb = Callback::new(move |opt: LanguageOption| {
        if is_phone.get() {
            opt.code.to_uppercase()
        } else if !opt.native.is_empty() {
            opt.native
        } else {
            opt.english
        }
    });

    let width_signal = Signal::derive(move || if is_phone.get() { 19.0 } else { 42.0 });

    view! {
        <header class="docs-topbar">
            <div class="docs-topbar-inner">
                <span class="docs-menu-btn">
                    {move || view! { <IconButton icon=icons::MENU label=crate::i18n::tr(lang(), "Open navigation") on_click=Callback::new(move |()| menu_open.set(true))/> }}
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
                <nav class="docs-topnav" aria-label=t("Sections")>
                    <a href="#/platforms/web">{t("Docs")}</a>
                    <a href="#/foundations/principles">{t("Foundations")}</a>
                    <a href="#/components/action-button">{t("Components")}</a>
                </nav>
                <span class="docs-topbar-spacer"></span>
                <a class="docs-toplink" href=REPO_URL>"GitHub"</a>
                <a class="docs-toplink" href=format!("{REPO_URL}/releases")>{t("Releases")}</a>
                <LanguagePicker
                    value=selected_code
                    languages=Signal::derive(crate::i18n::supported_languages)
                    on_select=Callback::new(move |code: String| {
                        if let Some(l) = Lang::from_code(&code) {
                            select_language(language, l);
                        }
                    })
                    height=6.0
                    width=width_signal
                    format=format_cb
                    aria_label=t("Language")
                    class="docs-header-lang"
                    option=Callback::new(move |opt: LanguageOption| {
                        let label = default_format(&opt);
                        view! {
                            {opt.flag.as_ref().map(render_flag)}
                            <span class="op-combo-label">{label}</span>
                        }
                        .into_any()
                    })
                />
                <ThemeToggle attr:aria-label=t("Dark mode") dark=dark on_toggle=Callback::new(move |v: bool| dark.set(v)) width=13.0 height=6.0/>
            </div>
        </header>
    }
}

#[component]
pub fn ChangeMarker(doc: &'static docs::Doc) -> impl IntoView {
    if doc.status == Some("draft") {
        view! { <span class="docs-nav-draft">{t("draft")}</span> }.into_any()
    } else if doc.status == Some("reviewed") && doc.draft_changes() > 0 {
        view! { <span class="docs-change-marker" title=t("has unreviewed changes") role="img" aria-label=t("has unreviewed changes")>"●"</span> }.into_any()
    } else {
        ().into_any()
    }
}
