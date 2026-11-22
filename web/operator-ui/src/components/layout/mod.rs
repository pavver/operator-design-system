//! Application shell layout: header, footer, drawers, PageSticky, and PageScroller.

use crate::components::icon_button::IconButton;
use crate::core::observers::use_scroll;
use crate::core::{Metric, Slot, root_class, root_style, ui_text};
use leptos::html::Div;
use leptos::prelude::*;
use operator_ui_tokens::icons;

pub(crate) const CSS: &str = include_str!("layout.css");

/// Positioning edge or corner for [`PageSticky`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StickyPosition {
    TopStart,
    TopEnd,
    BottomStart,
    #[default]
    BottomEnd,
    Top,
    Bottom,
    Start,
    End,
}

impl StickyPosition {
    pub fn as_attr(&self) -> &'static str {
        match self {
            Self::TopStart => "top-start",
            Self::TopEnd => "top-end",
            Self::BottomStart => "bottom-start",
            Self::BottomEnd => "bottom-end",
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Start => "start",
            Self::End => "end",
        }
    }
}

/// Context provided by [`Layout`] to allow child components to query or scroll the container.
#[derive(Clone, Copy)]
pub struct LayoutContext {
    pub page_ref: NodeRef<Div>,
}

/// Main application shell providing structured header, footer, start and end drawer regions.
#[component]
pub fn Layout(
    #[prop(optional, into)] container: Signal<bool>,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] footer: Slot,
    #[prop(optional, into)] start_drawer: Slot,
    #[prop(optional, into)] end_drawer: Slot,
    #[prop(default = true.into(), into)] start_drawer_open: Signal<bool>,
    #[prop(default = true.into(), into)] end_drawer_open: Signal<bool>,
    /// Header height in DU (`--op-layout-header-height`).
    #[prop(optional, into)]
    header_height: MaybeProp<f32>,
    /// Footer height in DU (`--op-layout-footer-height`).
    #[prop(optional, into)]
    footer_height: MaybeProp<f32>,
    /// Start drawer width in DU (`--op-layout-start-drawer-width`).
    #[prop(optional, into)]
    start_drawer_width: MaybeProp<f32>,
    /// End drawer width in DU (`--op-layout-end-drawer-width`).
    #[prop(optional, into)]
    end_drawer_width: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let page_ref = NodeRef::<Div>::new();
    provide_context(LayoutContext { page_ref });

    let metrics = vec![
        Metric::du("--op-layout-header-height", header_height),
        Metric::du("--op-layout-footer-height", footer_height),
        Metric::du("--op-layout-start-drawer-width", start_drawer_width),
        Metric::du("--op-layout-end-drawer-width", end_drawer_width),
    ];
    let class_fn = root_class("op-layout", class);
    let style_fn = root_style(metrics, style);

    let has_header = header.is_set();
    let has_footer = footer.is_set();
    let has_start_drawer = start_drawer.is_set();
    let has_end_drawer = end_drawer.is_set();

    view! {
        <div
            class=class_fn
            style=style_fn
            data-container=move || container.get().then_some("")
        >
            {has_header.then(|| view! {
                <header class="op-layout-header" role="banner">
                    {header.run()}
                </header>
            })}
            <div class="op-layout-middle">
                {has_start_drawer.then(|| view! {
                    <aside
                        class="op-layout-start-drawer"
                        role="complementary"
                        data-closed=move || (!start_drawer_open.get()).then_some("")
                    >
                        {start_drawer.run()}
                    </aside>
                })}
                <div class="op-layout-page-container" node_ref=page_ref role="main">
                    {children()}
                </div>
                {has_end_drawer.then(|| view! {
                    <aside
                        class="op-layout-end-drawer"
                        role="complementary"
                        data-closed=move || (!end_drawer_open.get()).then_some("")
                    >
                        {end_drawer.run()}
                    </aside>
                })}
            </div>
            {has_footer.then(|| view! {
                <footer class="op-layout-footer" role="contentinfo">
                    {footer.run()}
                </footer>
            })}
        </div>
    }
}

/// Helper component anchoring content to an edge or corner of the page.
#[component]
pub fn PageSticky(
    #[prop(optional)] position: StickyPosition,
    /// Horizontal offset in DU (`--op-page-sticky-offset-x`).
    #[prop(optional, into)]
    offset_x: MaybeProp<f32>,
    /// Vertical offset in DU (`--op-page-sticky-offset-y`).
    #[prop(optional, into)]
    offset_y: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let metrics = vec![
        Metric::du("--op-page-sticky-offset-x", offset_x),
        Metric::du("--op-page-sticky-offset-y", offset_y),
    ];
    let class_fn = root_class("op-page-sticky", class);
    let style_fn = root_style(metrics, style);

    view! {
        <div
            class=class_fn
            style=style_fn
            data-position=position.as_attr()
        >
            {children()}
        </div>
    }
}

/// Floating action button that appears when scrolled down and scrolls smoothly to the top.
#[component]
pub fn PageScroller(
    /// Scroll offset threshold in DU before showing the scroller.
    #[prop(default = 50.0.into(), into)]
    scroll_offset: MaybeProp<f32>,
    #[prop(optional)] position: StickyPosition,
    #[prop(optional, into)] offset_x: MaybeProp<f32>,
    #[prop(optional, into)] offset_y: MaybeProp<f32>,
    #[prop(optional)] reverse: bool,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let layout_ctx = use_context::<LayoutContext>();
    let target_ref = layout_ctx.map(|c| c.page_ref);
    let scroll = use_scroll(target_ref);

    let default_label = if reverse {
        "layout.scroll-bottom"
    } else {
        "layout.scroll-top"
    };
    let default_text = if reverse {
        "Scroll to bottom"
    } else {
        "Scroll to top"
    };
    let label = ui_text(aria_label, default_label, default_text);

    let is_visible = Signal::derive(move || {
        let du_threshold = scroll_offset.get().unwrap_or(50.0);
        let px_threshold = du_threshold * 4.0;
        let s = scroll.get();
        if reverse {
            let max_scroll = (s.scroll_height - s.client_height).max(0.0);
            (max_scroll - s.scroll_top) > px_threshold as f64
        } else {
            s.scroll_top > px_threshold as f64
        }
    });

    let on_click = move |_| {
        if let Some(ctx) = layout_ctx
            && let Some(el) = ctx.page_ref.get()
        {
            let target_top = if reverse { el.scroll_height() } else { 0 };
            el.set_scroll_top(target_top);
            return;
        }
        if let Some(win) = web_sys::window() {
            let target_top = if reverse {
                win.document()
                    .and_then(|d| d.document_element())
                    .map(|e| e.scroll_height() as f64)
                    .unwrap_or(10000.0)
            } else {
                0.0
            };
            win.scroll_to_with_x_and_y(0.0, target_top);
        }
    };

    let icon = if reverse {
        icons::ARROW_DOWNWARD
    } else {
        icons::ARROW_UPWARD
    };

    let metrics = vec![
        Metric::du("--op-page-sticky-offset-x", offset_x),
        Metric::du("--op-page-sticky-offset-y", offset_y),
    ];
    let class_fn = root_class("op-page-sticky op-page-scroller", class);
    let style_fn = root_style(metrics, style);

    view! {
        <div
            class=class_fn
            style=style_fn
            data-position=position.as_attr()
            data-hidden=move || (!is_visible.get()).then_some("")
            on:click=on_click
        >
            {match children {
                Some(c) => c().into_any(),
                None => view! {
                    <IconButton
                        icon=icon
                        label=label.get_untracked()
                    />
                }.into_any(),
            }}
        </div>
    }
}
