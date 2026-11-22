# Carousel

Sequential slides container with navigation arrows, dots indicators, swipe gesture, autoplay, and keyboard control.

Status: **draft** on web.

## Use

Use Carousel to present a set of image banners, cards, or featured content in a compact footprint.
It supports automatic rotation paused on hover and focus, as well as full keyboard and swipe gestures.

## Geometry

| Property | Default metric | Variable |
| :--- | :--- | :--- |
| Height | `75 DU` | `--op-carousel-height` |
| Arrow button size | `9 DU` | `--op-carousel-arrow-size` |
| Dot indicator size | `2 DU` | `--op-carousel-dot-size` |

## Behavior

* Left and Right arrow buttons navigate between slides.
* In infinite mode, the carousel loops continuously between the first and last slides.
* Bottom dot indicators show the active slide position and allow direct jumping.
* Left and Right arrow keys move through slides when the carousel is focused.
* Touch swipe gestures allow swiping between slides on touch-enabled devices.
* Autoplay advances slides every interval and pauses while hovered or focused.

## API

### Carousel

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `current` | usize rw-signal | required | Active slide index (0-based). |
| `count` | usize | required | Total count of slides. |
| `arrows` | bool signal | `true` | Displays previous and next arrow buttons. |
| `dots` | bool signal | `true` | Displays bottom dot navigation indicators. |
| `infinite` | bool signal | `true` | Loops seamlessly from start to finish. |
| `autoplay` | bool signal | `false` | Automatically cycles slides. |
| `interval_ms` | u64 signal | `5000` | Delay in milliseconds between autoplay transitions. |
| `swipeable` | bool signal | `true` | Enables touch swipe gestures. |
| `prev_label` | string | `carousel.prev` | Accessible label for previous slide button. |
| `next_label` | string | `carousel.next` | Accessible label for next slide button. |
| `aria_label` | string | `carousel.label` | Accessible region label for the carousel. |
| `height` | DU metric | `75 DU` | Height of the carousel container. |
| `arrow_size` | DU metric | `9 DU` | Size of navigation arrow buttons. |
| `dot_size` | DU metric | `2 DU` | Size of dot indicators. |

### CarouselSlide

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `class` | string | none | Additional CSS class for the slide. |
| `style` | string | none | Inline styles for the slide. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `height` | DU | `--op-carousel-height` | `75 DU` |
| `arrow_size` | DU | `--op-carousel-arrow-size` | `9 DU` |
| `dot_size` | DU | `--op-carousel-dot-size` | `2 DU` |

| Color variable | Default |
| :--- | :--- |
| `--op-carousel-background` | `surface` |
| `--op-carousel-text` | `text_primary` |
| `--op-carousel-border` | `border` |

## Strings

| Key | Default (en) | Description |
| :--- | :--- | :--- |
| `carousel.prev` | `Previous slide` | Screen reader label for previous button. |
| `carousel.next` | `Next slide` | Screen reader label for next button. |
| `carousel.label` | `Carousel` | Region label for the carousel. |

## Icons

| Key | Default | Description |
| :--- | :--- | :--- |
| `icons::CHEVRON_LEFT` | Material chevron left | Previous slide button icon. |
| `icons::CHEVRON_RIGHT` | Material chevron right | Next slide button icon. |

## Accessibility

* The carousel container has `role="region"`, `aria-roledescription="carousel"`, and is keyboard-focusable.
* Individual slides carry `role="group"` and `aria-roledescription="slide"`.
* Dot navigation indicators function as a `tablist` with `aria-selected` status.
