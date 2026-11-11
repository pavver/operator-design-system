# Accordion

Sections with header rows that show or hide their content.
A standalone section is a Collapsible; an Accordion keeps only one section open.

Status: **draft** on web.

## Geometry

| Property | Value |
| :--- | :--- |
| Header | At least `10 DU`; `2 DU` start and `3 DU` end padding. |
| Chevron | `4 DU` `chevron_right` in `text_secondary`; rotated 90° and `accent` when open. |
| Title | Medium weight; optional subtitle in `caption` style. |
| Trailing | Optional content at the end, such as a [Counter](counter.md). |
| Content | Indented to the title; `1 DU` top and `3 DU` bottom padding. |
| Accordion | `0.25 DU` `border` outline, `divider` between sections. |

## Behavior

* A click on the header toggles the section.
* The content height animates over `expand` (`200 ms`) with the standard curve; the chevron rotates with it.
* Closed content stays mounted but is inert: it is not focusable and is hidden from assistive technology.
* In an Accordion, opening a section closes the open one.

## API

| Component | Properties |
| :--- | :--- |
| `Accordion` | `children`: sections. |
| `Collapsible` | `title`, `subtitle`, `open` (external signal), `initially_open`, `trailing`, `children`. |

## Accessibility

The header is a button with `aria-expanded` and `aria-controls` pointing at the content region.
