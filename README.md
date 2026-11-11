<img src="assets/brand/logo-animated.svg" width="96" height="96" alt="Operator UI logo">

# Operator Design System

A compact, technical design system for applications that monitor and control devices.
It looks like a terminal or a control panel: Cascadia Code everywhere, gray editor-like surfaces, warm signal colors, sharp corners, and honest feedback for every async action.

**Documentation and live examples:** <https://pavver.github.io/operator-design-system/>

The site renders the same Markdown files from [docs](docs) that AI agents read, next to live examples of every component.

The repository contains three things:

* **The system:** principles, units, color, type, motion, and the shared state model, in [docs](docs).
* **Tokens:** one TOML file that generates CSS, Rust, and Kotlin sources.
* **Implementations:** hand-written components per platform, currently the web (Leptos).

## Components

| Component | Specification | Web (Leptos) |
| :--- | :--- | :--- |
| ActionButton | [action-button.md](docs/components/action-button.md) | Reviewed |
| IconButton | [icon-button.md](docs/components/icon-button.md) | Reviewed |
| AsyncSwitch | [switch.md](docs/components/switch.md) | Reviewed |
| AsyncCheckbox | [checkbox.md](docs/components/checkbox.md) | Reviewed |
| CheckboxGroup | [checkbox-group.md](docs/components/checkbox-group.md) | Reviewed |
| TextField | [text-field.md](docs/components/text-field.md) | Reviewed |
| Slider | [slider.md](docs/components/slider.md) | Reviewed |
| Stepper | [stepper.md](docs/components/stepper.md) | Reviewed |
| SegmentedControl | [segmented-control.md](docs/components/segmented-control.md) | Reviewed |
| DropdownMenu | [dropdown-menu.md](docs/components/dropdown-menu.md) | Reviewed |
| ListItem | [list-item.md](docs/components/list-item.md) | Reviewed |
| StatusBadge | [status-badge.md](docs/components/status-badge.md) | Reviewed |
| Avatar | [avatar.md](docs/components/avatar.md) | Reviewed |
| Header | [header.md](docs/components/header.md) | Reviewed |
| ConfirmDialog | [confirm-dialog.md](docs/components/confirm-dialog.md) | Reviewed |
| ConsoleTerminal | [console-terminal.md](docs/components/console-terminal.md) | Reviewed |
| MarqueeText | [marquee-text.md](docs/components/marquee-text.md) | Reviewed |
| ThemeToggle | [theme-toggle.md](docs/components/theme-toggle.md) | Reviewed |
| Icon, ProgressIndicator, Divider, Card | [primitives.md](docs/components/primitives.md) | Reviewed |
| Tooltip | [tooltip.md](docs/components/tooltip.md) | Reviewed |
| Toast | [toast.md](docs/components/toast.md) | Reviewed |
| Banner | [banner.md](docs/components/banner.md) | Reviewed |
| LinearProgress | [linear-progress.md](docs/components/linear-progress.md) | Reviewed |
| Skeleton | [skeleton.md](docs/components/skeleton.md) | Draft |
| EmptyState | [empty-state.md](docs/components/empty-state.md) | Draft |
| TextArea | [text-area.md](docs/components/text-area.md) | Reviewed |
| SearchField | [search-field.md](docs/components/search-field.md) | Reviewed |
| RadioGroup | [radio-group.md](docs/components/radio-group.md) | Reviewed |
| Combobox | [combobox.md](docs/components/combobox.md) | Reviewed |
| Chip | [chip.md](docs/components/chip.md) | Reviewed |
| Menu, MenuButton, ContextMenu | [menu.md](docs/components/menu.md) | Reviewed |
| Dialog | [dialog.md](docs/components/dialog.md) | Reviewed |
| Tabs | [tabs.md](docs/components/tabs.md) | Reviewed |
| Sheet | [sheet.md](docs/components/sheet.md) | Draft |
| Pagination | [pagination.md](docs/components/pagination.md) | Reviewed |
| DataTable | [data-table.md](docs/components/data-table.md) | Draft |
| Counter | [counter.md](docs/components/counter.md) | Draft |
| PropertyList | [property-list.md](docs/components/property-list.md) | Draft |
| Accordion, Collapsible | [accordion.md](docs/components/accordion.md) | Draft |
| CodeBlock | [code-block.md](docs/components/code-block.md) | Draft |
| Kbd | [kbd.md](docs/components/kbd.md) | Reviewed |
| Toolbar | [toolbar.md](docs/components/toolbar.md) | Draft |
| SplitPane | [split-pane.md](docs/components/split-pane.md) | Reviewed |
| ScrollArea | [scroll-area.md](docs/components/scroll-area.md) | Reviewed |
| Scrollbar | [scrollbar.md](docs/components/scrollbar.md) | Reviewed |

New components start as **Draft** and become **Reviewed** after they have been checked in every state.

## Cargo Features

The `operator-ui` crate provides one feature per component.
The `reviewed` feature is enabled by default and includes every reviewed component.
The `draft` feature includes components still in progress.
The `full` feature enables all reviewed and draft components.
Applications can opt out of default features and select individual components:

```toml
operator-ui = { git = "https://github.com/pavver/operator-design-system", default-features = false, features = ["action-button", "toast"] }
```

Pin a release with `tag = "vX.Y.Z"`; component features are available from the first release after v0.5.0.

Unused components ship neither code nor CSS in the final build.

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
web/operator-ui-demo/    documentation site with live examples (renders docs/)
docs/                    the design system itself
assets/brand/            logo: still mark (also the favicon) and animated version
scripts/release.sh       release helper
```

## Quick Start

```bash
cargo run -p operator-tokens          # regenerate tokens after editing tokens/tokens.toml
cd web/operator-ui-demo && trunk serve # open http://127.0.0.1:8090/ (documentation site)
```

## License

Code and documentation are dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
The bundled font and icon paths have their own licenses, see [NOTICE.md](NOTICE.md).
