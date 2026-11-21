# GifPicker

Looping video clip picker with trending search tags and automated viewport pause management.
GIF items are rendered as short looping MP4/WebM video clips and paused when scrolled out of view via `IntersectionObserver`.

Status: **draft** on web.

## Use

Use `GifPicker` in chat media keyboards, reaction composers, and messaging panels.
Real `.gif` files are slow to decode and CPU intensive; modern clients stream muted looping video clips instead.
The picker automatically pauses clips that leave the visible scrolling viewport to preserve device battery and CPU.

## Geometry

| Property | Value |
| :--- | :--- |
| Corner radius | `0 DU` (standard square geometry). |
| Standard border | `0.25 DU` container outline border. |
| Default height | `60 DU` (`--op-gif-picker-height`). |
| Cell height | `30 DU` (`--op-gif-picker-cell-height`). |

## Behavior

* Clicking a trending tag filters the grid by that thematic keyword.
* Searching via the input field queries both titles and tags simultaneously.
* An `IntersectionObserver` detects when video elements leave the viewport and pauses them.
* When a cell enters the viewport, playback resumes smoothly.
* Clicking a cell selects the item via the `on_select` callback.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `on_select` | `Callback<GifItem>` | none | Event reporting the chosen GIF item. |
| `gifs` | `Option<Vec<GifItem>>` | none | Curated collection of looping GIF clips. |
| `trending_tags` | `Option<Vec<String>>` | none | List of quick trending tag buttons. |
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
| `--op-gif-picker-bg` | `var(--op-color-surface)` | Picker background. |
| `--op-gif-picker-border` | `var(--op-color-border)` | Picker outline border. |
| `--op-gif-picker-accent` | `var(--op-color-accent)` | Active tag and hover border. |

## Strings

| Key | Default English | Description |
| :--- | :--- | :--- |
| `gif-picker.search` | `"Search GIFs..."` | Search input placeholder. |

## Icons

| Key | Default | Description |
| :--- | :--- | :--- |
| `gif-picker.search-icon` | `icons::SEARCH` | Search field icon. |

## Accessibility

* Video clips are automatically muted (`muted`) and looping (`loop playsinline`).
* Every cell includes an accessible text label indicating the clip title.
