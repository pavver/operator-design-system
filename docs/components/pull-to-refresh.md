# PullToRefresh

Touch-driven pull gesture container for refreshing views asynchronously with Operator states.

Status: **draft** on web.

## Use

Use PullToRefresh to enable mobile and touch-friendly view refreshes.
It detects downward swipes originating from the top of the container, shows directional pull guidance, and triggers an asynchronous refresh action.

## Geometry

| Property | Default metric | Variable |
| :--- | :--- | :--- |
| Pull threshold | `15 DU` | `--op-pull-to-refresh-threshold` |

## Behavior

* Only activates when the container's scroll position is at the very top.
* Pulling downward expands the top puller area with damping resistance.
* If pulled past the threshold, the indicator flips and informs the user to release.
* Upon release past the threshold, the `on_refresh` action runs.
* While refreshing, a spinner appears and the puller remains fixed until the action completes.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `on_refresh` | `AsyncAction` | required | Handler executed to perform the refresh operation. |
| `enabled` | bool signal | `true` | Enables or disables gesture tracking. |
| `threshold` | DU metric | `15 DU` | Pull distance required to trigger refresh. |
| `pull_label` | string | `pull-to-refresh.pull` | Guidance text while pulling down. |
| `release_label` | string | `pull-to-refresh.release` | Guidance text when threshold is reached. |
| `refreshing_label` | string | `pull-to-refresh.refreshing` | Status text while refresh is active. |
| `puller` | slot | none | Custom slot to replace the default puller header. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `threshold` | DU | `--op-pull-to-refresh-threshold` | `15 DU` |

## Strings

| Key | Default (en) | Description |
| :--- | :--- | :--- |
| `pull-to-refresh.pull` | `Pull to refresh` | Initial pull instruction. |
| `pull-to-refresh.release` | `Release to refresh` | Instruction when pull threshold is reached. |
| `pull-to-refresh.refreshing` | `Refreshing…` | Loading status indicator. |

## Icons

| Key | Default | Description |
| :--- | :--- | :--- |
| `icons::ARROW_DOWNWARD` | Material arrow downward | Arrow indicating pull direction. |

## Accessibility

* Status text informs users of current gesture progress and refresh outcome.
