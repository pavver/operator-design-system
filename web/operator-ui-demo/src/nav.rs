//! Sidebar order and page neighbours.

/// Sidebar groups and the documents they list, in order.
pub const NAV: &[(&str, &[&str])] = &[
    (
        "Getting started",
        &["platforms/web", "tokens", "porting", "releasing"],
    ),
    (
        "Foundations",
        &[
            "foundations/principles",
            "foundations/units",
            "foundations/color",
            "foundations/typography",
            "foundations/iconography",
            "foundations/motion",
            "foundations/states",
            "foundations/customization",
        ],
    ),
    (
        "Actions",
        &[
            "components/action-button",
            "components/icon-button",
            "components/menu",
            "components/toolbar",
        ],
    ),
    (
        "Selection",
        &[
            "components/switch",
            "components/checkbox",
            "components/checkbox-group",
            "components/radio-group",
            "components/segmented-control",
            "components/slider",
            "components/stepper",
            "components/chip",
            "components/theme-toggle",
        ],
    ),
    (
        "Text input",
        &[
            "components/text-field",
            "components/text-area",
            "components/search-field",
            "components/field",
            "components/dropdown-menu",
            "components/combobox",
            "components/language-picker",
            "components/value-fields",
        ],
    ),
    (
        "Feedback",
        &[
            "components/tooltip",
            "components/toast",
            "components/banner",
            "components/linear-progress",
            "components/skeleton",
            "components/empty-state",
            "components/status-badge",
        ],
    ),
    (
        "Overlays",
        &[
            "components/dialog",
            "components/confirm-dialog",
            "components/sheet",
            "components/popover",
            "components/hover-card",
        ],
    ),
    (
        "Navigation",
        &[
            "components/header",
            "components/tabs",
            "components/pagination",
            "components/navigation-rail",
            "components/side-panel",
        ],
    ),
    (
        "Data display",
        &[
            "components/list-item",
            "components/tree",
            "components/data-table",
            "components/property-list",
            "components/accordion",
            "components/counter",
            "components/avatar",
            "components/code-block",
            "components/kbd",
            "components/console-terminal",
            "components/marquee-text",
            "components/primitives",
        ],
    ),
    (
        "Layout",
        &[
            "components/split-pane",
            "components/scroll-area",
            "components/scrollbar",
            "components/virtual-list",
        ],
    ),
    ("Project extensions", &["components/graph"]),
];

pub fn group_of(slug: &str) -> Option<&'static str> {
    NAV.iter()
        .find(|(_, items)| items.contains(&slug))
        .map(|(g, _)| *g)
}

/// Neighbouring pages in sidebar order.
pub fn neighbours(slug: &str) -> (Option<&'static str>, Option<&'static str>) {
    let all: Vec<&'static str> = NAV.iter().flat_map(|(_, i)| i.iter().copied()).collect();
    let Some(i) = all.iter().position(|s| *s == slug) else {
        return (None, None);
    };
    (
        i.checked_sub(1).and_then(|p| all.get(p).copied()),
        all.get(i + 1).copied(),
    )
}
