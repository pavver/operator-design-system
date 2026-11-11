# ConfirmDialog

Modal confirmation for an important or destructive action that runs asynchronously inside the dialog.

Status: **reviewed** on web.

## Composition

ConfirmDialog is a preset of [Dialog](dialog.md) and shares its geometry, colors, and behavior.
It sets:

* Width: fixed `75 DU`, or the viewport width minus margins when narrower.
* Body: the message in `body` style and `text_secondary`.
* Buttons: Cancel and confirm; the confirm button is dangerous when `destructive` is set.
* Role: `alertdialog`.

## Behavior

* Confirm runs the action inside the dialog, with the confirm states described in [Dialog](dialog.md#behavior).
* Success closes the dialog after `500 ms`; error keeps it open so the user can retry or cancel.
* Cancel, Escape, and a click on the scrim close the dialog unless a request is running.

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
