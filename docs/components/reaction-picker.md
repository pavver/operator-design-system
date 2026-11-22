# ReactionPicker

Tactical reaction picker with quick reactions row and expandable category dialog.
It provides instant one-tap reactions as well as a full popover dialog with categories.

It ships in the media extension: the `reaction-picker` feature is in the `media` and `extensions` groups, not in `full`.

Status: **draft** on web.

## Use

Use `ReactionPicker` in chat messages, timeline posts, and collaboration feeds.
The host application passes the quick reactions strip and categorized reaction sections.
Tap any quick reaction to dispatch immediately, or click the expand button to view all categories.

## Geometry

| Property | Value |
| :--- | :--- |
| Corner radius | `0 DU` (standard square geometry). |
| Standard border | `0.25 DU` dialog outline border. |
| Dialog width | `85 DU` (`--op-reaction-picker-width`). |
| Dialog max height | `110 DU` (`--op-reaction-picker-max-height`). |
| Cell size | `9 DU` (`--op-reaction-picker-cell-size`). |

## Behavior

* Clicking any quick reaction fires `on_select` and leaves the full dialog closed.
* Clicking the expand button toggles the full modal category dialog.
* In the full dialog, tabs switch between app-supplied reaction sections.
* Picking an emoji in the dialog dispatches `on_select` and closes the dialog.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `on_select` | `Callback<String>` | none | Event reporting the chosen emoji. |
| `quick_reactions` | `Option<Vec<String>>` | none | Quick emojis row provided by host app. |
| `categories` | `Option<Vec<ReactionSection>>` | none | Categorized reaction sections from host app. |
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
| `--op-reaction-picker-bg` | `var(--op-surface)` | Dialog background. |
| `--op-reaction-picker-border` | `var(--op-border)` | Dialog border. |
| `--op-reaction-picker-accent` | `var(--op-accent)` | Dialog accent border & highlight. |
| `--op-reaction-picker-text` | `var(--op-text-primary)` | Dialog text and emoji color. |

## Strings

| Key | Default English | Description |
| :--- | :--- | :--- |
| `reaction-picker.title` | `"SELECT REACTION"` | Modal dialog header. |
| `reaction-picker.quick-reactions` | `"Quick reactions"` | Accessible label for quick bar. |
| `reaction-picker.more` | `"More reactions"` | Accessible label for expand button. |
| `reaction-picker.close` | `"Close"` | Accessible label for close button. |
| `reaction-picker.empty` | `"No reactions available"` | Empty state text. |

## Icons

| Key | Default | Description |
| :--- | :--- | :--- |
| `reaction-picker.close-icon` | `icons::CLOSE` | Close button icon. |
| `reaction-picker.expand-icon` | `icons::ADD` | Quick strip expand icon. |

## Accessibility

* Quick buttons and grid cells have accessible emoji names via `aria-label`.
* The full category dialog carries `role="dialog"` and `aria-modal="true"`.
