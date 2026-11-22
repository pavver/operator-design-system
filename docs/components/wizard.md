# Wizard

Multi-step guided workflow with numbered step headers, step validation, and asynchronous finish.

Status: **draft** on web.

## Use

Use Wizard to guide users through structured, sequential tasks such as account creation, setup wizards, or checkout flows.
Do not confuse with the numeric [Stepper](stepper.md) component, which handles `- value +` numeric input adjustments.

## Geometry

| Property | Default metric | Variable |
| :--- | :--- | :--- |
| Step size | `8 DU` | `--op-wizard-step-size` |
| Header gap | `2 DU` | `--op-wizard-header-gap` |
| Padding | `4 DU` | `--op-wizard-padding` |
| Content gap | `4 DU` | `--op-wizard-content-gap` |

## Behavior

* Steps display numbers or completed check icons connected by lines.
* Users can click directly on completed steps to return to earlier sections.
* Clicking Next triggers the optional `on_validate_step` callback for the current step.
* If validation fails, the error message is displayed below the step content, the Next button flashes the Error result state, and the wizard remains on the current step.
* The validation error clears when the step becomes valid or when the active step changes.
* If validation succeeds or is absent, the wizard advances to the next step.
* The final step shows a Finish button driven by an asynchronous action with Operator pending and result states.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `steps` | `Vec<WizardStep>` | required | List of step definitions with id, title, and optional subtitle. |
| `current` | usize rw-signal | required | Active step index (0-based). |
| `on_validate_step` | callback `(usize) -> Result<(), String>` | none | Validates current step before allowing navigation forward; returns Ok(()) or Err(message). |
| `error` | `Option<String>` rw-signal | none | Optional external signal holding the current validation error message. |
| `error_slot` | slot | none | Custom slot to replace default step error display. |
| `on_finish` | `AsyncAction` | none | Asynchronous handler invoked on final step submission. |
| `next_label` | string | `wizard.next` | Accessible label for the Next button. |
| `back_label` | string | `wizard.back` | Accessible label for the Back button. |
| `finish_label` | string | `wizard.finish` | Accessible label for the Finish button. |
| `navigation` | slot | none | Custom slot to replace default Next/Back navigation. |
| `header_gap` | DU metric | `2 DU` | Spacing between badge and step labels. |
| `step_size` | DU metric | `8 DU` | Size of the square step badge. |
| `padding` | DU metric | `4 DU` | Outer container padding. |
| `content_gap` | DU metric | `4 DU` | Vertical gap between header, body, and footer. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `header_gap` | DU | `--op-wizard-header-gap` | `2 DU` |
| `step_size` | DU | `--op-wizard-step-size` | `8 DU` |
| `padding` | DU | `--op-wizard-padding` | `4 DU` |
| `content_gap` | DU | `--op-wizard-content-gap` | `4 DU` |

| Color variable | Default |
| :--- | :--- |
| `--op-wizard-background` | `surface` |
| `--op-wizard-text` | `text_primary` |
| `--op-wizard-divider` | `divider` |
| `--op-wizard-accent` | `accent` |
| `--op-wizard-error` | `error` |

## Strings

| Key | Default (en) | Description |
| :--- | :--- | :--- |
| `wizard.next` | `Next` | Text for the forward navigation button. |
| `wizard.back` | `Back` | Text for the backward navigation button. |
| `wizard.finish` | `Finish` | Text for the final completion button. |
| `wizard.loading` | `Processing` | Pending state message. |
| `wizard.success` | `Completed` | Success state message. |
| `wizard.error` | `Error` | Error state message. |

## Icons

| Key | Default | Description |
| :--- | :--- | :--- |
| `icons::CHECK` | Material check | Icon displayed on completed step badges. |

## Accessibility

* The step list is marked with `role="region"` and `aria-label="Steps"`.
* Current step button has `aria-current="step"`.
* Square step indicator corners follow the `0 DU` standard.
