# PropertyList

List of label and value pairs, such as device details or connection settings.

Status: **draft** on web.

## Geometry

| Property | Value |
| :--- | :--- |
| Row | At least `10 DU`, `1 DU` vertical padding, `0.25 DU` `divider` below. |
| Columns | Label 40%, value 60%, `2 DU` apart. |
| Label | `text_secondary`. |
| Value | `text_primary`; long values wrap and break anywhere. |
| Copy button | `6 DU` [IconButton](icon-button.md) with `content_copy`, at the end of the value. |

When the list is narrower than `60 DU`, the label moves above the value in `caption` style.

## Behavior

* The copy button writes the value to the clipboard as an async action, so it shows the success check or the error pulse.
* A value can be any view, for example a [StatusBadge](status-badge.md).

## API

| Component | Properties |
| :--- | :--- |
| `PropertyList` | `children`: properties. |
| `Property` | `label`, `value` (string) or `children` (view), `copyable` (`false`). |

## Accessibility

A description list: labels are terms, values are descriptions.
