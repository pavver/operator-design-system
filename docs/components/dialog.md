# Dialog

Modal window for a focused task: a short form, details, or a choice between options.
[ConfirmDialog](confirm-dialog.md) is the specialized dialog for confirming one action.

Status: **draft** on web.

## Geometry

The dialog is a dynamic container and follows the clamp model from [units.md](../foundations/units.md).

| Property | Value |
| :--- | :--- |
| Width | `clamp(75 DU, 90% of the viewport, 140 DU)`, never wider than the viewport. |
| Height | Content height, at most the viewport minus `8 DU`; the body scrolls. |
| Background | `surface` with a `0.25 DU` `border` outline. |
| Header | Title in `screen_title` style; close button at the end; `0.25 DU` `divider` below. |
| Header padding | `2 DU`, `4 DU` at the start. |
| Body | `4 DU` padding, `3 DU` gap between blocks. |
| Actions | Right-aligned, `3 DU` × `4 DU` padding, `0.25 DU` `divider` above. |
| Scrim | Black at 55%. |
| Corners | `0 DU`. |

## Behavior

* Enter: scrim fades in and the dialog scales from 95% over `dialog_enter` (`150 ms`).
* Opening moves focus into the dialog.
* When `dismissible`, Escape, a scrim click, and the close button close it.
  A non-dismissible dialog hides the close button and must be closed by an action.
* Async actions use [ActionButton](action-button.md) states; the caller closes the dialog after success.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `open` | bool signal | required | Visibility. |
| `title` | string | required | Title and accessible name. |
| `actions` | view | none | Buttons at the bottom. |
| `dismissible` | bool | `true` | Escape, scrim, and close button. |
| `min_width` | DU | `75` | Clamp minimum. |
| `max_width` | DU | `140` | Clamp maximum. |
| `children` | view | required | Body. |

## Accessibility

`dialog` role with `aria-modal` and the title as its name.
