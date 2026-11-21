# StickerPicker

Pluggable vector and animated sticker picker with pack navigation and preview support.
Supports static PNG/WebP graphics, vector SVGs, and animated vector formats.

Status: **draft** on web.

## Use

Use `StickerPicker` in chat composer panels, media keyboards, and comment forms.
The library ships no sticker packs: the application installs packs through `StickerPack` models.
Long-press or hover displays an enlarged preview of the sticker graphic.

## Geometry

| Property | Value |
| :--- | :--- |
| Corner radius | `0 DU` (standard square geometry). |
| Standard border | `0.25 DU` outline border. |
| Container height | `60 DU` (`--op-sticker-picker-height`). |
| Cell size | `18 DU` (`--op-sticker-picker-cell-size`). |

## Behavior

* Clicking a pack tab scrolls to that pack's sticker list.
* Typing into the search field queries stickers across installed packs by name or emoji.
* Selecting a sticker fires `on_select` with a `StickerAttachment`.
* Hovering over any sticker displays an enlarged preview overlay.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `on_select` | `Callback<StickerAttachment>` | none | Event reporting the selected sticker attachment. |
| `sticker_packs` | `Option<Vec<StickerPack>>` | none | Installed sticker packs with icons and metadata. |
| `recent_stickers` | `Option<Vec<StickerItem>>` | none | Recently used stickers history. |
| `render_sticker` | `Option<Callback<StickerAttachment, AnyView>>` | none | Custom renderer slot for animated/vector formats. |
| `height` | `f32` | `60.0` | Picker height in Design Units. |
| `cell_size` | `f32` | `18.0` | Sticker cell size in Design Units. |
| `class` | `string` | none | Custom CSS classes. |
| `style` | `string` | none | Custom inline style overrides. |

## Customization

### Metric props

| Prop | CSS variable | Default | Unit |
| :--- | :--- | :--- | :--- |
| `height` | `--op-sticker-picker-height` | `60.0` | DU |
| `cell_size` | `--op-sticker-picker-cell-size` | `18.0` | DU |

### Color variables

| Variable | Default | Role |
| :--- | :--- | :--- |
| `--op-sticker-picker-bg` | `var(--op-surface)` | Picker background. |
| `--op-sticker-picker-border` | `var(--op-border)` | Picker outline border. |
| `--op-sticker-picker-accent` | `var(--op-accent)` | Pack tab highlight and cell focus. |
| `--op-sticker-picker-text` | `var(--op-text-primary)` | Picker text color. |

## Strings

| Key | Default English | Description |
| :--- | :--- | :--- |
| `sticker-picker.search` | `"Search stickers..."` | Search field placeholder. |
| `sticker-picker.recent` | `"Recent stickers"` | Recent stickers section and tab label. |
| `sticker-picker.packs` | `"Sticker packs"` | Sticker packs tablist accessible label. |
| `sticker-picker.search-prefix` | `"Search results"` | Search results header title prefix. |
| `sticker-picker.empty` | `"No stickers found"` | Empty search or pack state. |

## Icons

| Key | Default | Description |
| :--- | :--- | :--- |
| `sticker-picker.search-icon` | `icons::SEARCH` | Search field icon. |
| `sticker-picker.recent-icon` | `icons::REFRESH` | Recent stickers tab icon. |

## Accessibility

* Pack tabs have accessible names from pack titles and carry `role="tab"`.
* Each sticker cell carries an accessible description matching its title and emoji.
