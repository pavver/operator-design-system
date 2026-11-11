# States and Async Feedback

Every interactive component follows one state machine and one feedback language.
This is the heart of the system; a port that gets this right will feel like Operator UI even with rough edges elsewhere.

## ControlState

| State | Meaning | Default look |
| :--- | :--- | :--- |
| `Idle` | Waiting for input. | `border` outline. |
| `Pressed` | Pointer is down on the control. | `border_strong` outline, `pressed` background, ripple. |
| `Focused` | Keyboard or text focus. | `accent` outline at 1.5x stroke. |
| `Selected` | The control represents the current choice. | `accent` outline at 1.5x stroke. |
| `Pending` | An async request is running. | Running `pending` segment on the outline. |
| `Success` | The request succeeded. | Pulsing `success` outline. |
| `Error` | The request failed. | Pulsing `error` outline. |
| `Disabled` | Not available. | `disabled_border`, `disabled_surface`, `text_disabled`. |

A component may receive an external state.
When it does, the external state replaces the component's own async state.
`Disabled` always wins over every other state.

## ActionResult

An async handler returns `Success` or `Error`, and an error may carry a message.
Handlers take the value the user requested and return a future.

## The State Border

The state border is the main feedback surface of every control.
It is layered as follows, from bottom to top:

1. **Base outline.**
   Drawn inside the bounds with the stroke and color of the current state.
2. **Result background** (only on components that tint their background).
   The result color at `15%` opacity on its own layer.
3. **Result pulse.**
   A 2x stroke outline in the result color.
   Its alpha oscillates between `0.55` and `1.0` with a `400 ms` half period for as long as the result state lasts.
4. **Pending segment.**
   A segment covering 25% of the perimeter, drawn at 1.5x stroke in `pending`.
   It runs clockwise and completes one lap in `2000 ms`.
   The base outline stays visible underneath it.

### Result exit

When a result state ends, the component remembers which result it was.
The pulse layer, the result background, and result-colored content fade out over `500 ms` with the smooth curve.
The base outline of the new state is already visible underneath, so there is never a neutral frame.

### No glow

Feedback uses sharp vector strokes and flat tints only.
Blur, glow, and shadows are not used.

## Async Interaction Model

* **Pending blocks input.**
  While a control is `Pending` it ignores clicks, drags, keys, and ripples.
  This prevents duplicate requests.
* **Results do not block input.**
  During a `Success` or `Error` flash the control is fully interactive.
  A new interaction starts a new request immediately.
* **No optimistic updates.**
  A control shows the requested value while pending, but commits it only after `Success`.
* **Rollback on error.**
  After `Error` the control returns to the last confirmed value.
  The return uses the same animation as the forward movement.
* **Superseded requests are ignored.**
  When a newer request starts, the result of an older one is discarded.
* **Local result cycle.**
  A component that owns its result state holds it for `800 ms`, then fades it out over `500 ms`.

Some components define exceptions in their own specification.

## Press Feedback

A bounded ripple in the `ripple` color spreads from the pointer position inside the control.
Its opacity is `24%` in dark mode and `16%` in light mode.
The ripple grows to cover the control over `330 ms` and holds its last frame while the pointer stays down.
It fades over `220 ms` only after the pointer is released and the growth has finished.
Controls that act on release therefore show a steady pressed surface for as long as the press lasts.
Press feedback is suppressed while the control is `Pending` or `Disabled`.

## Hover Feedback

On devices with a hovering pointer, an `Idle` control under the pointer shows a `hover` background and a `border_strong` outline.
Danger buttons and checked or selected items deepen their own tint to `22%` instead of using `hover`.
Hover never changes `Pressed`, `Selected`, `Pending`, `Success`, `Error`, or `Disabled` visuals.
Touch screens have no hover state; press feedback alone confirms the touch.

## Focus

Keyboard focus shows a `0.5 DU` `accent` ring outside the control, offset by `0.5 DU`.
Text fields show focus through their own state border instead.
