# Porting to a New Platform

This guide lists what a platform implementation needs so that it looks and behaves like the reference.
The web library in [web/operator-ui](../web/operator-ui) is the current reference implementation.

## 1. Tokens

* Add an output for the platform to the token generator, or consume an existing one.
  Rust toolkits use [crates/operator-ui-tokens](../crates/operator-ui-tokens) directly.
* Expose DU and TU as the only length units, see [foundations/units.md](foundations/units.md).
* Load Cascadia Code from the unmodified font file.

## 2. Shared Primitives

Build these once before any component.
Each component then becomes mostly layout.

| Primitive | Responsibility | Web reference |
| :--- | :--- | :--- |
| `ControlState`, `ActionResult` | State and result types. | `src/core/` |
| Async action | A handler `(value) -> future of ActionResult`. | `AsyncAction` |
| Job token | Ignores results of superseded requests. | `JobToken` |
| State visual | Tracks the current state, the last result, and the exit fade. | `use_state_visual` |
| State border | Base outline, result pulse, result background, pending segment. | `.op-sb` in CSS |
| Content tone | Text and icon colors that follow the same result rules. | `.op-tone` in CSS |
| Ripple | Bounded press ripple from the pointer position. | `spawn_ripple` |
| Spinner | Rotating 270° arc. | `ProgressIndicator` |

The state border rules are specified in [foundations/states.md](foundations/states.md).
Draw the outline inside the bounds.
On canvas-based toolkits, inset the stroke rectangle by half the stroke width so edges land on whole pixels.

## 3. Components

Implement each component from its specification in [components](components).
The specification defines the behavior; the web code shows one way to achieve it.

## 4. Verification Checklist

For every component, check each item in dark and light mode:

* [ ] Every state in the specification renders as described.
* [ ] Sizes match the DU values at 100% and at a large text scale; text never clips.
* [ ] Result colors appear instantly and fade out over `500 ms` with no neutral frame.
* [ ] Pending blocks input; Success and Error do not.
* [ ] An error rolls the value back with the same animation as the forward change.
* [ ] A newer request discards the result of an older one.
* [ ] Keyboard focus and accessible names work.
* [ ] Rapid repeated input does not leave the component stuck in any state.

## 5. Status Tracking

Update the component table in the [README](../README.md) when a platform implementation is reviewed.
