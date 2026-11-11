# Web (Leptos)

[web/operator-ui](../../web/operator-ui) is the component library for Leptos 0.8 in client-side rendering mode.
[web/operator-ui-demo](../../web/operator-ui-demo) is the documentation site.
It has one page per foundation and component with live examples, and it is where every component is reviewed in every state.

## Documentation Site

Requirements: the `wasm32-unknown-unknown` target and [Trunk](https://trunkrs.dev).

```bash
rustup target add wasm32-unknown-unknown
cd web/operator-ui-demo
trunk serve
```

Open `http://127.0.0.1:8090/`.
The top bar switches dark and light mode.
Pages with async examples show a simulated backend bar that forces the next result (success, error, or alternating) and sets the latency.

The page text is not written in Rust.
The build script renders every file in [docs](..) to HTML, so each page shows the same Markdown that AI agents read.
Links between Markdown files become links between pages; links to other repository files open on GitHub.
To document a new component, add its Markdown file and register its examples in `src/pages/mod.rs` and its place in the sidebar in `src/main.rs`.

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
The documentation site shows how in [demo.css](../../web/operator-ui-demo/demo.css).

## Implementation Notes

* The base outline is an inset `box-shadow`, which keeps the stroke inside the bounds without changing the element size.
* The result pulse is an `::after` layer with a 2x border; the result background is a `::before` layer that fades by opacity only.
* `data-state` holds the current state, `data-result` keeps the last result during the exit fade, and `data-fading` is present while the fade runs.
* The pending segment is an SVG rectangle with `pathLength="100"` and a `25 75` dash pattern animated clockwise.
* The TextField draws its outline on a `::before` layer above the trailing controls.
* The SegmentedControl moves the indicator with a wrapper element, so result states that disable color transitions never cancel the slide.
* The Slider prevents the default pointer-down action to stop text selection and native drag, which would otherwise cancel the drag.
