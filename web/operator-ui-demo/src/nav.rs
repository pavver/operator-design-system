//! Sidebar order and page neighbours.

/// Sidebar groups and the documents they list, in order.
pub const NAV: &[(&str, &[&str])] = &[
    (
        "Getting started",
        &[
            "platforms/web",
            "tokens",
            "porting",
            "releasing",
            "changes-to-review",
        ],
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
            "components/fab",
            "components/split-button",
            "components/icon-button",
            "components/menu",
            "components/toolbar",
            "components/button-group",
            "components/launcher-tile",
            "components/ptz-pad",
            "components/media-record-button",
            "components/ptt-button",
            "components/pull-to-refresh",
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
            "components/knob",
            "components/range-slider",
            "components/rating",
            "components/stepper",
            "components/chip",
            "components/theme-toggle",
            "components/emoji-picker",
            "components/sticker-picker",
            "components/gif-picker",
            "components/reaction-picker",
        ],
    ),
    (
        "Text input",
        &[
            "components/text-field",
            "components/editor",
            "components/file-field",
            "components/form",
            "components/uploader",
            "components/text-area",
            "components/search-field",
            "components/field",
            "components/dropdown-menu",
            "components/combobox",
            "components/language-picker",
            "components/value-fields",
            "components/chat-input",
            "components/media-keyboard",
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
            "components/circular-progress",
            "components/loading-overlay",
            "components/loading-bar",
        ],
    ),
    (
        "Overlays",
        &[
            "components/dialog",
            "components/confirm-dialog",
            "components/sheet",
            "components/popover",
            "components/popup-edit",
            "components/hover-card",
            "components/gallery-viewer",
        ],
    ),
    (
        "Navigation",
        &[
            "components/header",
            "components/tabs",
            "components/tab-panels",
            "components/pagination",
            "components/navigation-rail",
            "components/side-panel",
            "components/breadcrumbs",
            "components/wizard",
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
            "components/timeline",
            "components/chat-message",
            "components/image",
            "components/video",
            "components/gallery",
            "components/video-stream",
            "components/carousel",
            "components/swipe-item",
        ],
    ),
    (
        "Layout",
        &[
            "components/layout",
            "components/split-pane",
            "components/scroll-area",
            "components/scrollbar",
            "components/virtual-list",
            "components/infinite-scroll",
            "components/parallax",
            "components/space",
            "components/aspect-ratio",
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
