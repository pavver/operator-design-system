# TextField

Single-line text input with an integrated async SAVE button and an optional password reveal toggle.

Status: **reviewed** on web.

## Use

* Editing a short value that is saved to a backend: a name, an address, a URL.
* Entering secrets with a reveal toggle.

## Anatomy

```
┌──────────────────────────────────────────────┬────┬──────────┐
│ value text                                   │ () │ ✓ SAVE   │
└──────────────────────────────────────────────┴────┴──────────┘
  input                                   reveal    save button
```

* **Container:** owns the state border.
* **Input:** fills the remaining width.
* **Reveal toggle** (password fields): square, flush to the right.
* **Save button** (when modified): an [ActionButton](action-button.md), flush to the right edge.

## Geometry

| Property | Value |
| :--- | :--- |
| Width | Fills the parent. |
| Height | `row_standard` (`10 DU`). |
| Corner radius | `0 DU`. |
| Left padding | `pad_card` (`3 DU`). |
| Right padding | `pad_card`, or `0 DU` while a trailing control is shown. |
| Reveal toggle | `10 DU` × full height, icon `icon_small` (`4 DU`). |
| Save button | Full height, width `12 DU` to `30 DU`, sized by its label. |
| Text | `body` style, `text_primary`; caret in `accent`. |
| Placeholder | `text_disabled`. |

## States

| State | Look |
| :--- | :--- |
| `Idle` | `surface` background, `border` outline. |
| `Focused` | `accent` outline at 1.5x stroke. |
| `Pending` | Running `pending` segment on the field outline. |
| `Success` / `Error` | Pulsing result outline on the field. |
| `Disabled` | `disabled_surface` background, `disabled_border`, `text_disabled` text. |

The field outline is drawn above the trailing controls.
The focus and result outlines therefore enclose the reveal toggle and the save button as one block.
The save button keeps its own left edge as a separator.

## Behavior

### Save button

* The save button appears when the value differs from the baseline (`initial_value`) and a save handler exists.
* It disappears when the value returns to the baseline, unless a save cycle is running.
* Pressing the save button does not move focus away from the input, so the field keeps its `Focused` outline.
* The save button does not draw its own pending segment; the field outline shows it instead.
* On success the save button keeps the check icon for the whole result phase instead of returning to its label.
* While the save request is pending, the save button ignores clicks.
  Editing the text stays possible.

### Save cycle

1. The field enters `Pending`.
2. On **success** the field shows `Success`.
   After `700 ms` the save button hides.
   After another `100 ms` the field returns to `Idle` and the result outline fades out over `500 ms`.
   The parent updates the baseline to the saved value.
3. On **error** the field shows `Error` for `1000 ms`, then returns to `Idle`.
   The save button stays visible so the user can retry.

The save button hides `100 ms` before the field leaves its result state, so its label never flashes back.

### Password reveal

* A password field shows the reveal toggle by default.
* The toggle switches the input between masked and plain text.
* The icon shows the action, not the state: `visibility` while masked, `visibility_off` while revealed.
* The icon is `text_secondary`, and `accent` on hover or while revealed.
* Pressing the toggle does not move focus away from the input.
* The toggle can be turned off with `show_reveal = false`.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `value` | read-write string | required | The current text. |
| `placeholder` | string | required | Hint shown while empty. |
| `initial_value` | string | value at creation | Baseline used to detect modifications. |
| `state` | `ControlState?` | none | External state override. |
| `enabled` | bool | `true` | Interactivity. |
| `save_text` | string | `"SAVE"` | Save button label. |
| `save_icon` | icon | `check` | Save button icon. |
| `on_save` | async `(String) -> ActionResult` | none | Save handler; enables the save button. |
| `password` | bool | `false` | Masks the input. |
| `show_reveal` | bool | `true` | Shows the reveal toggle on password fields. |
| `label` | string | none | Accessible name of the input. |

## Accessibility

* The input carries `label` as its accessible name.
* The reveal toggle is a button with the name "Show password" or "Hide password" and a pressed state.
* The save button reports busy while pending.
