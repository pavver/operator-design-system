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

The customization properties are optional and accept reactive values where indicated.

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `class`, `style` | reactive string | none | Root hooks. |
| `font_size` | reactive TU metric | design-system value | Sets `--op-confirm-dialog-font-size`. |
| `line_height` | reactive TU metric | design-system value | Sets `--op-confirm-dialog-line-height`. |
| `padding` | reactive DU metric | design-system value | Sets `--op-confirm-dialog-padding`. |
| `header_padding` | reactive DU metric | design-system value | Sets `--op-confirm-dialog-header-padding`. |
| `title_font_size` | reactive TU metric | design-system value | Sets `--op-confirm-dialog-title-font-size`. |
| `title_line_height` | reactive TU metric | design-system value | Sets `--op-confirm-dialog-title-line-height`. |
| `gap` | reactive DU metric | design-system value | Sets `--op-confirm-dialog-gap`. |
| `button_height` | reactive DU metric | design-system value | Sets `--op-confirm-dialog-button-height`. |
| `width` | reactive DU metric | design-system value | Sets `--op-confirm-dialog-width`. |
| `max_height` | reactive DU metric | design-system value | Sets `--op-confirm-dialog-max-height`. |
| `header` | `Slot` | none | Replaces the title content. |
| `actions` | `Slot` | none | Additional actions before the built-in action buttons. |
| `footer` | `Slot` | none | Content after the body and actions. |

## Customization

`class` and `style` apply to the Dialog scrim root, with variables inherited by the confirmation panel.
Metric props write the corresponding variables; an explicit `style` declaration wins.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `font_size` | TU | `--op-confirm-dialog-font-size` | `body_size` |
| `line_height` | TU | `--op-confirm-dialog-line-height` | `body_line` |
| `padding` | DU | `--op-confirm-dialog-padding` | `margin_screen` |
| `header_padding` | DU | `--op-confirm-dialog-header-padding` | `gutter` |
| `title_font_size` | TU | `--op-confirm-dialog-title-font-size` | `screen_title_size` |
| `title_line_height` | TU | `--op-confirm-dialog-title-line-height` | `screen_title_line` |
| `gap` | DU | `--op-confirm-dialog-gap` | `pad_card` |
| `button_height` | DU | `--op-confirm-dialog-button-height` | `row_compact` |
| `width` | DU | `--op-confirm-dialog-width` | `75 DU` |
| `max_height` | DU | `--op-confirm-dialog-max-height` | `calc(100vh - var(--op-confirm-dialog-padding, margin_screen) * 2)` |

| Color variable | Default |
| :--- | :--- |
| `--op-confirm-dialog-text` | `text_secondary` |
| `--op-confirm-dialog-title-text` | `text_primary` |
| `--op-confirm-dialog-background` | `surface` |
| `--op-confirm-dialog-divider` | `divider` |
| `--op-confirm-dialog-scrim` | `rgb(0 0 0 / 55%)` |
| `--op-confirm-dialog-border` | `border` |
| `--op-confirm-dialog-pending` | `pending` |
| `--op-confirm-dialog-success` | `success` |
| `--op-confirm-dialog-error` | `error` |

| Slot | Place |
| :--- | :--- |
| `header` | Replaces the title content. |
| `actions` | Additional actions before the built-in action buttons. |
| `footer` | Content after the body and actions. |

The confirmation family maps its container variables into the public Dialog shell.
`width` sets both clamp bounds, preserving the fixed default width.

### Strings

| Key | Default | Prop |
| :--- | :--- | :--- |
| `confirm-dialog.confirm` | `Delete` | `confirm_text` |
| `confirm-dialog.cancel` | `Cancel` | `cancel_text` |

### Icons

This component has no built-in icons.
