# Dialog

Modal window for a focused task: a short form, details, or a choice between options.
It is the shell for every modal window; [ConfirmDialog](confirm-dialog.md) is its preset for confirming one action.

Status: **reviewed** on web.

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
* While the dialog is open, the page behind it does not scroll.
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
| `actions` | `Slot` | none | Extra buttons at the bottom, before Cancel and confirm. |
| `dismissible` | bool | `true` | Escape, scrim, and close button. |
| `min_width` | reactive DU metric | `75` | Clamp minimum. |
| `max_width` | reactive DU metric | `140` | Clamp maximum. |
| `on_confirm` | async `() -> ActionResult` | none | Confirm action; adds Cancel and confirm buttons. |
| `confirm_text` | string | `"OK"` | Confirm button label. |
| `cancel_text` | string | `"Cancel"` | Cancel button label. |
| `destructive` | bool | `false` | Dangerous confirm button. |
| `alert` | bool | `false` | `alertdialog` role. |
| `children` | view | required | Body. |

The customization properties are optional and accept reactive values where indicated.

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `class`, `style` | reactive string | none | Root hooks. |
| `padding` | reactive DU metric | design-system value | Sets `--op-dialog-padding`. |
| `header_padding` | reactive DU metric | design-system value | Sets `--op-dialog-header-padding`. |
| `title_font_size` | reactive TU metric | design-system value | Sets `--op-dialog-title-font-size`. |
| `title_line_height` | reactive TU metric | design-system value | Sets `--op-dialog-title-line-height`. |
| `gap` | reactive DU metric | design-system value | Sets `--op-dialog-gap`. |
| `button_height` | reactive DU metric | design-system value | Sets `--op-dialog-button-height`. |
| `font_size` | reactive TU metric | design-system value | Sets `--op-dialog-font-size`. |
| `line_height` | reactive TU metric | design-system value | Sets `--op-dialog-line-height`. |
| `close_size` | reactive DU metric | design-system value | Sets `--op-dialog-close-size`. |
| `close_icon_size` | reactive DU metric | design-system value | Sets `--op-dialog-close-icon-size`. |
| `max_height` | reactive DU metric | design-system value | Sets `--op-dialog-max-height`. |
| `header` | `Slot` | none | Replaces the title content. |
| `footer` | `Slot` | none | Content after the body and actions. |
| `close_label` | reactive string | `dialog.close` | Close button accessible name. |
| `close_icon` | icon | `dialog.close` | Close button icon. |

## Customization

`class` and `style` apply to the scrim root, with variables inherited by the dialog panel.
Metric props write the corresponding variables; an explicit `style` declaration wins.
The text variable colors the panel and title; nested components retain their own color hooks.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `padding` | DU | `--op-dialog-padding` | `margin_screen` |
| `header_padding` | DU | `--op-dialog-header-padding` | `gutter` |
| `title_font_size` | TU | `--op-dialog-title-font-size` | `screen_title_size` |
| `title_line_height` | TU | `--op-dialog-title-line-height` | `screen_title_line` |
| `gap` | DU | `--op-dialog-gap` | `pad_card` |
| `button_height` | DU | `--op-dialog-button-height` | `row_compact` |
| `min_width` | DU | `--op-dialog-min-width` | `75 DU` |
| `max_width` | DU | `--op-dialog-max-width` | `140 DU` |
| `font_size` | TU | `--op-dialog-font-size` | `body_size` |
| `line_height` | TU | `--op-dialog-line-height` | `body_line` |
| `close_size` | DU | `--op-dialog-close-size` | `row_compact` |
| `close_icon_size` | DU | `--op-dialog-close-icon-size` | `icon_small` (`4 DU`) |
| `max_height` | DU | `--op-dialog-max-height` | `calc(100vh - var(--op-dialog-padding, margin_screen) * 2)` |

| Color variable | Default |
| :--- | :--- |
| `--op-dialog-scrim` | `rgb(0 0 0 / 55%)` |
| `--op-dialog-background` | `surface` |
| `--op-dialog-divider` | `divider` |
| `--op-dialog-text` | `text_primary` |
| `--op-dialog-border` | `border` |
| `--op-dialog-success` | `success` |
| `--op-dialog-error` | `error` |
| `--op-dialog-pending` | `pending` |

| Slot | Place |
| :--- | :--- |
| `header` | Replaces the title content. |
| `actions` | Additional actions before the built-in action buttons. |
| `footer` | Content after the body and actions. |

### Strings

| Key | Default | Prop |
| :--- | :--- | :--- |
| `dialog.confirm` | `OK` | `confirm_text` |
| `dialog.cancel` | `Cancel` | `cancel_text` |
| `dialog.close` | `Close` | `close_label` |

### Icons

| Key | Default | Prop |
| :--- | :--- | :--- |
| `dialog.close` | `close` | `close_icon` |

## Accessibility

`dialog` role, or `alertdialog` when `alert` is set, with `aria-modal` and the title as its name.
