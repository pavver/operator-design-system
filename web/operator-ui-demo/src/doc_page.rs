//! Document article, localized content, examples and table of contents.

use crate::REPO_URL;
use crate::docs;
use crate::history::{ChangeHistory, ChangesToReview};
use crate::i18n::{Lang, lang, t};
use crate::kit::SimBar;
use crate::nav::{group_of, neighbours};
use crate::pages;
use crate::shell::StatusPill;
use leptos::prelude::*;

#[component]
pub fn DocPage(doc: &'static docs::Doc) -> impl IntoView {
    let examples = pages::examples(doc.slug);
    let has_examples = examples.is_some();
    let (prev, next) = neighbours(doc.slug);
    let link = |slug: Option<&'static str>, label: &'static str, class: &'static str| {
        slug.and_then(docs::find).map(|d| {
            view! {
                <a class=format!("docs-pager-link {class}") href=format!("#/{}", d.slug)>
                    <span class="op-t-caption demo-muted">{t(label)}</span>
                    <span>{move || d.title(lang())}</span>
                </a>
            }
        })
    };
    let toc = move || {
        doc.toc(lang())
            .iter()
            .map(|(id, title)| {
                view! { <a href=format!("#/{}/{id}", doc.slug)>{*title}</a> }
            })
            .collect_view()
    };
    view! {
        <div class="docs-page">
            <article class="docs-article">
                {move || (lang() == Lang::Uk && doc.uk_body.is_none()).then(|| view! {
                    <p class="docs-translation-notice" role="status">{t("This page has not been translated yet")}</p>
                })}
                <header class="docs-head">
                    <div class="docs-kicker">{t(group_of(doc.slug).unwrap_or("Docs"))}</div>
                    <div class="docs-title-row">
                        <h1 class="docs-title">{move || doc.title(lang())}</h1>
                        <StatusPill status=doc.status/>
                    </div>
                    <div class="docs-lead" inner_html=move || doc.lead(lang())></div>
                </header>
                {examples.map(|(render, uses_backend)| view! {
                    <section class="docs-examples">
                        <h2 id="examples">{t("Examples")}</h2>
                        {uses_backend.then(|| view! { <SimBar/> })}
                        {render()}
                    </section>
                })}
                <div class="docs-md" inner_html=move || doc.body(lang())></div>
                {(doc.slug == "changes-to-review").then(|| view! { <ChangesToReview/> })}
                <footer class="docs-foot">
                    <div class="docs-pager">
                        {link(prev, "Previous", "docs-pager-prev")}
                        {link(next, "Next", "docs-pager-next")}
                    </div>
                    <div class="docs-foot-links op-t-caption">
                        <a href=move || {
                            let prefix = if lang() == Lang::Uk && doc.uk_body.is_some() { "i18n/uk/" } else { "" };
                            format!("{REPO_URL}/blob/main/docs/{prefix}{}.md", doc.slug)
                        }>{t("View Markdown source")}</a>
                        <span class="demo-muted">{t("The same file is the specification for AI agents.")}</span>
                    </div>
                </footer>
                <ChangeHistory doc=doc/>
            </article>
            <aside class="docs-toc" aria-label=t("On this page")>
                <div class="docs-toc-title">{t("On this page")}</div>
                {has_examples.then(|| view! { <a href=format!("#/{}/examples", doc.slug)>{t("Examples")}</a> })}
                {toc}
            </aside>
        </div>
    }
}
