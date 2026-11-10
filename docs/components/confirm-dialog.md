# ConfirmDialog

Modal confirmation for an important or destructive action that runs asynchronously inside the dialog.

Status: **reviewed** on web.

## Geometry

| Property | Value |
| :--- | :--- |
| Width | `75 DU`, or the viewport width minus margins when narrower. |
| Height | Grows with the message. |
| Padding | `margin_screen` (`4 DU`). |
| Gap between title, message, and actions | `margin_screen`. |
| Actions | Right-aligned, `pad_card` (`3 DU`) apart, wrapping on narrow screens. |
| Corner radius | `0 DU`. |
| Scrim | Black at 55% over the whole viewport. |

## Colors

* Dialog background `surface` with a state border.
* Title `section_title` style, message `body` style in `text_secondary`.
* The confirm button is a dangerous [ActionButton](action-button.md) when `destructive` is set.

## Behavior

* Entry: opacity from 0 and scale from `0.95` to 1 over `dialog_enter` (`150 ms`); no bounce.
* Confirm runs the action without closing the dialog.
  The dialog border and the confirm button enter `Pending`; cancel is disabled.
* On **success** the dialog shows `Success` for `500 ms`, then closes immediately without a fade.
* On **error** the dialog shows `Error` for `500 ms`, then returns to `Idle` and fades the result.
  Confirm and cancel unlock `300 ms` later, so the user can retry or cancel.
* Cancel and a click on the scrim close the dialog unless a request is running.

## API

| Property | Type | Default |
| :--- | :--- | :--- |
| `open` | read-write bool | required |
| `title` | string | required |
| `message` | string | required |
| `confirm_text` | string | `"Delete"` |
| `cancel_text` | string | `"Cancel"` |
| `destructive` | bool | `true` |
| `on_confirm` | async `() -> ActionResult` | required |
