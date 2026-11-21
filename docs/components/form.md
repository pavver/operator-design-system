# Form

Form validation container through registered FormField boundaries.
Groups fields, coordinates validation on submit or blur, focuses the first invalid field, and manages reset cycles.

Status: **draft** on web.

## Use

* Submitting user input with coordinated multi-field validation.
* Resetting form state and autofocusing erroneous inputs.

## Geometry

| Property | Value |
| :--- | :--- |
| Gap | `gap_small` (`2 DU`). |
| Padding | `pad_card` (`3 DU`). |
| Corner radius | `0 DU`. |

## Behavior

* Submitting the form validates all registered child fields in order.
* If validation fails, submission halts and focus moves to the first invalid field.
* If valid, the async `on_submit` action is invoked while the form enters a `Pending` state.
* Resetting the form clears errors and restores initial field values.
* While pending, the entire fieldset is marked `inert` and disabled.

## API

### Form

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `children` | `Children` | required | Child elements containing form fields. |
| `on_submit` | `AsyncAction` | none | Async submission handler. |
| `on_reset` | callback `()` | none | Reset event callback. |
| `on_validation` | callback `(bool)` | none | Emitted with overall validation outcome. |
| `enabled` | bool | `true` | Enables form submission and inputs. |
| `label` | string | `form.label` | Accessible label for the form. |
| `submit_label` | string | `form.submit` | Default submit button text. |
| `reset_label` | string | `form.reset` | Default reset button text. |
| `header` | `Slot` | none | Header content slot. |
| `actions` | `Slot` | none | Custom action buttons slot replacing default buttons. |
| `footer` | `Slot` | none | Footer content slot. |
| `gap` | DU metric | `gap_small` | Spacing between form fields and actions. |
| `padding` | DU metric | `pad_card` | Padding inside the form. |
| `class`, `style` | string | none | Root hooks. |

### FormField

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `value` | string signal | required | Signal of the field value. |
| `rules` | `Vec<ValidationRule>` | required | List of validation functions returning `Option<String>`. |
| `children` | `Children` | required | Wrapped input control. |
| `error` | `Option<RwSignal<Option<String>>>` | internal | Error text signal for the field. |
| `enabled` | bool | `true` | Enables the field boundary. |
| `gap` | DU metric | `gap_small` | Internal spacing. |
| `class`, `style` | string | none | Root hooks. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `gap` | DU | `--op-form-gap` | `gap_small` (`2 DU`) |
| `padding` | DU | `--op-form-padding` | `pad_card` (`3 DU`) |

| Color variable | Default |
| :--- | :--- |
| `--op-form-text` | `text_primary` |
| `--op-form-background` | `surface` |
| `--op-form-border` | `border` |
| `--op-form-error` | `error` |
| `--op-form-submit-background` | `accent` |
| `--op-form-submit-text` | `on_accent` |
| `--op-form-submit-border` | `accent` |
| `--op-form-reset-background` | `surface` |
| `--op-form-reset-text` | `text_secondary` |

| Slot | Place |
| :--- | :--- |
| `header` | Above the fields area. |
| `actions` | Button row area. |
| `footer` | Below the actions area. |

`class` and `style` apply to the root form element.

### Strings

| Key | Default |
| :--- | :--- |
| `form.label` | `Form` |
| `form.submit` | `Submit` |
| `form.reset` | `Reset` |

### Icons

Form does not define built-in icons.

## Accessibility

The form uses native `<form>` semantics with `novalidate` to prevent browser validation conflicts.
Fieldset elements become `disabled` and `inert` during async submission.
Overall error messages are reported with `role="status"`.
