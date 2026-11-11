# EmptyState

Message in place of content that does not exist yet or was filtered out.
It explains why the area is empty and what to do next.

Status: **reviewed** on web.

## Use

* Empty lists, tables, search results, and panels.
* Say what would normally be here and how to get it; avoid jokes and illustrations.

## Geometry

| Property | Value |
| :--- | :--- |
| Layout | Centered column. |
| Padding | `10 DU` vertical and `4 DU` horizontal; compact: `6 DU` × `3 DU`. |
| Icon | `12 DU`, compact `8 DU`, `text_disabled`, `2 DU` above the title. |
| Title | `row_title` style, `text_primary`. |
| Description | Body style, `text_secondary`, at most `90 DU` wide. |
| Actions | `3 DU` below the text, centered, `2 DU` apart. |

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `title` | string | required | What is missing. |
| `description` | string | none | Why, and what to do. |
| `icon` | icon | `inbox` | Symbol of the content. |
| `actions` | view | none | One or two buttons. |
| `compact` | bool | `false` | Smaller variant for panels and table bodies. |
