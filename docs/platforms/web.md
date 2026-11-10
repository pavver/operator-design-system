# Web (Leptos)

[web/operator-ui](../../web/operator-ui) is the component library for Leptos 0.8 in client-side rendering mode.
[web/operator-ui-demo](../../web/operator-ui-demo) is the gallery used to review every component and state.

## Gallery

Requirements: the `wasm32-unknown-unknown` target and [Trunk](https://trunkrs.dev).

```bash
rustup target add wasm32-unknown-unknown
cd web/operator-ui-demo
trunk serve
```

Open `http://127.0.0.1:8090/`.
The sidebar switches dark and light mode, forces the next backend result (success, error, or alternating), and sets the simulated latency.

## Usage

Wrap the application in `ThemeRoot`.
It sets the theme attributes and injects the token and component stylesheets once.

```rust
use operator_ui::prelude::*;

#[component]
fn Settings() -> impl IntoView {
    let name = RwSignal::new("Gate-North".to_string());
    view! {
        <ThemeRoot>
            <TextField
                value=name
                placeholder="Device name"
                on_save=AsyncAction::new(|value: String| async move {
                    // send `value` to the backend here
                    let _ = value;
                    ActionResult::Success
                })
            />
        </ThemeRoot>
    }
}
```

The application provides Cascadia Code through `@font-face`.
The gallery shows how in [demo.css](../../web/operator-ui-demo/demo.css).

## Implementation Notes

* The base outline is an inset `box-shadow`, which keeps the stroke inside the bounds without changing the element size.
* The result pulse is an `::after` layer with a 2x border; the result background is a `::before` layer that fades by opacity only.
* `data-state` holds the current state, `data-result` keeps the last result during the exit fade, and `data-fading` is present while the fade runs.
* The pending segment is an SVG rectangle with `pathLength="100"` and a `25 75` dash pattern animated clockwise.
* The TextField draws its outline on a `::before` layer above the trailing controls.
* The SegmentedControl moves the indicator with a wrapper element, so result states that disable color transitions never cancel the slide.
* The Slider prevents the default pointer-down action to stop text selection and native drag, which would otherwise cancel the drag.
