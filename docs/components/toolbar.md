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
| Title | Medium weight, `1 DU` horizontal padding, followed by a separator. |
| Separator | `0.25 DU` × `6 DU` `divider` with `1 DU` margins. |
| Commands | [IconButton](icon-button.md), `8 DU`; dense `6 DU`. |
| Overflow button | Same size as a command, `more_vert` icon, right after the last visible command. |
| End controls | After the free space, at the end of the bar; fields are `8 DU` tall. |

## Layout

```
[ Title | ⟳  ⏷  ⤓  ⋮            ( Search…     ) ]
          commands  overflow      end controls
```

## Behavior

* Commands are data, not free-form children, so each one can appear either as a button in the bar or as a row in the overflow menu.
* When the commands do not fit, they move into the overflow [Menu](menu.md) from the end, one at a time, and come back when space returns.
  The overflow button appears only when at least one command is in the menu.
* In the menu a command shows its icon and label; a separated command starts a group in both places.
* Commands marked `overflow` always live in the menu, after the commands that did not fit, as a separate group.
  Use them for rare or destructive commands.
* The title and the end controls never move into the menu.
* Each command button has a tooltip with its label.
* An async command shows its states on its own button.
  Run from the menu, its states show on the overflow button.
* Left and Right arrows move focus between the visible controls, except inside text inputs.

## API

`Toolbar`:

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `items` | list of `ToolbarItem` | required | Commands in order. |
| `title` | string | none | Title at the start. |
| `end` | view | none | Controls at the end that never overflow, such as a search field. |
| `label` | string | none | Accessible name. |
| `dense` | bool | `false` | Smaller variant. |

`ToolbarItem`:

| Builder | Description |
| :--- | :--- |
| `new(id, label, icon)` | Command; the label is the tooltip, the accessible name, and the menu text. |
| `on_click(callback)` | Runs the callback. |
| `on_async(action)` | Runs an async action with state feedback. |
| `separated()` | Starts a new group. |
| `overflow()` | Always in the overflow menu. |
| `danger()` | Destructive; error color in the menu. |
| `disabled()` | Not available. |

## Accessibility

The bar has the `toolbar` role and the given name.
The overflow button is named "More commands" and opens a `menu`.
