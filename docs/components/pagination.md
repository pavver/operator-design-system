# Pagination

Navigation between numbered pages of a long list or table.

Status: **draft** on web.

## Geometry

| Property | Value |
| :--- | :--- |
| Buttons | `8 DU` tall, at least `8 DU` wide, `1 DU` apart. |
| Previous / next | [IconButton](icon-button.md) with `chevron_left` and `chevron_right`. |
| Current page | `selection` background, `accent` bold text, `Selected` border. |
| Ellipsis | `…` in `text_secondary`, `8 DU` wide. |
| Numbers | Tabular figures. |
| Jump field | "Go to" `caption` label in `text_secondary`, then an `8 DU` tall field with a `0.25 DU` state border, `2.5 DU` side padding, wide enough for the digits of the last page; `2 DU` after the next button. |
| Jump placeholder | The current page number in `text_disabled`. |

## Page window

The first and last pages are always shown.
Around the current page, `siblings` pages are shown on each side (default 1).
A gap of one page is filled with that page; a longer gap becomes an ellipsis.

Example for page 6 of 24: `1 … 5 6 7 … 24`.

## Jump to a Page

The jump field lets the operator reach a distant page without clicking through the window.
It is shown from 20 pages by default; `jump` forces it on or off.
Only digits can be typed.
Enter goes to the typed page; a number past the end goes to the last page, and `0` goes to the first.
Escape clears the field; leaving the field clears it without navigating.
The field keeps the `Focused` border while it has focus and shows hover feedback like other controls.

## Behavior

* Previous is disabled on the first page; next is disabled on the last page.
* Compact mode replaces the numbers with `6 / 24` for narrow spaces.
* Pages are 1-based.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `page` | integer signal | required | Current page, from 1. |
| `total` | integer | required | Number of pages. |
| `siblings` | integer | `1` | Pages on each side of the current one. |
| `compact` | bool | `false` | Shows `page / total`. |
| `jump` | bool | from 20 pages | Shows the jump field. |

## Accessibility

A `navigation` landmark named "Pagination"; the current page has `aria-current="page"`.
The jump field is named "Go to page" and opens the numeric keyboard on touch devices.
