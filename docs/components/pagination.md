# Pagination

Navigation between numbered pages of a long list or table.

Status: **reviewed** on web.

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
| `gap` | DU | `pad_tight` | Spacing between page controls. |
| `page_size` | DU | `row_compact` | Size of page buttons. |
| `page_padding_x` | DU | `pad_tight` | Horizontal padding of page buttons. |
| `compact_gap` | DU | `pad_small` | Gap in compact mode. |
| `compact_padding_x` | DU | `pad_tight` | Horizontal padding in compact mode. |
| `label` | string | `pagination.label` | Accessible name of the pagination container. |
| `prev_label` | string | `pagination.previous` | Accessible label of the previous button. |
| `next_label` | string | `pagination.next` | Accessible label of the next button. |
| `page_label` | string | `pagination.page` | Template for page buttons. |
| `current_page_label` | string | `pagination.current_page` | Template for the current page input. |
| `prev_icon` | icon | `pagination.previous` | Previous button icon override. |
| `next_icon` | icon | `pagination.next` | Next button icon override. |
| `before` | slot | none | Slot before pagination elements. |
| `after` | slot | none | Slot after pagination elements. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `gap` | DU | `--op-pagination-gap` | `pad_tight` |
| `page_size` | DU | `--op-pagination-page-size` | `row_compact` |
| `page_padding_x` | DU | `--op-pagination-page-padding-x` | `pad_tight` |
| `compact_gap` | DU | `--op-pagination-compact-gap` | `pad_small` |
| `compact_padding_x` | DU | `--op-pagination-compact-padding-x` | `pad_tight` |

| Color variable | Default |
| :--- | :--- |
| `--op-pagination-page-background` | `surface` |
| `--op-pagination-page-text` | `text_primary` |
| `--op-pagination-current-background` | `selection` |
| `--op-pagination-current-text` | `accent` |
| `--op-pagination-gap-text` | `text_secondary` |
| `--op-pagination-compact-text` | `text_secondary` |

| Slot | Place |
| :--- | :--- |
| `before` | Before previous page button. |
| `after` | After next page button. |

`class` and `style` apply to the root nav element.

### Strings

| Key | Default |
| :--- | :--- |
| `pagination.label` | `Pagination` |
| `pagination.previous` | `Previous page` |
| `pagination.next` | `Next page` |
| `pagination.page` | `Page {n}` |
| `pagination.current_page` | `Page {page} of {total}, type a page number` |

### Icons

| Key | Default |
| :--- | :--- |
| `pagination.previous` | `chevron_left` |
| `pagination.next` | `chevron_right` |

## Accessibility

A `navigation` landmark named "Pagination"; the current page has `aria-current="page"`.
The current page field is named "Page 6 of 24, type a page number" and opens the numeric keyboard on touch devices.
