//! Navigation breadcrumbs: Breadcrumbs and BreadcrumbItem.

use crate::components::icon_button::IconButton;
use crate::components::menu::{Menu, MenuAnchor, MenuItem};
use crate::components::primitives::Icon;
use crate::core::{Metric, Slot, root_class, root_style, ui_text};
use leptos::html;
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, icons};

pub(crate) const CSS: &str = include_str!("breadcrumbs.css");

/// One link or milestone in a [`Breadcrumbs`] trail.
#[derive(Clone, Debug)]
pub struct BreadcrumbItem {
    pub label: String,
    pub href: Option<String>,
    pub icon: Option<IconData>,
    pub on_click: Option<Callback<()>>,
}

impl BreadcrumbItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: None,
            icon: None,
            on_click: None,
        }
    }

    pub fn href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    pub fn icon(mut self, icon: IconData) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn on_click(mut self, on_click: Callback<()>) -> Self {
        self.on_click = Some(on_click);
        self
    }
}

/// Navigation path trail with separators, collapsing intermediate items into a menu when too long.
#[component]
pub fn Breadcrumbs(
    /// Sequence of path items from the root to the current page.
    #[prop(into)]
    items: Signal<Vec<BreadcrumbItem>>,
    /// Maximum number of visible items before collapsing middle items into a menu.
    #[prop(optional, into)]
    max_items: MaybeProp<usize>,
    /// Text separator between items; defaults to `"/"`.
    #[prop(optional, into)]
    separator: MaybeProp<String>,
    /// Icon separator between items (overrides text separator).
    #[prop(optional)]
    separator_icon: Option<IconData>,
    /// Custom separator slot.
    #[prop(optional, into)]
    separator_slot: Slot,
    /// Accessible landmark label; defaults to `"Breadcrumbs"`.
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// Accessible label for the collapsed menu button; defaults to `"More pages"`.
    #[prop(optional, into)]
    more_label: MaybeProp<String>,
    /// Font size in TU (`--op-breadcrumbs-font-size`).
    #[prop(optional, into)]
    font_size: MaybeProp<f32>,
    /// Gap between items and separators in DU (`--op-breadcrumbs-gap`).
    #[prop(optional, into)]
    gap: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let resolved_label = ui_text(label, "breadcrumbs.label", "Breadcrumbs");
    let resolved_more = ui_text(more_label, "breadcrumbs.more", "More pages");
    let sep_text = move || separator.get().unwrap_or_else(|| "/".to_string());

    let menu_open = RwSignal::new(false);
    let anchor_pos = RwSignal::new(MenuAnchor::Point(0.0, 0.0));
    let more_btn_ref = NodeRef::<html::Div>::new();

    let render_separator = Callback::new(move |()| {
        if separator_slot.is_set() {
            separator_slot.run()
        } else if let Some(icon) = separator_icon {
            Some(view! { <Icon icon=icon size=4.0 /> }.into_any())
        } else {
            Some(view! { <span>{sep_text()}</span> }.into_any())
        }
    });

    let on_toggle_menu = Callback::new(move |()| {
        if let Some(el) = more_btn_ref.get() {
            let rect = el.get_bounding_client_rect();
            anchor_pos.set(MenuAnchor::Below(
                rect.left(),
                rect.top(),
                rect.width(),
                rect.height(),
            ));
        }
        menu_open.update(|v| *v = !*v);
    });

    view! {
        <nav
            class=root_class("op-breadcrumbs", class)
            aria-label=move || resolved_label.get()
            style=root_style(
                vec![
                    Metric::tu("--op-breadcrumbs-font-size", font_size),
                    Metric::du("--op-breadcrumbs-gap", gap),
                ],
                style,
            )
        >
            <ol class="op-breadcrumbs-list">
                {move || {
                    let all_items = items.get();
                    let total = all_items.len();
                    if total == 0 {
                        return ().into_any();
                    }

                    let max = max_items.get().unwrap_or(usize::MAX);
                    let should_collapse = total > max && max >= 2 && total > 2;

                    if should_collapse {
                        let first = &all_items[0];
                        let last = &all_items[total - 1];
                        let collapsed_items = all_items[1..total - 1].to_vec();

                        let menu_items: Vec<MenuItem> = collapsed_items
                            .iter()
                            .enumerate()
                            .map(|(idx, item)| {
                                let mut m = MenuItem::new(format!("crumb-{idx}"), item.label.clone());
                                if let Some(ic) = item.icon {
                                    m = m.icon(ic);
                                }
                                m
                            })
                            .collect();

                        let collapsed_clone = collapsed_items.clone();
                        let on_select = Callback::new(move |id: String| {
                            if let Some(item) = id
                                .strip_prefix("crumb-")
                                .and_then(|s| s.parse::<usize>().ok())
                                .and_then(|idx| collapsed_clone.get(idx))
                            {
                                if let Some(href) = &item.href {
                                    if let Some(w) = web_sys::window() {
                                        let _ = w.location().set_href(href);
                                    }
                                } else if let Some(cb) = &item.on_click {
                                    cb.run(());
                                }
                            }
                        });

                        view! {
                            <li class="op-breadcrumbs-item">
                                {first.icon.map(|ic| view! { <Icon icon=ic size=4.0 /> })}
                                {if let Some(href) = &first.href {
                                    view! { <a class="op-breadcrumbs-link" href=href.clone()>{first.label.clone()}</a> }.into_any()
                                } else if let Some(cb) = first.on_click {
                                    view! { <button type="button" class="op-breadcrumbs-link" on:click=move |_| cb.run(())>{first.label.clone()}</button> }.into_any()
                                } else {
                                    view! { <span class="op-breadcrumbs-text">{first.label.clone()}</span> }.into_any()
                                }}
                            </li>
                            <li class="op-breadcrumbs-sep" aria-hidden="true">{render_separator.run(())}</li>
                            <li class="op-breadcrumbs-item">
                                <div node_ref=more_btn_ref class="op-breadcrumbs-more-host">
                                    <IconButton
                                        icon=icons::MORE_VERT
                                        label=resolved_more.get()
                                        on_click=on_toggle_menu
                                    />
                                </div>
                                <Menu
                                    open=menu_open
                                    anchor=anchor_pos.into()
                                    items=menu_items
                                    on_select=on_select
                                />
                            </li>
                            <li class="op-breadcrumbs-sep" aria-hidden="true">{render_separator.run(())}</li>
                            <li class="op-breadcrumbs-item op-breadcrumbs-current" aria-current="page">
                                {last.icon.map(|ic| view! { <Icon icon=ic size=4.0 /> })}
                                <span class="op-breadcrumbs-text">{last.label.clone()}</span>
                            </li>
                        }.into_any()
                    } else {
                        all_items.into_iter().enumerate().map(|(idx, item)| {
                            let is_last = idx + 1 == total;
                            let label_text = item.label.clone();
                            let href_val = item.href.clone();
                            let click_cb = item.on_click;
                            let icon = item.icon;
                            view! {
                                <li
                                    class=if is_last { "op-breadcrumbs-item op-breadcrumbs-current" } else { "op-breadcrumbs-item" }
                                    aria-current=is_last.then_some("page")
                                >
                                    {icon.map(|ic| view! { <Icon icon=ic size=4.0 /> })}
                                    {if is_last {
                                        view! { <span class="op-breadcrumbs-text">{label_text}</span> }.into_any()
                                    } else if let Some(href) = href_val {
                                        view! { <a class="op-breadcrumbs-link" href=href>{label_text}</a> }.into_any()
                                    } else if let Some(cb) = click_cb {
                                        view! { <button type="button" class="op-breadcrumbs-link" on:click=move |_| cb.run(())>{label_text}</button> }.into_any()
                                    } else {
                                        view! { <span class="op-breadcrumbs-text">{label_text}</span> }.into_any()
                                    }}
                                </li>
                                {(!is_last).then(|| view! {
                                    <li class="op-breadcrumbs-sep" aria-hidden="true">{render_separator.run(())}</li>
                                })}
                            }
                        }).collect_view().into_any()
                    }
                }}
            </ol>
        </nav>
    }
}
