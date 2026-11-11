# Banner

Persistent message inside a screen or section about its current state.
It stays until the condition is gone or the user dismisses it.

Status: **reviewed** on web.

## Use

* A state that affects the whole view: offline mode, full storage, a pending firmware update.
* Prefer a [Toast](toast.md) for events and a [Dialog](dialog.md) for decisions that block work.

## Geometry

| Property | Value |
| :--- | :--- |
| Width | Full width of its container. |
| Padding | `3 DU`. |
| Background | Tone color at 15% over `surface`. |
| Start bar | `0.5 DU` in the tone color. |
| Icon | `8 DU`, tone color, `2 DU` from the text; the first text line is centered on it. |
| Title | Body style, bold. |
| Actions | Under the text, `8 DU` tall [ActionButtons](action-button.md), `2 DU` apart. |
| Action colors | Transparent background, tone-colored label and icon, outline in the tone at 50%; hover adds a 12% tone tint and a full tone outline, press 25%. Danger buttons keep their own colors. |
| Close | `8 DU` square button at the top end. |

The tones and their icons are the same as in [Toast](toast.md).

## Behavior

* The banner does not disappear on its own.
* With a dismiss handler it shows a close button; the caller removes the banner.
* Text wraps; long words break.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `tone` | tone | `Info` | Info, Success, Warning, or Error. |
| `title` | string | none | Optional bold first line. |
| `text` | string | required | Message. |
| `actions` | view | none | Buttons under the text. |
| `on_dismiss` | callback | none | Shows the close button. |

## Accessibility

Warning and error banners have the `alert` role; info and success have the `status` role.
