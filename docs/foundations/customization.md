# Customization

How an application adapts components without changing the design system.
The model follows established web frameworks (Quasar, Vuetify): shared prop groups, named slots, per-component metric variables, replaceable strings and icons, and public building blocks.
Every mechanism below is part of a component's specification, so each platform implements the same hooks.

Status: **draft** on web.

## Principles

* The defaults are the design system; customization changes the instance, never the rules.
  Units stay DU and TU, corners stay square, states follow [states.md](states.md).
* An application never targets internal class names or element structure.
  Only the hooks listed in a component's **Customization** section are stable.
* A hook that one component has, every component of the same family has with the same name.

## 1. Root class and style

Every component accepts `class` and `style` on its root element.
`class` is added to the component's own classes; `style` is appended after the component's own inline style, so it wins.

| Platform | Equivalent |
| :--- | :--- |
| Web | `class`, `style` props on the root element. |
| Compose | `modifier` parameter. |
| egui | none; use the metric props. |

## 2. Metric props

Every spacing and size a component draws is a prop.
A metric prop is a number in DU (spatial values) or TU (font sizes, line heights).
An unset prop keeps the design-system value.

On the web each metric prop is backed by a CSS custom property named `--op-<family>-<metric>`, read with the token as fallback, for example `height: var(--op-field-height, var(--op-dim-row-standard))`.
The prop writes the variable on the component root, so:

* one instance changes through the prop (`height=8.0`);
* a whole subtree changes through the variable in application CSS (`.my-table { --op-field-height: calc(var(--op-du) * 8); }`);
* the variables of a family are shared: every field reads `--op-field-*`, every button `--op-button-*`.

Colors are not props.
A component reads its colors through `--op-<family>-<role>` variables with token fallbacks, so an application can retint a subtree with tokens it owns, while every theme keeps the same token set.

Each component specification lists its metric props and color variables in a **Customization** table.

## 3. Slots

A slot is a named place where the caller inserts its own content.
Slot names are shared across families:

| Family | Slots |
| :--- | :--- |
| Fields | `prepend`, `append` (inside the box), `before`, `after` (outside the box), `label_slot`, `hint_slot`. |
| Buttons | `leading`, `trailing`, `content` (replaces the text). |
| Lists and menus | `option` (scoped: receives the item), `empty`, `before_options`, `after_options`, `header`, `footer`. |
| Containers (dialogs, sheets, cards, panels) | `header`, `actions`, `footer`, default children. |

A scoped slot receives data from the component, for example the option it renders.

| Platform | Slot | Scoped slot |
| :--- | :--- | :--- |
| Web | `Slot` (closure or `ViewFn`) | `ScopedSlot<T>` (`Callback<T, AnyView>`) |
| Compose | `@Composable () -> Unit` | `@Composable (T) -> Unit` |
| egui | `FnOnce(&mut Ui)` | `FnMut(&mut Ui, &T)` |

## 4. Strings

Components never show hard-coded text.
Every built-in text (button labels, accessible names, placeholders, month names) has a key such as `search-field.clear` and an English default.
Resolution order: the component prop, then the application's lookup for the key, then the English default.

The application provides one lookup for the whole tree through the theme root (`ThemeRoot strings=...` on the web); it reads its current language inside the lookup, so every string follows a language switch.
Each specification lists its keys in the **Strings** table.

## 5. Icons

Built-in icons are semantic keys (`field.clear`, `search-field.search`, `menu.submenu`) with a default symbol.
Resolution order: the component prop, then the application's icon lookup for the key (`ThemeRoot icons=...`), then the default.
Each specification lists its keys in the **Icons** table.

## 6. Building blocks

The primitives that components are built from are public, so an application can build its own component that looks and behaves like a native one:

* `Field`: the field shell (label, state-bordered box with slots, hint and error).
* `Popover`: anchored overlay placement.
* `VirtualList`: windowed list rendering.
* State visuals (`use_state_visual`), `AsyncAction`, `JobToken`, ripple, spinner.

## 7. Style cascade (web)

All component CSS is inside one cascade layer, `@layer operator`.
Unlayered application CSS therefore always wins over component styles regardless of selector specificity, and no `!important` is needed.
An application that has its own global element rules (`button { ... }`) should put them in a layer declared before `operator` (`@layer reset, operator;`) so they do not override components.
