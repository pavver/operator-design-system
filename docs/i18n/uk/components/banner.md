# Banner

Постійне повідомлення всередині екрана або розділу про його поточний стан.
Воно залишається, доки умова не зникне або доки користувач не відкине його.

Статус: **reviewed** у вебреалізації.

## Використання

* Стан, що впливає на весь вигляд: офлайн-режим, заповнене сховище, очікування оновлення прошивки.
* Надавайте перевагу [Toast](toast.md) для подій і [Dialog](dialog.md) для рішень, що блокують роботу.

## Геометрія

| Властивість | Значення |
| :--- | :--- |
| Ширина | Повна ширина свого контейнера. |
| Внутрішній відступ | `3 DU`. |
| Тло | Колір тону з непрозорістю 15% поверх `surface`. |
| Початкова смуга | `0.5 DU` у кольорі тону. |
| Піктограма | `icon_xlarge` (`8 DU`), колір тону, `2 DU` від тексту; перший рядок тексту центровано по ній. |
| Заголовок | Стиль body, жирний. |
| Дії | Під текстом, [ActionButtons](action-button.md) заввишки `8 DU`, на відстані `2 DU` одна від одної. |
| Кольори дій | Прозоре тло, мітка та піктограма в кольорі тону, контур у тоні з 50% непрозорістю; наведення додає відтінок тону 12% і повний контур у тоні, натискання — 25%. Кнопки Danger зберігають власні кольори. |
| Закриття | Квадратна кнопка `8 DU` вгорі в кінці. |

Тони та їхні піктограми такі самі, як у [Toast](toast.md).

## Поведінка

* Банер не зникає сам по собі.
* З обробником відкидання він показує кнопку закриття; викликач видаляє банер.
* Текст переноситься; довгі слова розриваються.

## API

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `tone` | tone | `Info` | Info, Success, Warning або Error. |
| `title` | string | немає | Необов’язковий жирний перший рядок. |
| `text` | string | обов’язково | Повідомлення. |
| `actions` | `Slot` | немає | Кнопки під текстом. |
| `on_dismiss` | callback | немає | Показує кнопку закриття. |

Властивості кастомізації є необов’язковими та приймають реактивні значення там, де це вказано.

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `class`, `style` | reactive string | немає | Кореневі хуки. |
| `gap` | reactive DU metric | значення дизайн-системи | Встановлює `--op-banner-gap`. |
| `padding` | reactive DU metric | значення дизайн-системи | Встановлює `--op-banner-padding`. |
| `body_gap` | reactive DU metric | design-system value | Встановлює `--op-banner-body-gap`. |
| `icon_size` | reactive DU metric | значення дизайн-системи | Встановлює `--op-banner-icon-size`. |
| `line_height` | reactive TU metric | значення дизайн-системи | Встановлює `--op-banner-line-height`. |
| `button_size` | reactive DU metric | значення дизайн-системи | Встановлює `--op-banner-button-size`. |
| `close_outset` | reactive DU metric | значення дизайн-системи | Встановлює `--op-banner-close-outset`. |
| `font_size` | reactive TU metric | значення дизайн-системи | Встановлює `--op-banner-font-size`. |
| `close_icon_size` | reactive DU metric | значення дизайн-системи | Встановлює `--op-banner-close-icon-size`. |
| `width` | reactive DU metric | значення дизайн-системи | Встановлює `--op-banner-width`. |
| `header` | `Slot` | немає | Замінює вміст заголовка. |
| `footer` | `Slot` | немає | Вміст після тіла та дій. |
| `dismiss_label` | reactive string | `banner.dismiss` | Доступна назва кнопки відкидання. |
| `icon` | icon | `tone icon key` | Перевизначення піктограми тону. |
| `dismiss_icon` | icon | `banner.dismiss` | Перевизначення піктограми відкидання. |

## Кастомізація

`class` та `style` застосовуються до контейнера банера.
Метричні пропси записують відповідні змінні; явне оголошення `style` має перевагу.

| Метричний проп | Одиниця | Змінна | Типове значення |
| :--- | :--- | :--- | :--- |
| `gap` | DU | `--op-banner-gap` | `gutter` |
| `padding` | DU | `--op-banner-padding` | `pad_card` |
| `body_gap` | DU | `--op-banner-body-gap` | `pad_tight` |
| `icon_size` | DU | `--op-banner-icon-size` | `icon_xlarge` (`8 DU`) |
| `line_height` | TU | `--op-banner-line-height` | `body_line` |
| `button_size` | DU | `--op-banner-button-size` | `row_compact` |
| `close_outset` | DU | `--op-banner-close-outset` | `pad_small` |
| `font_size` | TU | `--op-banner-font-size` | `body_size` |
| `close_icon_size` | DU | `--op-banner-close-icon-size` | `icon_small` (`4 DU`) |
| `width` | DU | `--op-banner-width` | `100%` |

| Змінна кольору | Типове значення |
| :--- | :--- |
| `--op-banner-tone` | `var(--tone)` |
| `--op-banner-surface` | `surface` |
| `--op-banner-text` | `text_primary` |
| `--op-banner-background` | `color-mix(in srgb, var(--op-banner-tone, var(--tone)) 15%, var(--op-banner-surface, surface))` |
| `--op-banner-secondary` | `text_secondary` |
| `--op-banner-hover` | `hover` |
| `--op-banner-ripple` | `ripple` |

| Слот | Розташування |
| :--- | :--- |
| `header` | Замінює вміст заголовка. |
| `actions` | Дії після тіла. |
| `footer` | Вміст після тіла та дій. |

### Рядки

| Ключ | Типове значення | Проп |
| :--- | :--- | :--- |
| `banner.dismiss` | `Dismiss` | `dismiss_label` |

### Піктограми

| Ключ | Типове значення | Проп |
| :--- | :--- | :--- |
| `banner.info` | `info` | `icon` |
| `banner.success` | `check_circle` | `icon` |
| `banner.warning` | `warning` | `icon` |
| `banner.error` | `error` | `icon` |
| `banner.dismiss` | `close` | `dismiss_icon` |

## Доступність

Банери warning та error мають роль `alert`; info та success мають роль `status`.