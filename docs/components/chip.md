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
| `text` | string | `chip.text` | Label. |
| `icon` | icon | none | Leading icon. |
| `selected` | bool signal | none | Makes a filter chip. |
| `on_click` | callback | none | Makes an action chip. |
| `on_remove` | callback | none | Adds the remove button. |
| `enabled` | bool | `true` | Enables interaction. |
| `color` | `ChipColor` | `Neutral` | Tone or custom color. |
| `height` | reactive DU | unset | Overrides `--op-chip-height`; see Customization. |
| `gap` | reactive DU | unset | Overrides `--op-chip-gap`; see Customization. |
| `padding_x` | reactive DU | unset | Overrides `--op-chip-padding-x`; see Customization. |
| `remove_size` | reactive DU | unset | Overrides `--op-chip-remove-size`; see Customization. |
| `remove_overlap` | reactive DU | unset | Overrides `--op-chip-remove-overlap`; see Customization. |
| `icon_size` | reactive DU | unset | Overrides `--op-chip-icon-size`; see Customization. |
| `font_size` | reactive TU | unset | Overrides `--op-chip-font-size`; see Customization. |
| `line_height` | reactive TU | unset | Overrides `--op-chip-line-height`; see Customization. |
| `class` | reactive string | none | Applied to the root element. |
| `style` | reactive string | none | Applied to the root element. |
| `leading` | slot | none | Before the default content. |
| `trailing` | slot | none | After the default content. |
| `content` | slot | none | Replaces the default text or icon; pending and result feedback keep priority. |
| `remove_label` | reactive string | `chip.remove` | String override; see Strings. |
| `checked_icon` | icon | `chip.checked` | Semantic icon override; see Icons. |
| `remove_icon` | icon | `chip.remove` | Semantic icon override; see Icons. |

`ChipSet` lays chips out in wrapping rows.

### ChipSet API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `children` | children | required | The chips. |
| `gap` | reactive DU | unset | Overrides `--op-chip-set-gap`, which falls back to `pad_tight`. |
| `class`, `style` | reactive string | none | Applied to the set root. |

## Accessibility

* A filter chip is a `checkbox` with `aria-checked`.
* The remove button uses the localized `chip.remove` template with the chip text inserted.

## Customization

`class` and `style` apply to the root element.
Metric props accept reactive DU or TU values; unset props keep the CSS variable or its exact design-system fallback.
Caller `style` is appended after metric declarations and takes precedence.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `height` | DU | `--op-chip-height` | `var(--op-dim-row-dense)` |
| `gap` | DU | `--op-chip-gap` | `var(--op-dim-pad-tight)` |
| `padding_x` | DU | `--op-chip-padding-x` | `var(--op-dim-gutter)` |
| `remove_size` | DU | `--op-chip-remove-size` | `var(--op-dim-row-dense)` |
| `remove_overlap` | DU | `--op-chip-remove-overlap` | `var(--op-dim-pad-tight)` |
| `icon_size` | DU | `--op-chip-icon-size` | `calc(var(--op-du) * 3.5)` |
| `font_size` | TU | `--op-chip-font-size` | `var(--op-type-body-small-size)` |
| `line_height` | TU | `--op-chip-line-height` | `var(--op-type-body-small-line)` |
| Color variable | Default |
| :--- | :--- |
| `--op-chip-accent` | `var(--op-accent)` |
| `--op-chip-border` | `var(--op-border)` |
| `--op-chip-emphasis-border` | `var(--op-border-strong)` |
| `--op-chip-disabled-border` | `var(--op-disabled-border)` |
| `--op-chip-disabled-background` | `var(--op-disabled-surface)` |
| `--op-chip-error` | `var(--op-error)` |
| `--op-chip-hover` | `var(--op-hover)` |
| `--op-chip-info` | `var(--op-info)` |
| `--op-chip-pending` | `var(--op-pending)` |
| `--op-chip-pressed` | `var(--op-pressed)` |
| `--op-chip-ripple` | `var(--op-ripple)` |
| `--op-chip-selection` | `var(--op-selection)` |
| `--op-chip-success` | `var(--op-success)` |
| `--op-chip-background` | `var(--op-surface)` |
| `--op-chip-disabled-text` | `var(--op-text-disabled)` |
| `--op-chip-text` | `var(--op-text-primary)` |
| `--op-chip-secondary-text` | `var(--op-text-secondary)` |
| `--op-chip-warning` | `var(--op-warning)` |
| Slot | Place |
| :--- | :--- |
| `leading` | Before the default content. |
| `trailing` | After the default content. |
| `content` | Replaces the default text or icon; pending and result feedback keep priority. |

ChipSet also accepts root `class` and `style`, and a DU `gap` prop backed by `--op-chip-set-gap` with `pad_tight` as its fallback.
The existing `ChipColor::Custom` value is written before the caller style, so the caller can override it.

### Strings

| Prop | Key | Default |
| :--- | :--- | :--- |
| `text` | `chip.text` | empty string |
| `remove_label` | `chip.remove` | `Remove {text}` |

`remove_label` is a complete template; `{text}` is replaced with the chip text so translations can choose the word order.

### Icons

| Prop | Key | Default |
| :--- | :--- | :--- |
| `checked_icon` | `chip.checked` | `check` |
| `remove_icon` | `chip.remove` | `close` |

Icons supplied as item data or through the existing `icon` prop remain caller-owned.
