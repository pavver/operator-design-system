# Chip

Compact token for a filter, a value, or a tag.

Status: **reviewed** on web.

## Variants

| Variant | Purpose | Interaction |
| :--- | :--- | :--- |
| Filter chip | Turns a filter on or off. | Toggles on click; shows a check mark when on. |
| Input chip | One value of a multi-value field, such as a tag. | Has a remove button. |
| Action chip | A small shortcut action. | Calls its handler. |
| Static tag | A label, such as a codec or resolution. | None. |

## Geometry

| Property | Value |
| :--- | :--- |
| Height | `6 DU`. |
| Padding | `2 DU` horizontal. |
| Type | `body_small` style. |
| Icon and check | `3.5 DU`, `1 DU` from the text. |
| Remove button | `6 DU` wide at the end. |
| Outline | State border. |
| Set | Chips wrap with `1 DU` gaps. |
| Corners | `0 DU`. |

## States

* Selected: `selection` background, `accent` text, and the `Selected` border.
* Pressed: `pressed` background.
* Disabled: disabled colors.
* The remove button turns `error` on hover.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `text` | string | required | Label. |
| `icon` | icon | none | Leading icon. |
| `selected` | bool signal | none | Makes a filter chip. |
| `on_click` | callback | none | Makes an action chip. |
| `on_remove` | callback | none | Adds the remove button. |
| `enabled` | bool | `true` | Enables interaction. |

`ChipSet` lays chips out in wrapping rows.

## Accessibility

* A filter chip is a `checkbox` with `aria-checked`.
* The remove button is named "Remove" followed by the chip text.
