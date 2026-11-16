//! Combobox options, sections and filtering (pure logic).

use leptos::prelude::*;
use operator_ui_tokens::Icon as IconData;

/// One choice of a [`Combobox`](super::Combobox) or [`ComboboxPopup`](super::ComboboxPopup).
#[derive(Clone, Debug, PartialEq)]
pub struct ComboOption {
    /// Committed value.
    pub value: String,
    /// Text shown in the list and in the field; the value when empty.
    pub label: String,
    /// Secondary text at the end of the row, for example a type or a unit.
    pub detail: Option<String>,
    pub icon: Option<IconData>,
    /// Section header; consecutive options with the same section are grouped under one header.
    pub section: Option<String>,
}

impl ComboOption {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: String::new(),
            detail: None,
            icon: None,
            section: None,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    pub fn icon(mut self, icon: IconData) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn section(mut self, section: impl Into<String>) -> Self {
        self.section = Some(section.into());
        self
    }

    /// The text shown for the option.
    pub fn text(&self) -> &str {
        if self.label.is_empty() {
            &self.value
        } else {
            &self.label
        }
    }

    /// Case-insensitive match of an already lowercased query against the text or the value.
    pub fn matches(&self, lower_query: &str) -> bool {
        lower_query.is_empty()
            || self.text().to_lowercase().contains(lower_query)
            || self.value.to_lowercase().contains(lower_query)
    }
}

impl From<String> for ComboOption {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for ComboOption {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

/// Options of a combobox: a fixed list or a signal that may change while the list is open.
#[derive(Clone, Copy)]
pub struct ComboOptions(pub Signal<Vec<ComboOption>>);

impl From<Signal<Vec<ComboOption>>> for ComboOptions {
    fn from(s: Signal<Vec<ComboOption>>) -> Self {
        Self(s)
    }
}

impl From<Memo<Vec<ComboOption>>> for ComboOptions {
    fn from(s: Memo<Vec<ComboOption>>) -> Self {
        Self(s.into())
    }
}

impl From<RwSignal<Vec<ComboOption>>> for ComboOptions {
    fn from(s: RwSignal<Vec<ComboOption>>) -> Self {
        Self(s.into())
    }
}

impl From<Vec<ComboOption>> for ComboOptions {
    fn from(v: Vec<ComboOption>) -> Self {
        Self(Signal::stored(v))
    }
}

impl From<Vec<String>> for ComboOptions {
    fn from(v: Vec<String>) -> Self {
        Self(Signal::stored(
            v.into_iter().map(ComboOption::from).collect(),
        ))
    }
}

impl From<Vec<&str>> for ComboOptions {
    fn from(v: Vec<&str>) -> Self {
        Self(Signal::stored(
            v.into_iter().map(ComboOption::from).collect(),
        ))
    }
}

/// One row of the rendered list.
#[derive(Clone, Debug, PartialEq)]
pub enum ComboRow {
    /// Section header.
    Section(String),
    /// An option and its position among the visible options (the keyboard index).
    Option(usize, ComboOption),
}

impl ComboRow {
    /// Stable key of the row.
    pub fn key(&self) -> String {
        match self {
            Self::Section(s) => format!("\u{1}{s}"),
            Self::Option(_, o) => o.value.clone(),
        }
    }
}

/// Rows for `query`: matching options in their order, with a header before each new section.
pub fn filter_rows(options: &[ComboOption], query: &str) -> Vec<ComboRow> {
    let q = query.to_lowercase();
    let mut rows = Vec::new();
    let mut section: Option<&str> = None;
    for (pos, option) in options.iter().filter(|o| o.matches(&q)).enumerate() {
        let current = option.section.as_deref();
        if current != section {
            if let Some(name) = current {
                rows.push(ComboRow::Section(name.to_string()));
            }
            section = current;
        }
        rows.push(ComboRow::Option(pos, option.clone()));
    }
    rows
}

/// Number of options among the rows.
pub fn option_count(rows: &[ComboRow]) -> usize {
    rows.iter()
        .filter(|r| matches!(r, ComboRow::Option(..)))
        .count()
}

/// Row index of the option at keyboard position `pos`.
pub fn row_of(rows: &[ComboRow], pos: usize) -> Option<usize> {
    rows.iter()
        .position(|r| matches!(r, ComboRow::Option(p, _) if *p == pos))
}

/// The option at keyboard position `pos`.
pub fn option_at(rows: &[ComboRow], pos: usize) -> Option<ComboOption> {
    rows.iter().find_map(|r| match r {
        ComboRow::Option(p, o) if *p == pos => Some(o.clone()),
        _ => None,
    })
}

/// Keyboard position `delta` steps from `active`, wrapping around `n` options.
pub fn step(active: usize, n: usize, delta: isize) -> usize {
    if n == 0 {
        return 0;
    }
    let n = n.cast_signed();
    (active.cast_signed() + delta).rem_euclid(n).cast_unsigned()
}

/// The text shown in the field for a committed value: the option's label, or the value itself.
pub fn display_text(options: &[ComboOption], value: &str) -> String {
    options
        .iter()
        .find(|o| o.value == value)
        .map_or_else(|| value.to_string(), |o| o.text().to_string())
}

/// The option whose text or value equals `text` exactly, ignoring case.
pub fn find_exact(options: &[ComboOption], text: &str) -> Option<ComboOption> {
    let t = text.to_lowercase();
    options
        .iter()
        .find(|o| o.text().to_lowercase() == t || o.value.to_lowercase() == t)
        .cloned()
}
