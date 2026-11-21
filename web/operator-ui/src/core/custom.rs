//! Customization hooks shared by every component, see `docs/foundations/customization.md`:
//! root class and style, metric overrides as CSS variables, text strings and semantic icons.

use leptos::prelude::*;
use operator_ui_tokens::Icon;

/// Unit of a metric override.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unit {
    /// Design Units (spacing, sizes).
    Du,
    /// Type Units (font sizes, line heights).
    Tu,
}

/// One metric prop of a component, written to the root as a CSS variable when set.
#[derive(Clone, Copy)]
pub struct Metric {
    var: &'static str,
    value: MaybeProp<f32>,
    unit: Unit,
}

impl Metric {
    /// A metric in Design Units, for example `Metric::du("--op-field-height", height)`.
    pub fn du(var: &'static str, value: MaybeProp<f32>) -> Self {
        Self {
            var,
            value,
            unit: Unit::Du,
        }
    }

    /// A metric in Type Units, for example `Metric::tu("--op-field-font-size", font_size)`.
    pub fn tu(var: &'static str, value: MaybeProp<f32>) -> Self {
        Self {
            var,
            value,
            unit: Unit::Tu,
        }
    }

    fn declaration(&self) -> Option<String> {
        let value = self.value.get()?;
        let unit = match self.unit {
            Unit::Du => "--op-du",
            Unit::Tu => "--op-tu",
        };
        Some(format!("{}:calc(var({unit}) * {value});", self.var))
    }
}

/// Root `class` of a component: its own classes plus the caller's `class` prop.
pub fn root_class(
    base: &'static str,
    extra: MaybeProp<String>,
) -> impl Fn() -> String + Send + Sync + 'static {
    move || match extra.get() {
        Some(extra) if !extra.trim().is_empty() => format!("{base} {extra}"),
        _ => base.to_string(),
    }
}

/// Root `style` of a component: the set metric variables followed by the caller's `style` prop,
/// so an explicit `style` wins over a metric prop.
pub fn root_style(
    metrics: Vec<Metric>,
    style: MaybeProp<String>,
) -> impl Fn() -> Option<String> + Send + Sync + 'static {
    move || {
        let mut out: String = metrics.iter().filter_map(Metric::declaration).collect();
        if let Some(style) = style.get() {
            out.push_str(&style);
        }
        (!out.is_empty()).then_some(out)
    }
}

/// Lookup of translated component strings, provided by [`ThemeRoot`](crate::core::ThemeRoot).
///
/// The callback receives a key such as `"search-field.clear"` and returns the text for the current
/// language, or `None` to keep the English default. Read the language from a signal inside the
/// callback so every string follows a language switch.
#[derive(Clone, Copy)]
pub struct UiStrings(pub Callback<&'static str, Option<String>>);

/// Replacement of semantic icons, provided by [`ThemeRoot`](crate::core::ThemeRoot).
///
/// The callback receives a key such as `"field.clear"` and returns the icon to draw, or `None` to
/// keep the default.
#[derive(Clone, Copy)]
pub struct UiIcons(pub Callback<&'static str, Option<Icon>>);

/// A component string: the caller's prop, else the app's translation for `key`, else `default`.
pub fn ui_text(
    prop: MaybeProp<String>,
    key: &'static str,
    default: &'static str,
) -> Signal<String> {
    let strings = use_context::<UiStrings>();
    Signal::derive(move || {
        prop.get()
            .or_else(|| strings.and_then(|s| s.0.try_run(key).flatten()))
            .unwrap_or_else(|| default.to_string())
    })
}

/// A component icon: the caller's prop, else the app's icon for `key`, else `default`.
pub fn ui_icon(prop: Option<Icon>, key: &'static str, default: Icon) -> Signal<Icon> {
    let icons = use_context::<UiIcons>();
    Signal::derive(move || {
        prop.or_else(|| icons.and_then(|i| i.0.try_run(key).flatten()))
            .unwrap_or(default)
    })
}

/// Optional content a caller places into a named part of a component (Quasar-style slot).
///
/// Accepts a closure (`before=|| view! { ... }`), a `ViewFn`, or an `Option<ViewFn>`, so a
/// component can forward its own slot props to an inner component unchanged.
#[derive(Clone, Default)]
pub struct Slot(Option<ViewFn>);

impl Slot {
    /// Whether the caller filled the slot.
    pub fn is_set(&self) -> bool {
        self.0.is_some()
    }

    /// Renders the content, or nothing for an empty slot.
    pub fn run(&self) -> Option<AnyView> {
        self.0.as_ref().map(ViewFn::run)
    }

    /// This slot when filled, else `fallback`.
    pub fn or(self, fallback: impl Into<Slot>) -> Slot {
        if self.is_set() { self } else { fallback.into() }
    }
}

impl<F, V> From<F> for Slot
where
    F: Fn() -> V + Send + Sync + 'static,
    V: IntoView + 'static,
{
    fn from(f: F) -> Self {
        Self(Some(ViewFn::from(f)))
    }
}

impl From<ViewFn> for Slot {
    fn from(f: ViewFn) -> Self {
        Self(Some(f))
    }
}

impl From<Option<ViewFn>> for Slot {
    fn from(f: Option<ViewFn>) -> Self {
        Self(f)
    }
}

/// Slot that receives data from the component, for example one option row of a list.
pub type ScopedSlot<T> = Callback<T, AnyView>;
