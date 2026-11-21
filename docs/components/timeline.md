# Timeline

Vertical sequence of dated milestones, status changes, or historical entries.
Each entry displays an indicator node with an optional icon and semantic tone.

Status: **draft** on web.

## Use

Use `Timeline` to display audit logs, deployment steps, shipment tracking, or change histories in chronological order.
Do not use `Timeline` for interactive linear wizard steps where user input is required; use [Stepper](stepper.md) instead.

## Geometry

| Property | Value |
| :--- | :--- |
| Node size | `6 DU` square (`0 DU` corner radius). |
| Connecting line | `0.25 DU` width (`--op-timeline-line-width`). |
| Entry gap | `4 DU` by default (`2 DU` in `dense` mode, `--op-timeline-gap`). |

## Behavior

* Entries connect via a continuous vertical line drawn along the rail.
* The line terminates at the last entry.
* The node dot supports semantic tones (`pending`, `success`, `warning`, `error`, `accent`) to indicate milestone status.

## API

### Timeline

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `dense` | bool | `false` | Reduces gap between entries. |
| `gap` | DU | `--op-timeline-gap` | Spacing between entries. |
| `line_width` | DU | `--op-timeline-line-width` | Width of the rail line. |
| `label` | string | `timeline.label` | Accessible label for the list. |
| `class` | string | none | CSS classes on the root element. |
| `style` | string | none | Inline CSS on the root element. |

### TimelineEntry

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `title` | string | none | Milestone title. |
| `date` | string | none | Date or timestamp string. |
| `tone` | string | `"default"` | Semantic tone (`"pending"`, `"success"`, `"warning"`, `"error"`, `"accent"`). |
| `icon` | Icon | none | Icon inside the node dot. |
| `title_slot` | Slot | none | Custom title slot. |
| `date_slot` | Slot | none | Custom date slot. |
| `icon_slot` | Slot | none | Custom node dot slot. |
| `class` | string | none | CSS classes on the entry element. |
| `style` | string | none | Inline CSS on the entry element. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `gap` | DU | `--op-timeline-gap` | `4 DU` |
| `line_width` | DU | `--op-timeline-line-width` | `0.25 DU` |

| Color variable | Default |
| :--- | :--- |
| `--op-timeline-line-color` | `var(--op-border)` |
| `--op-timeline-dot-bg` | `var(--op-surface-raised)` |
| `--op-timeline-dot-border` | `var(--op-border)` |
| `--op-timeline-dot-color` | `var(--op-text-primary)` |
| `--op-timeline-date-color` | `var(--op-text-secondary)` |
| `--op-timeline-title-color` | `var(--op-text-primary)` |
| `--op-timeline-body-color` | `var(--op-text-secondary)` |

`class` and `style` apply to the `<ol>` container on `Timeline` and `<li>` on `TimelineEntry`.

### Strings

| Key | Default |
| :--- | :--- |
| `timeline.label` | `"Timeline"` |

### Icons

Entries accept any icon from `operator_ui_tokens::icons`.

## Accessibility

`Timeline` is structured as an ordered list (`<ol>`) with an accessible landmark label (`aria-label`).
Nodes and connecting lines are decorative and marked with `aria-hidden="true"`.
