# Breadcrumbs

Navigation path trail indicating the current page location within a hierarchy.
Long paths collapse intermediate steps into an overflow menu.

Status: **draft** on web.

## Use

Use `Breadcrumbs` near the top of pages that have deep hierarchical structure (such as file trees, settings sections, or multi-level catalogs).
Do not use `Breadcrumbs` for flat navigation or linear step sequences; use [Stepper](stepper.md) or [Tabs](tabs.md) instead.

## Geometry

| Property | Value |
| :--- | :--- |
| Font size | Inherited or `--op-breadcrumbs-font-size`. |
| Gap | `1 DU` between items and separators (`--op-breadcrumbs-gap`). |

## Behavior

* Items before the current page render as clickable navigation links or buttons.
* The current (last) item renders with `aria-current="page"` in the primary text color.
* When `max_items` is set and the item count exceeds it, middle items collapse into an overflow button that reveals them in a [Menu](menu.md).

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `items` | signal `Vec<BreadcrumbItem>` | required | Path items from root to current page. |
| `max_items` | usize | none | Maximum visible items before collapsing middle items. |
| `separator` | string | `"/"` | Text separator between items. |
| `separator_icon` | Icon | none | Icon separator between items (overrides text). |
| `separator_slot` | Slot | none | Custom separator slot. |
| `label` | string | `breadcrumbs.label` | Accessible landmark label. |
| `more_label` | string | `breadcrumbs.more` | Accessible label for the collapse menu button. |
| `font_size` | TU | `--op-breadcrumbs-font-size` | Text size. |
| `gap` | DU | `--op-breadcrumbs-gap` | Spacing between items. |
| `class` | string | none | CSS classes on the root element. |
| `style` | string | none | Inline CSS on the root element. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `font_size` | TU | `--op-breadcrumbs-font-size` | `inherit` |
| `gap` | DU | `--op-breadcrumbs-gap` | `1 DU` |

| Color variable | Default |
| :--- | :--- |
| `--op-breadcrumbs-text` | `var(--op-text-secondary)` |
| `--op-breadcrumbs-active-text` | `var(--op-text-primary)` |
| `--op-breadcrumbs-separator-color` | `var(--op-text-secondary)` |

`class` and `style` apply to the root `<nav>`.

### Strings

| Key | Default |
| :--- | :--- |
| `breadcrumbs.label` | `"Breadcrumbs"` |
| `breadcrumbs.more` | `"More pages"` |

### Icons

| Key | Default |
| :--- | :--- |
| Collapse menu button | `icons::MORE_VERT` |

## Accessibility

`Breadcrumbs` renders as a `<nav>` landmark with an accessible label (`aria-label`).
Items are organized in an ordered list (`<ol>`), and the current page is marked with `aria-current="page"`.
Separators are hidden from assistive technologies with `aria-hidden="true"`.
