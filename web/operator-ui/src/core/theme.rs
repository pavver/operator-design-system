//! Theme root: injects the stylesheet and exposes the active theme through context.

use leptos::prelude::*;
pub use operator_ui_tokens::Theme;
use std::sync::LazyLock;

const TOKENS_CSS: &str = include_str!("../../style/tokens.css");
const BASE_CSS: &str = include_str!("base.css");
const STATE_CSS: &str = include_str!("state.css");
const HOVER_CSS: &str = include_str!("hover.css");

fn stylesheet() -> &'static str {
    static CSS: LazyLock<String> = LazyLock::new(|| {
        [
            TOKENS_CSS,
            BASE_CSS,
            crate::components::primitives::CSS,
            STATE_CSS,
            crate::components::action_button::CSS,
            crate::components::icon_button::CSS,
            crate::components::switch::CSS,
            crate::components::checkbox::CSS,
            crate::components::checkbox_group::CSS,
            crate::components::text_field::CSS,
            crate::components::slider::CSS,
            crate::components::stepper::CSS,
            crate::components::segmented_control::CSS,
            crate::components::dropdown_menu::CSS,
            crate::components::list_item::CSS,
            crate::components::status_badge::CSS,
            crate::components::avatar::CSS,
            crate::components::header::CSS,
            crate::components::console_terminal::CSS,
            crate::components::confirm_dialog::CSS,
            crate::components::marquee_text::CSS,
            crate::components::tooltip::CSS,
            crate::components::theme_toggle::CSS,
            crate::components::toast::CSS,
            crate::components::banner::CSS,
            crate::components::linear_progress::CSS,
            crate::components::skeleton::CSS,
            crate::components::empty_state::CSS,
            crate::components::text_area::CSS,
            crate::components::search_field::CSS,
            crate::components::radio_group::CSS,
            crate::components::combobox::CSS,
            crate::components::chip::CSS,
            crate::components::menu::CSS,
            crate::components::dialog::CSS,
            crate::components::tabs::CSS,
            crate::components::sheet::CSS,
            crate::components::pagination::CSS,
            crate::components::data_table::CSS,
            crate::components::counter::CSS,
            crate::components::property_list::CSS,
            crate::components::accordion::CSS,
            crate::components::code_block::CSS,
            crate::components::kbd::CSS,
            crate::components::toolbar::CSS,
            crate::components::split_pane::CSS,
            crate::components::scroll_area::CSS,
            crate::components::scrollbar::CSS,
            HOVER_CSS,
        ]
        .concat()
    });
    &CSS
}

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
            {inject.then(|| view! { <style>{stylesheet()}</style> })}
            {children()}
        </div>
    }
}
