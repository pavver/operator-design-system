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

## Accessibility

* Error toasts have the `alert` role; others have the `status` role, so screen readers announce them.
* The stack is a region named "Notifications".
* A toast never takes focus.
