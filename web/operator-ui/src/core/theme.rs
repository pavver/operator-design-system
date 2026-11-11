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
            #[cfg(feature = "primitives")]
            crate::components::primitives::CSS,
            STATE_CSS,
            #[cfg(feature = "action-button")]
            crate::components::action_button::CSS,
            #[cfg(feature = "icon-button")]
            crate::components::icon_button::CSS,
            #[cfg(feature = "switch")]
            crate::components::switch::CSS,
            #[cfg(feature = "checkbox")]
            crate::components::checkbox::CSS,
            #[cfg(feature = "checkbox-group")]
            crate::components::checkbox_group::CSS,
            #[cfg(feature = "text-field")]
            crate::components::text_field::CSS,
            #[cfg(feature = "slider")]
            crate::components::slider::CSS,
            #[cfg(feature = "stepper")]
            crate::components::stepper::CSS,
            #[cfg(feature = "segmented-control")]
            crate::components::segmented_control::CSS,
            #[cfg(feature = "dropdown-menu")]
            crate::components::dropdown_menu::CSS,
            #[cfg(feature = "list-item")]
            crate::components::list_item::CSS,
            #[cfg(feature = "status-badge")]
            crate::components::status_badge::CSS,
            #[cfg(feature = "avatar")]
            crate::components::avatar::CSS,
            #[cfg(feature = "header")]
            crate::components::header::CSS,
            #[cfg(feature = "console-terminal")]
            crate::components::console_terminal::CSS,
            #[cfg(feature = "confirm-dialog")]
            crate::components::confirm_dialog::CSS,
            #[cfg(feature = "marquee-text")]
            crate::components::marquee_text::CSS,
            #[cfg(feature = "tooltip")]
            crate::components::tooltip::CSS,
            #[cfg(feature = "theme-toggle")]
            crate::components::theme_toggle::CSS,
            #[cfg(feature = "toast")]
            crate::components::toast::CSS,
            #[cfg(feature = "banner")]
            crate::components::banner::CSS,
            #[cfg(feature = "linear-progress")]
            crate::components::linear_progress::CSS,
            #[cfg(feature = "skeleton")]
            crate::components::skeleton::CSS,
            #[cfg(feature = "empty-state")]
            crate::components::empty_state::CSS,
            #[cfg(feature = "text-area")]
            crate::components::text_area::CSS,
            #[cfg(feature = "search-field")]
            crate::components::search_field::CSS,
            #[cfg(feature = "radio-group")]
            crate::components::radio_group::CSS,
            #[cfg(feature = "combobox")]
            crate::components::combobox::CSS,
            #[cfg(feature = "chip")]
            crate::components::chip::CSS,
            #[cfg(feature = "menu")]
            crate::components::menu::CSS,
            #[cfg(feature = "dialog")]
            crate::components::dialog::CSS,
            #[cfg(feature = "tabs")]
            crate::components::tabs::CSS,
            #[cfg(feature = "sheet")]
            crate::components::sheet::CSS,
            #[cfg(feature = "pagination")]
            crate::components::pagination::CSS,
            #[cfg(feature = "data-table")]
            crate::components::data_table::CSS,
            #[cfg(feature = "counter")]
            crate::components::counter::CSS,
            #[cfg(feature = "property-list")]
            crate::components::property_list::CSS,
            #[cfg(feature = "accordion")]
            crate::components::accordion::CSS,
            #[cfg(feature = "code-block")]
            crate::components::code_block::CSS,
            #[cfg(feature = "kbd")]
            crate::components::kbd::CSS,
            #[cfg(feature = "toolbar")]
            crate::components::toolbar::CSS,
            #[cfg(feature = "split-pane")]
            crate::components::split_pane::CSS,
            #[cfg(feature = "scroll-area")]
            crate::components::scroll_area::CSS,
            #[cfg(feature = "scrollbar")]
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
