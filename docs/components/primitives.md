# Primitives

Small building blocks used by other components.

Status: **reviewed** on web.

## Icon

* Renders one icon from [tokens/icons](../../tokens/icons), see [iconography.md](../foundations/iconography.md).
* Size in DU, `icon_large` (`6 DU`) by default.
* Uses the current content color unless a color is given.
* Without a label it is hidden from assistive technology; with a label it is an image with that name.

## ProgressIndicator

* A circular arc covering 75% of the circle, rotating once per `spinner_lap` (`900 ms`), linear.
* Size and stroke in DU; `6 DU` with a `0.5 DU` stroke by default.
* Inside controls it uses `spinner_stroke` (`0.375 DU`) and the `pending` color.
* Round line caps.

## Divider

* A horizontal line, full width, `0.25 DU` thick, in `divider`.

## Card

* A rectangular container with `pad_card` (`3 DU`) padding and a `0.25 DU` `border` outline.
* `surface` background by default; `surface_alt` for sunken displays.
* Corner radius `0 DU`, no shadow.
