# PopupEdit

Inline value editing in an anchored popover with async save.
Replaces static text with an editing popover on demand, offering save and cancel controls.

Status: **draft** on web.

## Use

* Inline table cell editing or profile field adjustments.
* Quick property modifications without navigating away.

## Geometry

| Property | Value |
| :--- | :--- |
| Width | `72 DU`. |
| Padding | `pad_card` (`3 DU`). |
| Gap | `gap_small` (`2 DU`). |
| Corner radius | `0 DU`. |

## Behavior

* Clicking the trigger opens an anchored popover dialog containing a [TextField](text-field.md).
* The input is automatically focused upon opening.
* Submitting the form or clicking save runs validation and triggers the async save action.
* On success, the value updates and the popover closes.
* On error, the error message displays and the draft value remains.
* Pressing Escape or clicking outside cancels changes and closes the popover.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `value` | string signal | required | Target value to be edited. |
| `on_save` | `AsyncAction<String>` | none | Async action executed when saving. |
| `on_change` | callback `(String)` | none | Emits confirmed value updates. |
| `validate` | callback `(String -> Option<String>)` | none | Synchronous validation callback returning error text. |
| `enabled` | bool | `true` | Enables editing interaction. |
| `label` | string | `popup-edit.label` | Accessible name of the editor dialog. |
| `save_label` | string | `popup-edit.save` | Label for save action button. |
| `cancel_label` | string | `popup-edit.cancel` | Label for cancel action button. |
| `content` | `Slot` | none | Custom trigger visual replacing raw value text. |
| `header` | `Slot` | none | Content above the input in popover. |
| `footer` | `Slot` | none | Content below the actions in popover. |
| `width` | DU metric | `72 DU` | Popover panel width. |
| `padding` | DU metric | `pad_card` | Padding inside the panel. |
| `gap` | DU metric | `gap_small` | Spacing inside the form. |
| `class`, `style` | string | none | Root hooks. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `width` | DU | `--op-popup-edit-width` | `72 DU` |
| `padding` | DU | `--op-popup-edit-padding` | `pad_card` (`3 DU`) |
| `gap` | DU | `--op-popup-edit-gap` | `gap_small` (`2 DU`) |

| Color variable | Default |
| :--- | :--- |
| `--op-popup-edit-background` | `surface` |
| `--op-popup-edit-text` | `inherit` |
| `--op-popup-edit-accent` | `accent` |

| Slot | Place |
| :--- | :--- |
| `content` | Custom content within the trigger button. |
| `header` | Top of the editing panel. |
| `footer` | Bottom of the editing panel. |

`class` and `style` apply to the root wrapper.

### Strings

| Key | Default |
| :--- | :--- |
| `popup-edit.label` | `Edit value` |
| `popup-edit.save` | `Save` |
| `popup-edit.cancel` | `Cancel` |

### Icons

PopupEdit does not define built-in icons.

## Accessibility

The trigger button sets `aria-haspopup="dialog"` and `aria-expanded`.
The opened panel behaves as a modal dialog and traps focus within the text field and buttons.
Escape dismisses the editing session cleanly.
