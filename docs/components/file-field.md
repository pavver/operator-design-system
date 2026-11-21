# FileField

File picker and drop target built on the shared Field shell.
Supports selecting multiple files, file size limits, MIME type filtering, drag-and-drop, and removable chip items.

Status: **draft** on web.

## Use

* Selecting files from the file system.
* Dropping files directly onto an input target.

## Geometry

The control adopts the geometry and layout of [Field](field.md).
Default height is `row_standard` (`10 DU`), expanding vertically as file chips wrap.

## Behavior

* Clicking the picker button opens the native file browser.
* Dragging files over the control highlights the drop zone.
* Dropping files filters them according to `accept` rules and `max_size`.
* Valid files are added to the list, while rejected filenames emit through `on_rejected`.
* Selected files render as removable [Chip](chip.md) items.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `value` | `Vec<PickedFile>` signal | required | List of currently selected files. |
| `multiple` | bool | `true` | Allows selecting more than one file. |
| `append_files` | bool | `true` | Appends newly picked files to the current list. |
| `accept` | string | none | MIME types or extensions filter (e.g. `image/*,.pdf`). |
| `max_files` | usize | none | Maximum count of files allowed. |
| `max_size` | f64 | none | Maximum allowed file size in bytes. |
| `enabled` | bool | `true` | Enables input and drop actions. |
| `state` | `ControlState` | none | Explicit control state override. |
| `on_change` | callback `(Vec<PickedFile>)` | none | Emitted when files are updated. |
| `on_rejected` | callback `(Vec<String>)` | none | Emitted with names of rejected files. |
| `label` | string | `file-field.label` | Visible label and accessible name. |
| `hint` | string | none | Helpful description under the field. |
| `error` | string | none | Error message replacing the hint. |
| `pick_label` | string | `file-field.pick` | Label for the picker button. |
| `pick_icon` | icon | `file-field.pick` | Icon for the picker button. |
| `variant` | `FieldVariant` | `Outlined` | Border style variant. |
| `prepend` | `Slot` | none | Slot before the file chips. |
| `append` | `Slot` | none | Slot after the file chips. |
| `before` | `Slot` | none | Slot before the field box. |
| `after` | `Slot` | none | Slot after the field box. |
| `file` | `ScopedSlot<PickedFile>` | none | Scoped slot to render custom file items. |
| `chip_gap` | DU metric | `gap_micro` | Spacing between file chips. |
| `class`, `style` | string | none | Root hooks. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `height` | DU | `--op-field-height` | `row_standard` (`10 DU`) |
| `padding_x` | DU | `--op-field-padding-x` | `pad_card` |
| `slot_size` | DU | `--op-field-slot-size` | `row_standard` |
| `gap` | DU | `--op-field-gap` | `gap_micro` |
| `font_size` | TU | `--op-field-font-size` | `type_body_size` |
| `chip_gap` | DU | `--op-file-field-chip-gap` | `gap_micro` (`0.5 DU`) |

| Color variable | Default |
| :--- | :--- |
| `--op-file-field-drop-background` | `hover` |
| `--op-file-field-text` | `text_primary` |
| `--op-file-field-disabled-text` | `text_disabled` |

| Slot | Place |
| :--- | :--- |
| `prepend` | Inside the box, preceding the file list. |
| `append` | Inside the box, trailing the file list. |
| `before` | Outside the box, preceding it. |
| `after` | Outside the box, trailing it. |
| `file` | Scoped slot rendering an individual file item. |

`class` and `style` apply to the root field container.

### Strings

| Key | Default |
| :--- | :--- |
| `file-field.pick` | `Choose files` |
| `file-field.label` | `Files` |

### Icons

| Key | Default |
| :--- | :--- |
| `file-field.pick` | `add` |

## Accessibility

The picker button has accessible labelling and triggers a hidden native file input.
Selected files expose individual accessible remove buttons on chips.
Dropping files updates assistive technology through state notifications.
