# SearchField

Single-line input for filtering and searching.
It reports the query after a short pause, so results follow the typing without a request per key.

Status: **reviewed** on web.

## Geometry

The field has the shape of [TextField](text-field.md) with a leading slot:

| Property | Value |
| :--- | :--- |
| Height | `10 DU`. |
| Leading slot | `10 DU` square with a `6 DU` (`icon_large`) search icon in `text_secondary`; `accent` while focused. The loading spinner is `4.5 DU`. |
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
| `placeholder` | string | `"Search"` | Hint. |
| `on_search` | callback `(string)` | none | Receives the query. |
| `debounce_ms` | integer | `300` | Pause before reporting. |
| `pending` | bool | `false` | Shows the spinner. |
| `enabled` | bool | `true` | Enables input. |
| `label` | string | `"Search"` | Accessible name. |

## Accessibility

The field is a `search` landmark and the input has the `search` type.
