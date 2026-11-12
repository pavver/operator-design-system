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

## Colors

| Color | Use |
| :--- | :--- |
| `Neutral` | Default: `surface` background, `text_primary`. |
| `Accent`, `Success`, `Warning`, `Error`, `Info` | Status of the value, using the matching color token. |
| `Custom(color)` | User-defined tags with their own CSS color. |

* A colored chip has a background of 14% of the color over `surface`, text and icons in the color, and an outline of 55% of the color over `surface`.
* A custom color is not tuned per mode, so its text is mixed 65% with `text_primary` to stay readable in dark and light mode.
* Selected colored chip: 28% background and a `1.5×` outline in the text color.
* Pressed colored chip: 22% of the color over `pressed`.
* Disabled chips use the disabled colors whatever their color.
* Color never carries meaning alone: tone chips also say it in the text or the icon.

## States

* Selected: `selection` background, `accent` text, and the `Selected` border; colored chips follow [Colors](#colors).
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
| `color` | `ChipColor` | `Neutral` | Tone or custom color. |

`ChipSet` lays chips out in wrapping rows.

## Accessibility

* A filter chip is a `checkbox` with `aria-checked`.
* The remove button is named "Remove" followed by the chip text.
