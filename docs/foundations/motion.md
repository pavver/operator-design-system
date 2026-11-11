# Motion

Motion exists only to explain what happened.
The idle interface is still.

## Durations

All values are in milliseconds and live in the `[motion]` table of [tokens.toml](../../tokens/tokens.toml).

| Token | Value | Use |
| :--- | :--- | :--- |
| `state_color` | `150` | Color change between ordinary states. |
| `result_fade` | `500` | Exit fade of a Success or Error color. |
| `result_pulse_half` | `400` | Half period of the result border pulse. |
| `result_hold` | `800` | How long a component keeps its own result state. |
| `local_flash_icon` | `500` | Result icon phase of a button flash. |
| `local_flash_tail` | `300` | Colored label phase after the icon. |
| `pending_lap` | `2000` | One lap of the pending border segment. |
| `spinner_lap` | `900` | One turn of a spinner. |
| `pending_indicator_delay` | `700` | Pending time before a control replaces its content with the spinner. |
| `pending_indicator_fade` | `200` | Fade-in of the pending spinner. |
| `segment_slide` | `150` | Sliding indicators and value jumps. |
| `dialog_enter` | `150` | Dialog entry. |
| `tooltip_hide_delay` | `800` | Delay before a value tooltip hides. |
| `alarm_flash_half` | `250` | Half period of alarm flashing. |
| `tooltip_show_delay` | `500` | Hover time before a [Tooltip](../components/tooltip.md) opens. |
| `long_press` | `500` | Touch hold that counts as a long press. |
| `toast_duration` | `4000` | Visible time of a [Toast](../components/toast.md). |
| `toast_error_duration` | `8000` | Visible time of an error toast. |
| `sheet_slide` | `200` | [Sheet](../components/sheet.md) entry. |
| `expand` | `200` | Height change of a [Collapsible](../components/accordion.md). |
| `skeleton_lap` | `1500` | One sweep of the [Skeleton](../components/skeleton.md) highlight. |
| `search_debounce` | `300` | Typing pause before a [SearchField](../components/search-field.md) reports. |
| `overscroll_release` | `300` | Spring-back of [stretch overscroll](../components/scroll-area.md). |

## Curves

| Name | Definition | Use |
| :--- | :--- | :--- |
| Standard | `cubic-bezier(0.4, 0, 0.2, 1)` | Ordinary state colors, value jumps. |
| Smooth | `cubic-bezier(0.45, 0, 0.55, 1)` (smoothstep) | Result exit fades. |
| Linear | `linear` | Indicator slides, spinners, pending laps. |

## Rules

* Result colors appear instantly and leave with a `500 ms` smoothstep fade.
* Switching directly between Success and Error changes the color instantly, without a fade.
* Border, background, text, and icon transitions of one control start in the same frame.
* A movement that is undone (rollback) uses the same duration and curve as the movement that caused it.
* Movement follows the pointer exactly during a drag and never animates behind it.
