# GifPicker

Looping animated GIF and video clip picker with IntersectionObserver viewport playback.
GIFs are delivered as looping MP4/WebM videos and automatically paused when scrolled outside the viewport.

Status: **draft** on web.

## Use

Use `GifPicker` in chat inputs, social reaction sheets, and comment dialogs.
Clips and trending tags are provided entirely by the host application.
The component renders high-efficiency muted looping videos instead of legacy high-overhead `.gif` bitmaps.

## Geometry

| Property | Value |
| :--- | :--- |
| Corner radius | `0 DU` (standard square geometry). |
| Standard border | `0.25 DU` outline border. |
| Container height | `60 DU` (`--op-gif-picker-height`). |
| Cell height | `30 DU` (`--op-gif-picker-cell-height`). |

## Behavior

* Videos auto-play muted in a loop when intersecting the visible viewport.
* An `IntersectionObserver` pauses off-screen videos to minimize CPU and power usage.
* Clicking any tag filters the list by the tag key; clicking it again clears the tag filter.
* Selecting a clip emits `on_select` with the `GifItem`.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `on_select` | `Callback<GifItem>` | none | Event reporting the chosen GIF item. |
| `gifs` | `Option<Vec<GifItem>>` | none | Curated collection of looping GIF clips from host app. |
| `trending_tags` | `Option<Vec<String>>` | none | List of quick trending tag buttons from host app. |
| `height` | `f32` | `60.0` | Container height in Design Units. |
| `cell_height` | `f32` | `30.0` | Cell height in Design Units. |
| `class` | `string` | none | Custom CSS classes. |
| `style` | `string` | none | Custom inline style overrides. |

## Customization

### Metric props

| Prop | CSS variable | Default | Unit |
| :--- | :--- | :--- | :--- |
| `height` | `--op-gif-picker-height` | `60.0` | DU |
| `cell_height` | `--op-gif-picker-cell-height` | `30.0` | DU |

### Color variables

| Variable | Default | Role |
| :--- | :--- | :--- |
| `--op-gif-picker-bg` | `var(--op-surface)` | Picker background. |
| `--op-gif-picker-border` | `var(--op-border)` | Picker outline border. |
| `--op-gif-picker-accent` | `var(--op-accent)` | Active tag and hover border. |
| `--op-gif-picker-text` | `var(--op-text-primary)` | Text and title bar color. |

## Strings

| Key | Default English | Description |
| :--- | :--- | :--- |
| `gif-picker.search` | `"Search GIFs..."` | Search input placeholder. |
| `gif-picker.tags` | `"Trending GIF tags"` | Accessible label for tag toolbar. |
| `gif-picker.empty` | `"No GIFs found"` | Empty search or tag state text. |

## Icons

| Key | Default | Description |
| :--- | :--- | :--- |
| `gif-picker.search-icon` | `icons::SEARCH` | Search field icon. |

## Accessibility

* Video clips are automatically muted (`muted`) and looping (`loop playsinline`).
* Every cell includes an accessible text label indicating the clip title.
