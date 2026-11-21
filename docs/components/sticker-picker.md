# StickerPicker

Pluggable sticker picker with pack navigation strips, search query filtering, and enlarged hover previews.
It supports raster pictures, vector SVGs, and application-rendered animated formats.

Status: **draft** on web.

## Use

Use `StickerPicker` in media keyboards, chat input panels, and messaging interfaces.
The component ships no built-in packs: all packs, metadata, and graphics are installed by the host application.
Long-pressing or hovering over a sticker displays an enlarged floating preview overlay.

## Geometry

| Property | Value |
| :--- | :--- |
| Corner radius | `0 DU` (standard square geometry). |
| Standard border | `0.25 DU` container outline border. |
| Default height | `60 DU` (`--op-sticker-picker-height`). |
| Cell size | `18 DU` (`--op-sticker-picker-cell-size`). |

## Behavior

* Clicking a pack icon in the navigation strip switches the visible sticker collection.
* Live search matches against sticker titles and associated emoji keywords.
* Selecting a sticker invokes `on_select` with a polymorphic `StickerAttachment`.
* Hovering over a sticker opens an enlarged floating preview box.

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
| `--op-sticker-picker-bg` | `var(--op-color-surface)` | Picker background. |
| `--op-sticker-picker-border` | `var(--op-color-border)` | Picker outline border. |
| `--op-sticker-picker-accent` | `var(--op-color-accent)` | Pack tab highlight and cell focus. |

## Strings

| Key | Default English | Description |
| :--- | :--- | :--- |
| `sticker-picker.search` | `"Search stickers..."` | Search field placeholder. |

## Icons

| Key | Default | Description |
| :--- | :--- | :--- |
| `sticker-picker.search-icon` | `icons::SEARCH` | Search field icon. |

## Accessibility

* Pack tabs have accessible names from pack titles and carry `role="tab"`.
* Each sticker cell carries an accessible description matching its title and emoji.
