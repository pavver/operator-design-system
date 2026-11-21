# Toast

Brief message at the edge of the screen about something that just happened.
It is the place for the error text that a control cannot show in its own border.

Status: **reviewed** on web.

## Use

* Confirm a background action that has no visible control, for example "Settings saved".
* Report the reason of a failed async action: the control pulses red, the toast says why.
* Offer one quick follow-up, such as Undo or Retry.
* Do not use a toast for information the user must act on before continuing; use a [Dialog](dialog.md) or a [Banner](banner.md).

## Geometry

| Property | Value |
| :--- | :--- |
| Position | One of six places: top or bottom, each at the start, center, or end; `4 DU` from the viewport edges. |
| Default position | Bottom center. |
| Width | `min(105 DU, viewport − 8 DU)`. |
| Height | At least `12 DU`; grows with wrapped text. |
| Padding | `1 DU` vertical, `3 DU` start, `1 DU` end. |
| Start bar | `0.5 DU` in the tone color. |
| Outline | `0.25 DU` `border`, plus a soft drop shadow. |
| Icon | `8 DU`, tone color. |
| Tall layout | From 4 wrapped text lines: a header with the icon, the tone name (uppercase `caption`, tone color), the action, and the close button; the text spans the full width below, `3 DU` bottom padding. |
| Buttons | Action and close are `8 DU` tall; the action is an uppercase `caption` label in `accent`. |
| Stack | Up to 3 toasts, `2 DU` apart; the oldest is dropped first. |
| Order | The newest toast is nearest the viewport edge: at the bottom of a bottom stack, at the top of a top stack. |

## Tones

| Tone | Color | Icon | Role |
| :--- | :--- | :--- | :--- |
| Info | `info` | `info` | Neutral news. |
| Success | `success` | `check_circle` | Confirmed result. |
| Warning | `warning` | `warning` | Attention without failure. |
| Error | `error` | `error` | Failure with its reason. |

## Behavior

* A toast stays for `toast_duration` (`4000 ms`); error toasts stay for `toast_error_duration` (`8000 ms`).
* While the pointer is over a toast its timer is held; when the time has run out, the toast closes once the pointer leaves.
* The action runs its handler and closes the toast.
* The close button dismisses at once.
* The toast counts the wrapped lines of its text and switches to the tall layout at 4 lines or more.
* It switches back only at 2 lines or fewer, because the tall layout gives the text more width.
* The count follows width changes and text scaling.
* Enter: fade in and move `2 DU` away from the nearest edge over `150 ms`; exit reverses it.
* The position is set on the host and can change at runtime; toasts already on screen move with the stack.
* Toasts are created through a toaster handle provided by a host at the root of the application.
* `report(result, what)` shows an error toast for a failed result and does nothing on success.

## API

| Item | Description |
| :--- | :--- |
| `ToastHost` | Wraps the application; provides the toaster and renders the stack at its initial position. |
| `ToastPosition` | `TopStart`, `TopCenter`, `TopEnd`, `BottomStart`, `BottomCenter` (default), `BottomEnd`. |
| `set_position(position)` | Moves the stack. |
| `use_toaster()` | Returns the toaster, or nothing outside a host. |
| `show(spec)` | Shows a toast described by tone, text, optional action, and optional duration; returns its id. |
| `message(tone, text)` | Shorthand without an action. |
| `report(result, what)` | Error toast "`what` failed: reason" for a failed result. |
| `dismiss(id)` | Closes a toast. |

The customization properties are optional and accept reactive values where indicated.

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `class`, `style` | reactive string | none | Root hooks. |
| `shift` | reactive DU metric | design-system value | Sets `--op-toast-shift`. |
| `viewport_margin` | reactive DU metric | design-system value | Sets `--op-toast-viewport-margin`. |
| `gap` | reactive DU metric | design-system value | Sets `--op-toast-gap`. |
| `width` | reactive DU metric | design-system value | Sets `--op-toast-width`. |
| `min_height` | reactive DU metric | design-system value | Sets `--op-toast-min-height`. |
| `padding_y` | reactive DU metric | design-system value | Sets `--op-toast-padding-y`. |
| `padding_start` | reactive DU metric | design-system value | Sets `--op-toast-padding-start`. |
| `shadow_blur` | reactive DU metric | design-system value | Sets `--op-toast-shadow-blur`. |
| `label_font_size` | reactive TU metric | design-system value | Sets `--op-toast-label-font-size`. |
| `label_line_height` | reactive TU metric | design-system value | Sets `--op-toast-label-line-height`. |
| `text_padding_end` | reactive DU metric | design-system value | Sets `--op-toast-text-padding-end`. |
| `button_size` | reactive DU metric | design-system value | Sets `--op-toast-button-size`. |
| `font_size` | reactive TU metric | design-system value | Sets `--op-toast-font-size`. |
| `line_height` | reactive TU metric | design-system value | Sets `--op-toast-line-height`. |
| `icon_size` | reactive DU metric | design-system value | Sets `--op-toast-icon-size`. |
| `close_icon_size` | reactive DU metric | design-system value | Sets `--op-toast-close-icon-size`. |
| `letter_spacing` | reactive TU metric | design-system value | Sets `--op-toast-letter-spacing`. |
| `padding_end` | reactive DU metric | design-system value | Sets `--op-toast-padding-end`. |
| `shadow_y` | reactive DU metric | design-system value | Sets `--op-toast-shadow-y`. |
| `header` | `Slot` | none | Replaces the tone header. |
| `actions` | `Slot` | none | Actions before the optional ToastSpec action. |
| `footer` | `Slot` | none | Content after the body and actions. |
| `Toast.spec` | ToastSpec | `required` | Content and optional action for a standalone toast. |
| `Toast.on_dismiss` | callback | `none` | Called by close and by the built-in action; the caller removes the toast. |
| `Toast.on_hover` | callback(bool) | `none` | Reports pointer entry and exit. |
| `Toast.leaving` | bool signal | `false` | Runs the exit animation. |
| `ToastHost.label` | reactive string | `toast.notifications` | Stack accessible name. |
| `ToastHost.failed_text` | reactive string | `toast.failed` | Failure template with `{what}`. |
| `ToastHost.failed_reason_text` | reactive string | `toast.failed-reason` | Failure template with `{what}` and `{reason}`. |
| `dismiss_label` | reactive string | `toast.dismiss` | Close button accessible name. |
| `tone_label` | reactive string | `tone string key` | Tall toast header text. |
| `icon` | icon | `tone icon key` | Tone icon override. |
| `dismiss_icon` | icon | `toast.dismiss` | Close icon override. |

