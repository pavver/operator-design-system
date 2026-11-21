# Field

Shell shared by every text-like input: a label, a state-bordered box with slots, and a hint or error line.
TextField, SearchField, the value fields and Combobox are built on it, and an application can wrap its own control in it to look like a native field.

Status: **draft** on web.

## Geometry

| Part | Value |
| :--- | :--- |
| Box height | `--op-field-height`, default `row_standard` (`10 DU`). |
| Box padding | `--op-field-padding-x` on both sides, default `pad_card`; none after a full-height trailing button. |
| Label | `caption` type in `text_secondary` above the box; `accent` while the box is focused. |
| Hint, error | `caption` type below the box; the error uses `error` and draws a standard `error` outline on the box. |
| Gaps | `--op-field-gap` between label, row and hint, and between the box and the `before`/`after` slots, default `gap_micro`. |
| Inner slots | `prepend` and `append` sit inside the box with `pad_tight` towards the control. |

## Variants

| Variant | Look |
| :--- | :--- |
| `outlined` | `--op-field-background` (`surface`) with the state outline. |
| `borderless` | No background and no base outline while idle or disabled; focus and result outlines still show. For fields inside table cells, property rows and node rows. |

## Behavior

* The box shows the state of the control as described in [states.md](../foundations/states.md): focused, disabled, pending (run segment), success and error.
* `error` replaces `hint`; both are announced (`role="alert"` for the error).
* The field never owns the value; the wrapped control does.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| children | view | required | The control, usually `<input class="op-field-input">`. |
| `state` | control state signal | required | State shown by the outline. |
| `label` | string | none | Text above the box. |
| `hint` | string | none | Text below the box. |
| `error` | string | none | Error text below the box. |
| `variant` | `outlined`, `borderless` | `outlined` | Box look. |
| `trailing` | bool signal | `false` | The box ends with a full-height button (no end padding). |
| `pending_run` | bool | `false` | Draws the pending run segment. |
| `role` | string | none | ARIA role of the box. |
| `box_class` | string | none | Extra class of the box, for components built on Field. |
| `metrics` | `FieldMetrics` | unset | Metric overrides, see below. |

## Customization

| Metric | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `height` | DU | `--op-field-height` | `row_standard` |
| `padding_x` | DU | `--op-field-padding-x` | `pad_card` |
| `slot_size` | DU | `--op-field-slot-size` | `row_standard` |
| `gap` | DU | `--op-field-gap` | `gap_micro` |
| `font_size` | TU | `--op-field-font-size` | `type_body_size` |

| Color variable | Default |
| :--- | :--- |
| `--op-field-background` | `surface` |
| `--op-field-text` | `text_primary` |

| Slot | Place |
| :--- | :--- |
| `prepend`, `append` | Inside the box, before and after the control. |
| `before`, `after` | Outside the box, on the same row. |
| `label_slot` | Replaces the label. |
| `hint_slot` | Replaces the hint and error line. |

`class` and `style` apply to the root.

## Accessibility

* The wrapped control carries the accessible name; components built on Field use the visible `label` when it is set.
* The error line has `role="alert"`.
