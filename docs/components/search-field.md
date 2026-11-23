# SearchField

Single-line input for filtering and searching.
It reports the query after a short pause, so results follow the typing without a request per key.

Status: **reviewed** on web.

## Geometry

The field has the shape of [TextField](text-field.md) with a leading slot:

| Property | Value |
| :--- | :--- |
| Height | `10 DU`. |
| Leading slot | `10 DU` square with an `icon_large` (`6 DU`) search icon in `text_secondary`; `accent` while focused. The loading spinner is `4.5 DU`. |
| Clear button | `10 DU` square at the end, shown only while the field has text. |

## Behavior

* The query is reported `search_debounce` (`300 ms`) after the last change.
* Enter reports the query at once.
* Clear empties the field, reports the empty query, and keeps the focus in the field.
* Escape clears a non-empty field.
* While `pending` is set, a spinner in the `pending` color replaces the search icon.
* The search itself is the caller's job; the field never filters anything.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `value` | string signal | required | Query. |
| `placeholder` | string | `search-field.placeholder` | Hint inside the empty field. |
| `on_search` | callback `(string)` | none | Receives the query. |
| `debounce_ms` | integer | `300` | Pause before reporting. |
| `pending` | bool | `false` | Shows the spinner. |
| `enabled` | bool | `true` | Enables input. |
| `label` | string | none | Visible label above the field; also the accessible name. |
| `aria_label` | string | `search-field.label` | Accessible name when there is no visible label. |
| `clear_label` | string | `search-field.clear` | Accessible name of the clear button. |
| `hint` | string | none | Helper text below the field. |
| `error` | string | none | Error text below the field; replaces the hint and draws the error outline. |
| `variant` | `outlined` or `borderless` | `outlined` | Field look, see [Field](field.md). |
| `icon`, `clear_icon` | icon | `search-field.search`, `field.clear` | Icon overrides. |

## Customization

SearchField is built on [Field](field.md) and has the same hooks.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `height` | DU | `--op-field-height` | `row_standard` (`10 DU`) |
| `padding_x` | DU | `--op-field-padding-x` | `pad_card` |
| `slot_size` | DU | `--op-field-slot-size` | `row_standard`; width of the search icon slot and the clear button |
| `gap` | DU | `--op-field-gap` | `gap_micro`; between label, box, hint and the outer slots |
| `font_size` | TU | `--op-field-font-size` | `type_body_size` |

| Color variable | Default |
| :--- | :--- |
| `--op-field-background` | `surface` |
| `--op-field-text` | `text_primary` |

| Slot | Place |
| :--- | :--- |
| `prepend` | Replaces the search icon. |
| `append` | Inside the box, before the clear button. |
| `before`, `after` | Outside the box. |

`class` and `style` apply to the root (label, box and hint).

### Strings

| Key | Default |
| :--- | :--- |
| `search-field.placeholder` | `Search` |
| `search-field.label` | `Search` |
| `search-field.clear` | `Clear search` |

### Icons

| Key | Default |
| :--- | :--- |
| `search-field.search` | `search` |
| `field.clear` | `close` |

## Accessibility

The field is a `search` landmark and the input has the `search` type.
