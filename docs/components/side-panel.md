# SidePanel

Titled panel at a screen edge that collapses to a narrow strip, such as a palette, an outline or a property panel next to a canvas.

Status: **draft** on web.

## Geometry

| Property | Value |
| :--- | :--- |
| Width | `75 DU` by default; collapsed `row_standard` (`10 DU`). |
| Header | `row_standard` tall: the title in `section_title` style, optional actions, and a `row_compact` [IconButton](icon-button.md) that collapses the panel; `0.25 DU` `divider` below. |
| Strip | Only the expand button, at the top. |
| Edge | `0.25 DU` `divider` on the edge facing the content. |
| Body | Fills the rest and scrolls. |

## Behavior

* The collapse button points to the panel's edge (`chevron_left` for a start panel); the expand button points away from it.
* Width changes animate with the `expand` motion.
* The collapsed state is a signal, so the application can remember it.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `title` | string signal | required | Header text and accessible name. |
| `collapsed` | bool signal | required | Collapsed state. |
| `side` | `PanelSide` | `Start` | `Start` or `End`. |
| `width` | DU | `75` | Expanded width. |
| `actions` | view | none | Header actions before the collapse button. |
| `children` | view | required | Panel content. |
