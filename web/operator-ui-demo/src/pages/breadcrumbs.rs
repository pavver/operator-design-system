//! Live examples for Breadcrumbs.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn breadcrumbs() -> AnyView {
    let standard_items = RwSignal::new(vec![
        BreadcrumbItem::new("Home").href("#/"),
        BreadcrumbItem::new("Components").href("#/components/action-button"),
        BreadcrumbItem::new("Navigation").href("#/components/header"),
        BreadcrumbItem::new("Breadcrumbs"),
    ]);

    let long_items = RwSignal::new(vec![
        BreadcrumbItem::new("Root").href("#/"),
        BreadcrumbItem::new("Workspace").href("#/"),
        BreadcrumbItem::new("Projects").href("#/"),
        BreadcrumbItem::new("Design System").href("#/"),
        BreadcrumbItem::new("Components").href("#/"),
        BreadcrumbItem::new("Current Page"),
    ]);

    view! {
        <Example
            title="Standard trail"
            note="Path with default separators."
            block=true
        >
            <div class="demo-stack demo-narrow">
                <Breadcrumbs items=standard_items />
            </div>
        </Example>
        <Example
            title="Collapsed middle"
            note="Collapses middle items into a menu when exceeding max_items."
            block=true
        >
            <div class="demo-stack demo-narrow">
                <Breadcrumbs
                    items=long_items
                    max_items=3
                />
            </div>
        </Example>
        <Example
            title="Custom icon separator"
            note="Uses a chevron icon separator."
            block=true
        >
            <div class="demo-stack demo-narrow">
                <Breadcrumbs
                    items=standard_items
                    separator_icon=icons::CHEVRON_RIGHT
                />
            </div>
        </Example>
    }
    .into_any()
}
