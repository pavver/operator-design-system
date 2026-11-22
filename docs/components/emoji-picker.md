# EmojiPicker

Pluggable emoji picker with category tabs, live query filtering, and custom emoji packs.
It supports Unicode emojis, application-installed custom graphics via `CustomEmojiMap`, and recent emoji memory.

It ships in the media extension: the `emoji-picker` feature is in the `media` and `extensions` groups, not in `full`.

Status: **draft** on web.

## Use

Use `EmojiPicker` in chat inputs, comment fields, media keyboards, and status selection dialogs.
The component does not bundle heavy static assets: emoji packs and custom emojis are passed by the host application.
Live search filters both standard emojis and custom emoji aliases.

## Geometry

| Property | Value |
| :--- | :--- |
| Corner radius | `0 DU` (standard square geometry). |
| Standard border | `0.25 DU` standard container border. |
| Default height | `60 DU` (`--op-emoji-picker-height`). |
| Cell size | `8.5 DU` (`--op-emoji-picker-cell-size`). |

## Behavior

* Clicking a category tab scrolls the grid and reveals emojis belonging to that category.
* Typing into the search field dynamically filters emojis across all categories.
* Selecting an emoji invokes `on_select` with the Unicode character or custom emoji key.
* The Recent category keeps track of previously dispatched emojis when passed by the application.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `on_select` | `Callback<String>` | none | Event reporting the selected emoji string or code. |
| `categories` | `Option<Vec<EmojiCategory>>` | none | Pluggable emoji categories provided by the host app. |
| `recent_emojis` | `Option<Vec<EmojiItem>>` | none | List of recently used emojis provided by the host app. |
| `custom_emojis` | `Option<CustomEmojiMap>` | none | Registry of custom image-backed emojis. |
| `height` | `f32` | `60.0` | Container height in Design Units. |
| `cell_size` | `f32` | `8.5` | Emoji cell size in Design Units. |
| `class` | `string` | none | Custom CSS classes. |
| `style` | `string` | none | Custom inline style overrides. |

## Customization

### Metric props

| Prop | CSS variable | Default | Unit |
| :--- | :--- | :--- | :--- |
| `height` | `--op-emoji-picker-height` | `60.0` | DU |
| `cell_size` | `--op-emoji-picker-cell-size` | `8.5` | DU |

### Color variables

| Variable | Default | Role |
| :--- | :--- | :--- |
| `--op-emoji-picker-bg` | `var(--op-surface)` | Container background. |
| `--op-emoji-picker-border` | `var(--op-border)` | Container outline border. |
| `--op-emoji-picker-accent` | `var(--op-accent)` | Tab highlight and hover border. |
| `--op-emoji-picker-text` | `var(--op-text-primary)` | Text and emoji fallback color. |

## Strings

| Key | Default English | Description |
| :--- | :--- | :--- |
| `emoji-picker.search` | `"Search emoji..."` | Search field placeholder. |
| `emoji-picker.search-prefix` | `"Search"` | Search results header title prefix. |
| `emoji-picker.recent` | `"Recent"` | Recent section title and tab label. |
| `emoji-picker.categories` | `"Emoji categories"` | Categories tablist accessible label. |
| `emoji-picker.empty` | `"No emoji found"` | Empty search or category state. |

## Icons

| Key | Default | Description |
| :--- | :--- | :--- |
| `emoji-picker.search-icon` | `icons::SEARCH` | Search field icon. |
| `emoji-picker.recent-icon` | `icons::REFRESH` | Recent category tab icon. |

## Accessibility

* Category buttons have explicit `role="tab"` and `aria-selected` attributes.
* Grid cells provide accessible text names for screen readers via `aria-label`.
