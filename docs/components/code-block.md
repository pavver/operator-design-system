# CodeBlock

Monospaced block for code, configuration, and log output, with a copy button.

Status: **draft** on web.

## Geometry

| Property | Value |
| :--- | :--- |
| Background | `surface_alt` with a `0.25 DU` `border` outline. |
| Header | Optional, at least `8 DU`: language label in uppercase `caption`, copy button at the end, `divider` below. |
| Body | `console` style, `3 DU` padding. |
| Line numbers | `8 DU` column in `text_disabled`, end-aligned, not selectable. |
| Height | Content height, or `max_height` and then scrolling. |

## Behavior

* Long lines scroll horizontally; with `wrap` they wrap and break anywhere.
* The copy button copies the whole text as an async action with its success or error state.
* The header is hidden when there is no language and copying is off.
* Tabs are four columns wide.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `code` | string | required | Content. |
| `language` | string | none | Header label. |
| `line_numbers` | bool | `false` | Numbers each line. |
| `wrap` | bool | `false` | Wraps long lines. |
| `copy` | bool | `true` | Shows the copy button. |
| `max_height` | DU | none | Height limit. |
