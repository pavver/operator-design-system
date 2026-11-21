# ReactionPicker

Tactical reaction picker for chat messages, alerts, and feedback channels.
It combines a quick reactions strip with an expandable full modal category selector.

Status: **draft** on web.

## Use

Use `ReactionPicker` in conversational feeds, message action popovers, and post reaction bars.
Users can either select an immediate top reaction from the quick strip or expand the comprehensive dialog.
Categories filter between Popular, Smileys, Gestures, Symbols, and Tactical objects.

## Geometry

| Property | Value |
| :--- | :--- |
| Corner radius | `0 DU` (standard square geometry). |
| Standard border | `0.25 DU` standard outline border. |
| Dialog width | `85 DU` (`--op-reaction-picker-width`). |
| Dialog max height | `110 DU` (`--op-reaction-picker-max-height`). |
| Emoji cell size | `9 DU` (`--op-reaction-picker-cell-size`). |

## Behavior

* Clicking an emoji in either the quick strip or the modal selects the reaction via `on_select`.
* The '+' button in the quick strip expands the full categorized reaction picker.
* Switching category tabs re-renders the high-density grid for that category.
* Clicking outside the dialog or pressing the close button dismisses the overlay.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `on_select` | `Callback<String>` | none | Event reporting the chosen emoji. |
| `quick_reactions` | `Option<Vec<String>>` | `[👍, 👎, ❤️, 🔥, 👏, 🎉, ⚡]` | Emojis in the quick row. |
| `show_quick_bar` | `bool` | `true` | Whether to display the quick reactions strip. |
| `open` | `bool` | `false` | Controls visibility of full dialog. |
| `on_close` | `Callback<()>` | none | Event when dialog is dismissed. |
| `width` | `f32` | `85.0` | Dialog width in Design Units. |
| `max_height` | `f32` | `110.0` | Dialog max height in Design Units. |
| `class` | `string` | none | Custom CSS classes. |
| `style` | `string` | none | Custom inline style overrides. |

## Customization

### Metric props

| Prop | CSS variable | Default | Unit |
| :--- | :--- | :--- | :--- |
| `width` | `--op-reaction-picker-width` | `85.0` | DU |
| `max_height` | `--op-reaction-picker-max-height` | `110.0` | DU |

### Color variables

| Variable | Default | Role |
| :--- | :--- | :--- |
| `--op-reaction-picker-bg` | `var(--op-color-surface)` | Dialog background. |
| `--op-reaction-picker-border` | `var(--op-color-border)` | Dialog border. |
| `--op-reaction-picker-accent` | `var(--op-color-accent)` | Dialog accent border & highlight. |

## Strings

| Key | Default English | Description |
| :--- | :--- | :--- |
| `reaction-picker.title` | `"SELECT REACTION"` | Modal dialog header. |

## Icons

| Key | Default | Description |
| :--- | :--- | :--- |
| `reaction-picker.close` | `icons::CLOSE` | Close button icon. |
| `reaction-picker.expand` | `icons::ADD` | Quick strip expand icon. |

## Accessibility

* Quick buttons and grid cells have accessible emoji names via `aria-label`.
* The full category dialog carries `role="dialog"` and `aria-modal="true"`.
