//! Localized component history, with browser-local dates and relative times.

use crate::docs::{Change, Doc, DOCS};
use crate::i18n::{Lang, lang, t, tr};
use crate::shell::StatusPill;
use leptos::prelude::*;
use std::time::Duration;

fn plural(n: u64, one: &'static str, few: &'static str, many: &'static str) -> &'static str {
    if (11..=14).contains(&(n % 100)) {
        many
    } else {
        match n % 10 {
            1 => one,
            2..=4 => few,
            _ => many,
        }
    }
}

pub fn relative_time(language: Lang, elapsed_ms: f64) -> String {
    let minutes = (elapsed_ms.max(0.0) / 60_000.0) as u64;
    if minutes == 0 {
        return tr(language, "just now").to_string();
    }
    let (n, en, forms) = if minutes < 60 {
        (minutes, "minute", ("хвилину", "хвилини", "хвилин"))
    } else if minutes < 1_440 {
        (minutes / 60, "hour", ("годину", "години", "годин"))
    } else if minutes < 43_200 {
        (minutes / 1_440, "day", ("день", "дні", "днів"))
    } else if minutes < 525_600 {
        (minutes / 43_200, "month", ("місяць", "місяці", "місяців"))
    } else {
        (minutes / 525_600, "year", ("рік", "роки", "років"))
    };
    match language {
        Lang::En => format!("{n} {en}{} ago", if n == 1 { "" } else { "s" }),
        Lang::Uk => format!("{n} {} тому", plural(n, forms.0, forms.1, forms.2)),
    }
}

fn draft_count(language: Lang, count: usize) -> String {
    match language {
        Lang::En => format!("{count} draft"),
        Lang::Uk => format!(
            "{count} {}",
            plural(
                count as u64,
                "неперевірена",
                "неперевірені",
                "неперевірених"
            )
        ),
    }
}

fn date_label(date: &str, language: Lang, summary: bool) -> String {
    let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_str(date));
    let day = date.get_date();
    let month = date.get_month() as usize;
    let year = date.get_full_year();
    if language == Lang::Uk && !summary {
        return format!("{day:02}.{:02}.{year}", month + 1);
    }
    let months = match language {
        Lang::En => [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ],
        Lang::Uk => [
            "січня",
            "лютого",
            "березня",
            "квітня",
            "травня",
            "червня",
            "липня",
            "серпня",
            "вересня",
            "жовтня",
            "листопада",
            "грудня",
        ],
    };
    format!("{day} {} {year}", months.get(month).copied().unwrap_or(""))
}

#[component]
fn ChangeDetail(
    title: &'static str,
    en: Option<&'static str>,
    uk: Option<&'static str>,
) -> impl IntoView {
    move || {
        let html = match lang() {
            Lang::En => en,
            Lang::Uk => uk,
        };
        html.map(|html| view! {
            <div class="docs-history-detail">
                <strong>{t(title)}</strong>
                <div class="docs-md" inner_html=html></div>
            </div>
        })
    }
}

fn kind_label(kind: &str) -> &'static str {
    match kind {
        "new" => "New component",
        "feature" => "Feature",
        "fix" => "Fix",
        "breaking" => "Breaking change",
        "docs" => "Documentation",
        "refactor" => "Refactor",
        _ => "Change",
    }
}

#[component]
fn HistoryRow(
    change: &'static Change,
    #[prop(optional)] doc: Option<&'static Doc>,
) -> impl IntoView {
    view! {
        <li class="docs-history-row">
            {doc.map(|doc| view! {
                <a class="docs-history-component" href=format!("#/{}", doc.slug)>
                    {move || doc.title(lang())}
                </a>
            })}
            <div class="docs-history-meta">
                <time datetime=change.date>{move || date_label(change.date, lang(), false)}</time>
                <StatusPill status=Some(change.status)/>
                <span class="docs-pill" data-kind=change.kind>{t(kind_label(change.kind))}</span>
                {change.commit.map(|hash| view! {
                    <a class="docs-history-commit" href=format!("https://github.com/pavver/operator-design-system/commit/{hash}")>
                        <code>{hash.get(..7).unwrap_or(hash)}</code>
                    </a>
                })}
            </div>
            <div class="docs-md" inner_html=move || match lang() {
                Lang::En => change.en_html,
                Lang::Uk => change.uk_html,
            }></div>
            <ChangeDetail title="What to check" en=change.en_check_html uk=change.uk_check_html/>
            <ChangeDetail title="Migration" en=change.en_migration_html uk=change.uk_migration_html/>
        </li>
    }
}

