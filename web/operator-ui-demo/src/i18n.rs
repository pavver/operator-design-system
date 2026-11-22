//! Site translations and the language context shared with examples.

mod example_notes;
mod example_titles;

use leptos::prelude::*;
use operator_ui::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lang {
    En,
    Uk,
}

impl Lang {
    pub const ALL: &[Lang] = &[Self::En, Self::Uk];

    pub fn code(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Uk => "uk",
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        let trimmed = code.trim();
        Self::ALL
            .iter()
            .copied()
            .find(|&lang| lang.code().eq_ignore_ascii_case(trimmed))
    }

    pub fn to_operator_language(self) -> Option<Language> {
        Language::from_code(self.code())
    }

    pub fn to_language_option(self) -> LanguageOption {
        if let Some(op_lang) = self.to_operator_language() {
            LanguageOption::from(op_lang)
        } else {
            LanguageOption::new(self.code(), self.code(), self.code())
        }
    }
}

/// Returns the site's supported languages mapped to Operator UI LanguageOptions.
pub fn supported_languages() -> Vec<LanguageOption> {
    Lang::ALL.iter().map(|&l| l.to_language_option()).collect()
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
    if let Some(lang) = stored.as_deref().and_then(Lang::from_code) {
        return lang;
    }
    if let Some(nav_lang) = window.navigator().language() {
        let nav_lower = nav_lang.to_ascii_lowercase();
        for &lang in Lang::ALL {
            if nav_lower.starts_with(lang.code()) {
                return lang;
            }
        }
    }
    Lang::En
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
        "Media extension" => "Розширення медіа",
        "Media" => "Медіа",
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
        "Changes to review" => "Зміни для перевірки",
        "No changes awaiting review." => "Немає змін, що очікують перевірки.",
        "New component" => "Новий компонент",
        "Feature" => "Нова можливість",
        "Fix" => "Виправлення",
        "Breaking change" => "Несумісна зміна",
        "Refactor" => "Рефакторинг",
        "Change" => "Зміна",
        "What to check" => "Що перевірити",
        "Migration" => "Перехід на нову версію",
        "last change" => "остання зміна",
        "just now" => "щойно",
        "Simulated backend" => "Симулятор сервера",
        "Success" => "Успіх",
        "Error" => "Помилка",
        "Alternate" => "Почергово",
        _ => example_titles::ukrainian(key)
            .or_else(|| example_notes::ukrainian(key))
            .unwrap_or(key),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lang_code_and_from_code() {
        assert_eq!(Lang::En.code(), "en");
        assert_eq!(Lang::Uk.code(), "uk");
        assert_eq!(Lang::from_code("en"), Some(Lang::En));
        assert_eq!(Lang::from_code("UK"), Some(Lang::Uk));
        assert_eq!(Lang::from_code("de"), None);
    }

    #[test]
    fn test_supported_languages() {
        let langs = supported_languages();
        assert_eq!(langs.len(), Lang::ALL.len());
        assert_eq!(langs[0].code, "en");
        assert_eq!(langs[1].code, "uk");
        assert!(langs[0].flag.is_some());
        assert!(langs[1].flag.is_some());
    }
}