## Customization

`class` and `style` apply to the ToastHost stack or the standalone Toast container.
Metric props write the corresponding variables; an explicit `style` declaration wins.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `shift` | DU | `--op-toast-shift` | `2 DU` |
| `viewport_margin` | DU | `--op-toast-viewport-margin` | `margin_screen` |
| `gap` | DU | `--op-toast-gap` | `gutter` |
| `width` | DU | `--op-toast-width` | `105 DU` on the host; `100%` standalone |
| `min_height` | DU | `--op-toast-min-height` | `row_control` |
| `padding_y` | DU | `--op-toast-padding-y` | `pad_tight` |
| `padding_start` | DU | `--op-toast-padding-start` | `pad_card` |
| `shadow_blur` | DU | `--op-toast-shadow-blur` | `6 DU` |
| `label_font_size` | TU | `--op-toast-label-font-size` | `caption_size` |
| `label_line_height` | TU | `--op-toast-label-line-height` | `caption_line` |
| `text_padding_end` | DU | `--op-toast-text-padding-end` | `pad_small` |
| `button_size` | DU | `--op-toast-button-size` | `row_compact` |
| `font_size` | TU | `--op-toast-font-size` | `body_size` |
| `line_height` | TU | `--op-toast-line-height` | `body_line` |
| `icon_size` | DU | `--op-toast-icon-size` | `8 DU` |
| `close_icon_size` | DU | `--op-toast-close-icon-size` | `icon_small` |
| `letter_spacing` | TU | `--op-toast-letter-spacing` | `0.04em (relative to caption size)` |
| `padding_end` | DU | `--op-toast-padding-end` | `pad_tight` |
| `shadow_y` | DU | `--op-toast-shadow-y` | `2 DU` |

| Color variable | Default |
| :--- | :--- |
| `--op-toast-background` | `surface` |
| `--op-toast-tone` | `var(--tone)` |
| `--op-toast-border` | `border` |
| `--op-toast-shadow` | `rgb(0 0 0 / 35%)` |
| `--op-toast-text` | `text_primary` |
| `--op-toast-action` | `accent` |
| `--op-toast-secondary` | `text_secondary` |
| `--op-toast-hover` | `hover` |
| `--op-toast-ripple` | `ripple` |

| Slot | Place |
| :--- | :--- |
| `header` | Replaces the tone header. |
| `actions` | Actions before the optional ToastSpec action. |
| `footer` | Content after the body and actions. |

ToastHost accepts the same appearance metrics, color variables, slots, and icon overrides as Toast.
Its metric variables inherit into every hosted toast; its slots render inside each toast.
`viewport_margin` places the stack and caps standalone toast width at the viewport margins.
`gap` also separates the hosted toasts.
The standalone Toast delegates removal and lifetime management to its caller.
The host keeps the existing timer, hover pause, limit, ordering, and exit animation behavior.

### Strings

| Key | Default | Prop |
| :--- | :--- | :--- |
| `toast.notifications` | `Notifications` | `ToastHost.label` |
| `toast.dismiss` | `Dismiss` | `dismiss_label` |
| `toast.failed` | `{what} failed` | `ToastHost.failed_text` |
| `toast.failed-reason` | `{what} failed: {reason}` | `ToastHost.failed_reason_text` |
| `toast.info` | `Info` | `tone_label` |
| `toast.success` | `Success` | `tone_label` |
| `toast.warning` | `Warning` | `tone_label` |
| `toast.error` | `Error` | `tone_label` |

### Icons

| Key | Default | Prop |
| :--- | :--- | :--- |
| `toast.info` | `info` | `icon` |
| `toast.success` | `check_circle` | `icon` |
| `toast.warning` | `warning` | `icon` |
| `toast.error` | `error` | `icon` |
| `toast.dismiss` | `close` | `dismiss_icon` |

## Accessibility

* Error toasts have the `alert` role; others have the `status` role, so screen readers announce them.
* The stack is a region named "Notifications".
* A toast never takes focus.
