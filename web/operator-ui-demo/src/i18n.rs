//! Site translations and the language context shared with examples.

use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lang {
    En,
    Uk,
}

impl Lang {
    pub fn code(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Uk => "uk",
        }
    }
}

/// Storage may be unavailable in private browsing or restricted embedded contexts.
pub fn initial_language() -> Lang {
    let Some(window) = web_sys::window() else {
        return Lang::En;
    };
    let stored = window
        .local_storage()
        .ok()
        .flatten()
        .and_then(|storage| storage.get_item("ods-lang").ok().flatten());
    match stored.as_deref() {
        Some("en") => Lang::En,
        Some("uk") => Lang::Uk,
        _ if window
            .navigator()
            .language()
            .is_some_and(|s| s.starts_with("uk")) =>
        {
            Lang::Uk
        }
        _ => Lang::En,
    }
}

pub fn select_language(language: RwSignal<Lang>, value: Lang) {
    language.set(value);
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
        let _ = storage.set_item("ods-lang", value.code());
    }
}

pub fn lang() -> Lang {
    use_context::<RwSignal<Lang>>().map_or(Lang::En, |language| language.get())
}

/// Reactive captions for examples: `l("Search cameras", "Пошук камер")`.
pub fn l(en: &'static str, uk: &'static str) -> Signal<String> {
    Signal::derive(move || match lang() {
        Lang::En => en.to_string(),
        Lang::Uk => uk.to_string(),
    })
}

pub fn t(key: &'static str) -> Signal<String> {
    Signal::derive(move || tr(lang(), key).to_string())
}

/// English strings double as stable lookup keys.
pub fn tr(language: Lang, key: &'static str) -> &'static str {
    if language == Lang::En {
        return key;
    }
    match key {
        "Getting started" => "Початок роботи",
        "Foundations" => "Основи",
        "Actions" => "Дії",
        "Selection" => "Вибір",
        "Text input" => "Введення тексту",
        "Feedback" => "Зворотний зв’язок",
        "Overlays" => "Накладні елементи",
        "Navigation" => "Навігація",
        "Data display" => "Відображення даних",
        "Layout" => "Компонування",
        "Project extensions" => "Розширення для проєктів",
        "draft" => "чернетка",
        "reviewed" => "перевірено",
        "approved" => "затверджено",
        "has unreviewed changes" => "є неперевірені зміни",
        "Documentation" => "Документація",
        "Sections" => "Розділи",
        "Docs" => "Документація",
        "Components" => "Компоненти",
        "Open navigation" => "Відкрити навігацію",
        "Language" => "Мова",
        "Dark mode" => "Темний режим",
        "Releases" => "Випуски",
        "Examples" => "Приклади",
        "On this page" => "На цій сторінці",
        "Previous" => "Попередня",
        "Next" => "Наступна",
        "View Markdown source" => "Переглянути джерело Markdown",
        "The same file is the specification for AI agents." => {
            "Цей самий файл є специфікацією для ШІ-агентів."
        }
        "This page has not been translated yet" => "Цю сторінку ще не перекладено",
        "A design system for operator consoles: dense, rectangular, monospaced. Every control shows the life of an asynchronous request — pending, success, error — with one visual language on every platform." => {
            "Дизайн-система для операторських консолей: щільна, прямокутна, моноширинна. Кожен елемент показує перебіг асинхронного запиту — очікування, успіх, помилку — єдиною візуальною мовою на всіх платформах."
        }
        "Get started" => "Почати роботу",
        "Read the principles" => "Прочитати принципи",
        "reviewed components" => "перевірених компонентів",
        "drafts" => "чернеток",
        "tokens for web, Rust and Kotlin" => "токени для вебу, Rust і Kotlin",
        "Night mode" => "Нічний режим",
        "Commits after the camera confirms" => "Зберігається після підтвердження камери",
        "Restart stream" => "Перезапустити потік",
        "Delete" => "Видалити",
        "Async states" => "Асинхронні стани",
        "Pending, success and error are part of every control, not an afterthought." => {
            "Очікування, успіх і помилка закладені в кожен елемент керування від початку."
        }
        "Design Units" => "Одиниці дизайну",
        "Geometry in DU, text in TU. One grid for web, desktop and Android." => {
            "Геометрія в DU, текст у TU. Єдина сітка для вебу, настільних застосунків і Android."
        }
        "Generated tokens" => "Згенеровані токени",
        "One TOML file produces CSS variables, a Rust crate and Kotlin constants." => {
            "Один файл TOML породжує змінні CSS, крейт Rust і константи Kotlin."
        }
        "Specs for agents" => "Специфікації для агентів",
        "Every page here is a Markdown spec an AI agent can implement on a new platform." => {
            "Кожна сторінка — це специфікація Markdown, за якою ШІ-агент може створити реалізацію для нової платформи."
        }
        "Page not found" => "Сторінку не знайдено",
        "There is no page named" => "Немає сторінки з назвою",
        "Back to the start" => "На головну",
        "Change history" => "Історія змін",
        "last change" => "остання зміна",
        "just now" => "щойно",
        "Simulated backend" => "Симулятор сервера",
        "Success" => "Успіх",
        "Error" => "Помилка",
        "Alternate" => "Почергово",
        _ => key,
    }
}
