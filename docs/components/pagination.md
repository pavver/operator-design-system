# Pagination

Navigation between numbered pages of a long list or table.

Status: **draft** on web.

## Geometry

| Property | Value |
| :--- | :--- |
| Buttons | `8 DU` tall, at least `8 DU` wide, `1 DU` apart. |
| Previous / next | [IconButton](icon-button.md) with `chevron_left` and `chevron_right`. |
| Current page | Editable field: `selection` background, `accent` bold text, `Selected` border; wide enough for the digits of the last page. |
| Current page while editing | `surface` background, `text_primary` text, `Focused` border, no focus outline. |
| Ellipsis | `…` in `text_secondary`, `8 DU` wide. |
| Numbers | Tabular figures. |

## Page window

The first and last pages are always shown.
Around the current page, `siblings` pages are shown on each side (default 1).
A gap of one page is filled with that page; a longer gap becomes an ellipsis.

Example for page 6 of 24: `1 … 5 6 7 … 24`.

## Type a Page

The current page is a field, so the operator can reach a distant page without clicking through the window.
Focusing it selects the number, so typing replaces it.
Only digits can be typed.
Enter goes to the typed page; a number past the end goes to the last page, and `0` goes to the first.
Escape restores the current page; leaving the field restores it without navigating.
In compact mode the field is followed by `/ total`.

## Behavior

* Previous is disabled on the first page; next is disabled on the last page.
* Compact mode replaces the numbers with the current page field and `/ 24` for narrow spaces.
* Pages are 1-based.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `page` | integer signal | required | Current page, from 1. |
| `total` | integer | required | Number of pages. |
| `siblings` | integer | `1` | Pages on each side of the current one. |
| `compact` | bool | `false` | Shows `page / total`. |

## Accessibility

A `navigation` landmark named "Pagination"; the current page has `aria-current="page"`.
The current page field is named "Page 6 of 24, type a page number" and opens the numeric keyboard on touch devices.
