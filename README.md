# Operator Design System

A compact, technical design system for applications that monitor and control devices.
It looks like a terminal or a control panel: Cascadia Code everywhere, gray editor-like surfaces, warm signal colors, sharp corners, and honest feedback for every async action.

**Live gallery:** <https://pavver.github.io/operator-design-system/>

The repository contains three things:

* **The system:** principles, units, color, type, motion, and the shared state model, in [docs](docs).
* **Tokens:** one TOML file that generates CSS, Rust, and Kotlin sources.
* **Implementations:** hand-written components per platform, currently the web (Leptos).

## Components

| Component | Specification | Web (Leptos) |
| :--- | :--- | :--- |
| TextField | [text-field.md](docs/components/text-field.md) | Reviewed |
| Slider | [slider.md](docs/components/slider.md) | Reviewed |
| SegmentedControl | [segmented-control.md](docs/components/segmented-control.md) | Reviewed |
| ActionButton | [action-button.md](docs/components/action-button.md) | Draft (TextField dependency) |

Components are added one at a time, after each one has been reviewed in every state.

## Documentation

Foundations:

* [Principles](docs/foundations/principles.md)
* [Units: DU and TU](docs/foundations/units.md)
* [Color](docs/foundations/color.md)
* [Typography](docs/foundations/typography.md)
* [Iconography](docs/foundations/iconography.md)
* [Motion](docs/foundations/motion.md)
* [States and async feedback](docs/foundations/states.md)

Engineering:

* [Tokens and code generation](docs/tokens.md)
* [Porting to a new platform](docs/porting.md)
* [Web (Leptos)](docs/platforms/web.md)
* [Releasing](docs/releasing.md)

## Repository Layout

```
tokens/                  tokens.toml and SVG icons (source of truth)
tools/operator-tokens/   token generator
tools/import_material_symbols.py  icon importer
crates/operator-ui-tokens/  generated Rust tokens (no_std)
generated/kotlin/        generated Compose tokens
web/operator-ui/         Leptos component library
web/operator-ui-demo/    interactive gallery
docs/                    the design system itself
scripts/release.sh       release helper
```

## Quick Start

```bash
cargo run -p operator-tokens          # regenerate tokens after editing tokens/tokens.toml
cd web/operator-ui-demo && trunk serve # open http://127.0.0.1:8090/
```

## License

Code and documentation are dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
The bundled font and icon paths have their own licenses, see [NOTICE.md](NOTICE.md).
