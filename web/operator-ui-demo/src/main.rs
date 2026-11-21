//! Operator UI documentation, generated from the shared Markdown specifications.

mod component_strings;
mod doc_page;
mod docs;
mod history;
mod home;
mod i18n;
mod kit;
mod nav;
mod pages;
mod routing;
mod shell;
mod showcase;

use doc_page::DocPage;
use home::Home;
use i18n::{lang, t, tr};
use kit::Backend;
use leptos::ev;
use leptos::prelude::*;
use operator_ui::prelude::*;
use operator_ui::tokens::Theme as TokenTheme;
use routing::{current_hash, parse_route, scroll_to};
use shell::{Sidebar, TopBar};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const REPO_URL: &str = "https://github.com/pavver/operator-design-system";

#[component]
fn NotFound(slug: String) -> impl IntoView {
    let slug = StoredValue::new(slug);
    view! {
        <div class="docs-page">
            <article class="docs-article">
                {move || view! { <EmptyState
                    icon=icons::SEARCH
                    title=tr(lang(), "Page not found")
                    description=format!("{} \"{}\".", tr(lang(), "There is no page named"), slug.get_value())
                    actions=ViewFn::from(|| view! { <a class="docs-cta" href="#/">{t("Back to the start")}</a> })
                /> }}
            </article>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    let language = RwSignal::new(i18n::initial_language());
    provide_context(language);
    // Read the signal inside the lookup so mounted components follow the language picker.
    let strings = Callback::new(move |key: &'static str| match language.get() {
        i18n::Lang::En => None,
        i18n::Lang::Uk => component_strings::ukrainian(key).map(str::to_string),
    });
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
            |d| format!("{} · Operator UI", d.title(lang())),
        );
        let locale = language.get().code();
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            doc.set_title(&title);
            if let Some(html) = doc.document_element() {
                let _ = html.set_attribute("lang", locale);
            }
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
        <ThemeRoot theme=theme dark=dark strings=strings class="docs-root">
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
    // `#/showcase` is a standalone page without the documentation chrome.
    if parse_route(&current_hash()).slug == "showcase" {
        mount_to_body(|| view! { <ThemeRoot class="sc-page"><showcase::Showcase/></ThemeRoot> });
    } else {
        mount_to_body(App);
    }
}
