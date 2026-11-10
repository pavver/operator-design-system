//! Theme root: injects the stylesheet and exposes the active theme through context.

use leptos::prelude::*;
pub use operator_ui_tokens::Theme;

const TOKENS_CSS: &str = include_str!("../style/tokens.css");
const COMPONENTS_CSS: &str = include_str!("../style/operator-ui.css");

/// Active theme and mode, provided to every descendant component.
#[derive(Clone, Copy)]
pub struct OperatorTheme {
    pub theme: RwSignal<Theme>,
    pub dark: RwSignal<bool>,
}

impl OperatorTheme {
    /// Returns the nearest theme context, or the default dark Operator theme.
    pub fn current() -> Self {
        use_context::<Self>().unwrap_or_else(|| Self {
            theme: RwSignal::new(Theme::Operator),
            dark: RwSignal::new(true),
        })
    }
}

/// Root container that applies `data-op-theme` / `data-op-mode` and injects the CSS once.
///
/// Nested roots are allowed (e.g. side-by-side dark and light previews);
/// only the outermost root injects the stylesheet.
#[component]
pub fn ThemeRoot(
    #[prop(optional)] theme: Option<RwSignal<Theme>>,
    #[prop(optional)] dark: Option<RwSignal<bool>>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let parent = use_context::<OperatorTheme>();
    let ctx = OperatorTheme {
        theme: theme.unwrap_or_else(|| {
            parent
                .map(|p| p.theme)
                .unwrap_or_else(|| RwSignal::new(Theme::Operator))
        }),
        dark: dark.unwrap_or_else(|| {
            parent
                .map(|p| p.dark)
                .unwrap_or_else(|| RwSignal::new(true))
        }),
    };
    provide_context(ctx);
    let inject = parent.is_none();
    view! {
        <div
            class=format!("op-root {class}")
            data-op-theme=move || ctx.theme.get().id()
            data-op-mode=move || if ctx.dark.get() { "dark" } else { "light" }
        >
            {inject.then(|| view! { <style>{TOKENS_CSS}{COMPONENTS_CSS}</style> })}
            {children()}
        </div>
    }
}
