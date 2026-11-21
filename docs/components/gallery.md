# Gallery

Responsive media grid of photos and videos with selection, captions, and viewport-lazy loading via `IntersectionObserver`.

Status: **draft** on web.

## Use

Use `Gallery` when presenting an album, message attachments, or a media library of photos and videos.
Do not use `Gallery` for a single isolated photograph; use [Image](image.md) instead.

## Geometry

| Property | Value |
| :--- | :--- |
| Corner radius | `0 DU` (standard square geometry). |
| Default column width | `30 DU` minimum in responsive mode. |
| Gap | Standard `0.5 DU` gap. |
| Border | Standard `0.25 DU` inside each cell. |

## Behavior

* Images outside the viewport defer downloading until intersecting the viewport using `IntersectionObserver`.
* Supports both fluid responsive CSS grid mode and compact chat-bubble Telegram layout.
* Videos display a play badge indicator over the thumbnail.
* Optional selection mode adds checkboxes to each item and emits selection change events.
* Clicking an item invokes `on_item_click` with the item index.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `items` | signal Vec GalleryMediaItem | required | Collection of media items to display. |
| `layout` | GalleryLayout | `Grid` | Layout mode (`Grid` or `Telegram`). |
| `column_min_width` | DU | `30.0` | Minimum column width in DU for responsive grid. |
| `gap` | DU | `0.5` | Spacing between items in DU. |
| `selectable` | bool | `false` | Enables multi-item selection mode. |
| `selected_ids` | HashSet string | empty | Set of currently selected media IDs. |
| `on_selection_change` | callback HashSet | none | Invoked when selection changes. |
| `on_item_click` | callback usize | none | Invoked when a media item is clicked. |
| `empty_slot` | Slot | none | Custom view when no items exist. |
| `empty_label` | string | `"No media items"` | Translatable label rendered when no items exist. |
| `class` | string | none | CSS classes on the root element. |
| `style` | string | none | Inline CSS on the root element. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `column_min_width` | DU | `--op-gallery-column-min-width` | `30 DU` |
| `gap` | DU | `--op-gallery-gap` | `0.5 DU` |

| Color variable | Default |
| :--- | :--- |
| `--op-gallery-item-bg` | `var(--op-surface)` |
| `--op-gallery-border` | `var(--op-border)` |
| `--op-gallery-item-hover-border` | `var(--op-border-strong)` |

`class` and `style` apply to the root container.

### Strings

| Key | Default |
| :--- | :--- |
| `gallery.empty` | `"No media items"` |

### Icons

| Key | Default |
| :--- | :--- |
| Video play badge | `icons::VIDEOCAM` |

## Accessibility

Each item contains accessible alternative text or title attributes.
Selection checkboxes include accessible labels.
Keyboard focus and clicks operate predictably across all grid cells.