#[component]
pub fn ChangeHistory(doc: &'static Doc) -> impl IntoView {
    let Some(latest) = doc.changes.first() else {
        return ().into_any();
    };
    let now = RwSignal::new(js_sys::Date::now());
    let timer = set_interval_with_handle(
        move || now.set(js_sys::Date::now()),
        Duration::from_secs(60),
    );
    on_cleanup(move || {
        if let Ok(timer) = timer {
            timer.clear();
        }
    });
    let summary = move || {
        let language = lang();
        let relative = relative_time(language, now.get() - js_sys::Date::parse(latest.date));
        let drafts = doc.draft_changes();
        let suffix = if drafts > 0 {
            format!(" · {}", draft_count(language, drafts))
        } else {
            String::new()
        };
        format!(
            "{} · {} {} ({relative}){suffix}",
            tr(language, "Change history"),
            tr(language, "last change"),
            date_label(latest.date, language, true)
        )
    };
    view! {
        <details class="docs-history">
            <summary>{summary}</summary>
            <ol aria-label=t("Change history")>
                {doc.changes.iter().map(|change| view! { <HistoryRow change=change/> }).collect_view()}
            </ol>
        </details>
    }.into_any()
}

/// The queue always reflects the same draft files as component histories.
#[component]
pub fn ChangesToReview() -> impl IntoView {
    let mut changes: Vec<_> = DOCS
        .iter()
        .flat_map(|doc| {
            doc.changes
                .iter()
                .filter(|change| change.status == "draft")
                .map(move |change| (doc, change))
        })
        .collect();
    changes.sort_by(|a, b| {
        b.1.sort_key.cmp(&a.1.sort_key).then_with(|| a.0.slug.cmp(b.0.slug))
    });
    if changes.is_empty() {
        return view! { <p role="status">{t("No changes awaiting review.")}</p> }.into_any();
    }
    view! {
        <ol class="docs-review-changes" aria-label=t("Changes to review")>
            {changes.into_iter().map(|(doc, change)| view! {
                <HistoryRow change=change doc=doc/>
            }).collect_view()}
        </ol>
    }.into_any()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ukrainian_plural_endings_include_teens_and_compound_numbers() {
        for (n, ending) in [
            (1, "хвилину"),
            (2, "хвилини"),
            (4, "хвилини"),
            (5, "хвилин"),
            (11, "хвилин"),
            (12, "хвилин"),
            (14, "хвилин"),
            (21, "хвилину"),
            (22, "хвилини"),
        ] {
            assert_eq!(
                relative_time(Lang::Uk, n as f64 * 60_000.0),
                format!("{n} {ending} тому")
            );
        }
        assert_eq!(draft_count(Lang::Uk, 3), "3 неперевірені");
        assert_eq!(draft_count(Lang::Uk, 11), "11 неперевірених");
        assert_eq!(draft_count(Lang::Uk, 21), "21 неперевірена");
    }

    #[test]
    fn relative_time_changes_units_at_the_expected_boundaries() {
        for (minutes, en, uk) in [
            (0, "just now", "щойно"),
            (60, "1 hour ago", "1 годину тому"),
            (120, "2 hours ago", "2 години тому"),
            (720, "12 hours ago", "12 годин тому"),
            (1_440, "1 day ago", "1 день тому"),
            (2_880, "2 days ago", "2 дні тому"),
            (43_200, "1 month ago", "1 місяць тому"),
            (86_400, "2 months ago", "2 місяці тому"),
            (525_600, "1 year ago", "1 рік тому"),
            (1_051_200, "2 years ago", "2 роки тому"),
        ] {
            assert_eq!(relative_time(Lang::En, f64::from(minutes) * 60_000.0), en);
            assert_eq!(relative_time(Lang::Uk, f64::from(minutes) * 60_000.0), uk);
        }
        assert_eq!(relative_time(Lang::En, -1.0), "just now");
    }
}
