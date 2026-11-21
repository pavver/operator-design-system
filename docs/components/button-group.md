# ButtonGroup

Container grouping related [ActionButton](action-button.md) and [IconButton](icon-button.md) controls with shared borders.
It creates visually cohesive command strips and segmented action bars.

Status: **draft** on web.

## Use

Use `ButtonGroup` to bundle actions that belong to the same workflow, such as alignment controls, edit/preview modes, or zoom controls.
Do not use `ButtonGroup` for mutually exclusive single selections when [SegmentedControl](segmented-control.md) or [RadioGroup](radio-group.md) is intended.

## Geometry

| Property | Value |
| :--- | :--- |
| Gap | `0 DU` (`--op-button-group-gap`). |
| Overlap | `0.25 DU` negative margin on subsequent buttons so adjacent borders merge into one single standard border. |

## Behavior

* Adjacent buttons share their dividing borders.
* When focused or hovered, an active button is elevated above its siblings so its focus outline stays fully visible.
* In `spread` mode, each child expands equally to fill the full container width.
* In `vertical` mode, buttons stack vertically with shared horizontal borders.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `vertical` | bool | `false` | Stacks buttons vertically. |
| `spread` | bool | `false` | Stretches buttons equally to fill width. |
| `label` | string | `button-group.label` | Accessible label for the group. |
| `gap` | DU | none | Gap between buttons in DU (`0 DU` by default). |
| `class` | string | none | CSS classes on the root element. |
| `style` | string | none | Inline CSS on the root element. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `gap` | DU | `--op-button-group-gap` | `0` |

`class` and `style` apply to the root `div`.

### Strings

| Key | Default |
| :--- | :--- |
| `button-group.label` | `"Button group"` |

### Icons

`ButtonGroup` uses no internal icons; children provide their own icons.

## Accessibility

`ButtonGroup` renders with `role="group"` and an accessible label (`aria-label`).
