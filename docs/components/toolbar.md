# Toolbar

Horizontal bar of commands for the view below it.

Status: **draft** on web.

## Geometry

| Property | Value |
| :--- | :--- |
| Height | At least `10 DU`; dense `8 DU`. |
| Padding | `1 DU` horizontal. |
| Gap | `1 DU`; dense `0.5 DU`. |
| Background | `surface` with a `0.25 DU` `border` line below. |
| Title | Medium weight, `1 DU` horizontal padding. |
| Separator | `0.25 DU` × `6 DU` `divider` with `1 DU` margins. |
| Spacer | Takes the free space and pushes the following controls to the end. |
| Buttons and fields | `8 DU` tall inside the bar. |

## Behavior

* Controls keep their own states; the toolbar only arranges them.
* Content that does not fit scrolls horizontally without a visible scrollbar.
  Put rarely used commands into a [MenuButton](menu.md) at the end instead of letting the bar overflow.
* Left and Right arrows move focus between the controls, except inside text inputs.

## API

| Component | Properties |
| :--- | :--- |
| `Toolbar` | `label`, `dense`, `children`. |
| `ToolbarTitle` | `text`. |
| `ToolbarSeparator` | none. |
| `ToolbarSpacer` | none. |

## Accessibility

The bar has the `toolbar` role and the given name.
