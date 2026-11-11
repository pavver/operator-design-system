# Principles

Operator UI is a compact, technical interface language for monitoring and controlling devices in real time.
It borrows its look from terminals, control panels, and code editors.
It is used for mobile, web, and desktop applications, and every platform follows the same rules.

## Core Ideas

* **Information density.**
  Elements sit close together.
  Decorative padding is avoided.
* **Function over decoration.**
  Every line, color, and glyph carries meaning or a control affordance.
* **A quiet idle state.**
  Nothing moves or pulses while the interface waits for the user.
  Motion is reserved for feedback.
* **Honest feedback.**
  Every asynchronous action shows that it is running, then shows how it ended.
  A value is committed only after the backend confirms it.
* **One mental model everywhere.**
  The same state machine, units, and tokens drive every component on every platform.

## Geometry

The base primitive is the rectangle.
Buttons, fields, cards, lists, dialogs, and menus have a corner radius of `0 DU`.
Sharp corners keep the technical character and let elements stack without gaps.

Round shapes are allowed only where the round form is functional:

* The slider thumb.
* Switch tracks and thumbs.
* Radio marks and their dots: the circle is the universal sign of a single choice.
* Avatars of people.
* Push-to-talk buttons.
* Small dots such as slider ticks and status LEDs.

Each component specification states explicitly when it uses one of these exceptions.

## Borders

A standard border is `0.25 DU` thick.
An emphasis border is `0.5 DU` thick.
Every border is drawn inside the element bounds (inner stroke containment).
A border never changes the size of the element it outlines.

## Layout

Screens are built from horizontal rows rather than tiles.
Rows grow vertically with their content and never clip text when the user enlarges fonts.
Containers that hold dynamic content never use a fixed width.
They use the clamp model described in [units.md](units.md).

## Font

Cascadia Code is the only typeface.
See [typography.md](typography.md).

## Prohibited Effects

* Blur, glow, and drop shadows used as feedback.
* Gradients.
* Rounded corners outside the listed exceptions.
* Optimistic updates that show a value before the backend accepts it.
