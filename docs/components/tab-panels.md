# TabPanels

Animated content containers synchronized with a selected tab index.

Status: **draft** on web.

## Use

Use TabPanels together with [Tabs](tabs.md) to render tab content panels with smooth enter transitions.
It supports conditional mounting as well as keep-alive mode for preserving form state across switches.

## Geometry

| Property | Default metric | Variable |
| :--- | :--- | :--- |
| Padding | `0 DU` | `--op-tab-panels-padding` |

## Behavior

* Only the panel whose index matches the active selection value is visible.
* In animated mode, entering panels execute a subtle fade and vertical slide animation.
* If `keep_alive` is false, inactive panels are unmounted from the DOM.
* If `keep_alive` is true, inactive panels remain mounted with `display: none`.

## API

### TabPanels

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `value` | usize signal | required | Active tab index driving panel selection. |
| `animated` | bool signal | `true` | Enables animated transitions when changing panels. |
| `keep_alive` | bool | `false` | Keeps inactive panels mounted in the DOM. |
| `padding` | DU metric | none | Inner padding of the panels container. |

### TabPanel

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `index` | usize | required | Index corresponding to the tab item. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `padding` | DU | `--op-tab-panels-padding` | `0 DU` |

| Color variable | Default |
| :--- | :--- |
| `--op-tab-panels-background` | `transparent` |
| `--op-tab-panels-text` | `text_primary` |

## Accessibility

* Panels carry `role="tabpanel"` and `tabindex="0"`.
* Inactive keep-alive panels are hidden from assistive technology via `display: none`.
