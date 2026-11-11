# Dialog

Modal window for a focused task: a short form, details, or a choice between options.
It is the shell for every modal window; [ConfirmDialog](confirm-dialog.md) is its preset for confirming one action.

Status: **draft** on web.

## Geometry

The dialog is a dynamic container and follows the clamp model from [units.md](../foundations/units.md).

| Property | Value |
| :--- | :--- |
| Width | `clamp(75 DU, 90% of the viewport, 140 DU)`, never wider than the viewport. |
| Height | Content height, at most the viewport minus `8 DU`; the body scrolls. |
| Background | `surface` with the state border from [states.md](../foundations/states.md). |
| Header | Title in `screen_title` style; close button at the end unless the dialog has a confirm action; `0.25 DU` `divider` below. |
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
* Without `on_confirm`, buttons come from `actions` and the caller closes the dialog.
* With `on_confirm`, the dialog adds Cancel and a confirm [ActionButton](action-button.md) after `actions`, and Cancel replaces the close button.
* Confirm runs the action without closing the dialog.
  The dialog border and the confirm button enter `Pending`; Cancel is disabled and the dialog cannot be dismissed.
* On **success** the dialog shows `Success` for `500 ms`, then closes immediately without a fade.
* On **error** the dialog shows `Error` for `500 ms`, then returns to `Idle` and fades the result.
  Confirm and Cancel unlock `300 ms` later, so the user can retry or cancel.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `open` | bool signal | required | Visibility. |
| `title` | string | required | Title and accessible name. |
| `actions` | view | none | Extra buttons at the bottom, before Cancel and confirm. |
| `dismissible` | bool | `true` | Escape, scrim, and close button. |
| `min_width` | DU | `75` | Clamp minimum. |
| `max_width` | DU | `140` | Clamp maximum. |
| `on_confirm` | async `() -> ActionResult` | none | Confirm action; adds Cancel and confirm buttons. |
| `confirm_text` | string | `"OK"` | Confirm button label. |
| `cancel_text` | string | `"Cancel"` | Cancel button label. |
| `destructive` | bool | `false` | Dangerous confirm button. |
| `alert` | bool | `false` | `alertdialog` role. |
| `children` | view | required | Body. |

## Accessibility

`dialog` role, or `alertdialog` when `alert` is set, with `aria-modal` and the title as its name.
