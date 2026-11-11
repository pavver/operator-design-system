# Sheet

Panel that slides in from a screen edge over a scrim.
On the left it is the navigation drawer; on the right a details panel; at the bottom a sheet of actions.

Status: **draft** on web.

## Geometry

| Property | Value |
| :--- | :--- |
| Side sheet width | `size` (default `75 DU`), at most 85% of the viewport. |
| Bottom sheet height | Content height, at most `size` and 85% of the viewport. |
| Background | `surface`; a `0.25 DU` `border` on the edge facing the content. |
| Header | Optional, at least `12 DU`: title in `row_title` style, close button, `divider` below. |
| Handle | Bottom sheet only: `10 DU` × `1 DU` `border_strong` bar, `2 DU` from the top. |
| Scrim | Black at 55%. |
| Corners | `0 DU`. |

## Behavior

* The sheet slides in from its edge over `sheet_slide` (`200 ms`) with the standard curve while the scrim fades in.
* Opening moves focus into the sheet.
* Escape, a scrim click, or the close button closes it.
* The body scrolls on its own.
* While the sheet is open, the page behind it does not scroll, also under the scrim.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `open` | bool signal | required | Visibility. |
| `side` | Left, Right, or Bottom | Left | Edge. |
| `title` | string | none | Header and accessible name. |
| `size` | DU | `75` | Width or maximum height. |
| `children` | view | required | Content. |

## Accessibility

`dialog` role with `aria-modal`.
