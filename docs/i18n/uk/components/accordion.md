# Accordion

Розділи з рядками заголовків, які показують або приховують свій вміст.
Окремий розділ — це Collapsible; Accordion тримає відкритим лише один розділ.

Статус: **reviewed** у вебреалізації.

## Геометрія

| Властивість | Значення |
| :--- | :--- |
| Заголовок | Щонайменше `10 DU`; внутрішній відступ `2 DU` на початку та `3 DU` в кінці. |
| Шеврон | `4 DU` `chevron_right` у `text_secondary`; повернутий на 90° та `accent` у відкритому стані. |
| Назва | Середня насиченість; необов'язковий підзаголовок у стилі `caption`. |
| Кінцевий елемент | Необов'язковий вміст у кінці, як-от [Counter](counter.md). |
| Вміст | З відступом до назви; внутрішній відступ `1 DU` згори та `3 DU` знизу. |
| Accordion | Контур `border` товщиною `0.25 DU`, `divider` між розділами. |

## Поведінка

* Клік на заголовку перемикає розділ.
* Висота вмісту анімується за час `expand` (`200 ms`) зі стандартною кривою; шеврон повертається разом із нею.
* Закритий вміст залишається змонтованим, але є інертним: він не може отримувати фокус і прихований від допоміжних технологій.
* В Accordion відкриття розділу закриває відкритий.

## API

| Компонент | Властивості |
| :--- | :--- |
| `Accordion` | `children`: розділи. |
| `Collapsible` | `title`, `subtitle`, `open` (зовнішній сигнал), `initially_open`, `trailing`, `children`. |

### API кастомізації

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `header_height` | optional number (DU) | `row_standard` | Перевизначає `--op-accordion-header-height`. |
| `padding_start` | optional number (DU) | `gutter` | Перевизначає `--op-accordion-padding-start`. |
| `padding_end` | optional number (DU) | `pad_card` | Перевизначає `--op-accordion-padding-end`. |
| `padding_y` | optional number (DU) | `pad_tight` | Перевизначає `--op-accordion-padding-y`. |
| `gap` | optional number (DU) | `gutter` | Перевизначає `--op-accordion-gap`. |
| `icon_size` | optional number (DU) | `icon_small` (`4 DU`) | Перевизначає `--op-accordion-icon-size`. |
| `content_padding_top` | optional number (DU) | `pad_tight` | Перевизначає `--op-accordion-content-padding-top`. |
| `content_padding_end` | optional number (DU) | `pad_card` | Перевизначає `--op-accordion-content-padding-end`. |
| `content_padding_bottom` | optional number (DU) | `pad_card` | Перевизначає `--op-accordion-content-padding-bottom`. |
| `content_padding_start` | optional number (DU) | `padding_start + gap + icon_size` (типово `2 × gutter + icon_small`) | Перевизначає `--op-accordion-content-padding-start`. |
| `font_size` | optional number (TU) | `inherit` | Перевизначає `--op-accordion-font-size`. |
| `line_height` | optional number (TU) | `inherit` | Перевизначає `--op-accordion-line-height`. |
| `subtitle_font_size` | optional number (TU) | `type_caption_size` | Перевизначає `--op-accordion-subtitle-font-size`. |
| `subtitle_line_height` | optional number (TU) | `type_caption_line` | Перевизначає `--op-accordion-subtitle-line-height`. |
| `header` | slot | none | Accordion: перед розділами; Collapsible: замінює назву та підзаголовок усередині кнопки перемикання. |
| `actions` | slot | none | Accordion: перед розділами; Collapsible: зовні кнопки перемикання, перед панеллю. |
| `footer` | slot | none | Accordion: після розділів; Collapsible: після вмісту всередині анімованої панелі. |
| `trailing` | slot | none | Лише Collapsible; кінцевий вміст усередині кнопки перемикання. |
| `expand_label` | optional string | `accordion.expand` | Перевизначення тексту. |
| `collapse_label` | optional string | `accordion.collapse` | Перевизначення тексту. |
| `chevron_icon` | optional icon | `accordion.chevron` | Перевизначення піктограми. |
| `class` | optional string | none | Додається до кореневих класів. |
| `style` | optional string | none | Додається після кореневих оголошень метрик. |

## Доступність

Заголовок є кнопкою з `aria-expanded` та `aria-controls`, що вказують на область вмісту.

## Кастомізація

`class` і `style` застосовуються до кореня.
Явні кореневі стилі мають перевагу над метричними пропсами; невстановлені метрики зберігають початковий рендеринг.
Метричні та кореневі хуки доступні як для Accordion, так і для окремого Collapsible; групові змінні успадковуються в розділи.
`expand_label`, `collapse_label` та `chevron_icon` належать до Collapsible.
Опис дії слідує за станом відкритості, не замінюючи назву як доступну назву.

| Метричний проп | Одиниця | Змінна | Типове значення |
| :--- | :--- | :--- | :--- |
| `header_height` | DU | `--op-accordion-header-height` | `row_standard` |
| `padding_start` | DU | `--op-accordion-padding-start` | `gutter` |
| `padding_end` | DU | `--op-accordion-padding-end` | `pad_card` |
| `padding_y` | DU | `--op-accordion-padding-y` | `pad_tight` |
| `gap` | DU | `--op-accordion-gap` | `gutter` |
| `icon_size` | DU | `--op-accordion-icon-size` | `icon_small` (`4 DU`) |
| `content_padding_top` | DU | `--op-accordion-content-padding-top` | `pad_tight` |
| `content_padding_end` | DU | `--op-accordion-content-padding-end` | `pad_card` |
| `content_padding_bottom` | DU | `--op-accordion-content-padding-bottom` | `pad_card` |
| `content_padding_start` | DU | `--op-accordion-content-padding-start` | `padding_start + gap + icon_size` (типово `2 × gutter + icon_small`) |
| `font_size` | TU | `--op-accordion-font-size` | `inherit` |
| `line_height` | TU | `--op-accordion-line-height` | `inherit` |
| `subtitle_font_size` | TU | `--op-accordion-subtitle-font-size` | `type_caption_size` |
| `subtitle_line_height` | TU | `--op-accordion-subtitle-line-height` | `type_caption_line` |

Товщина меж залишається фіксованою геометрією, вказаною вище.

| Змінна кольору | Типове значення |
| :--- | :--- |
| `--op-accordion-border` | `border` |
| `--op-accordion-divider` | `divider` |
| `--op-accordion-text` | `text_primary` |
| `--op-accordion-hover` | `hover` |
| `--op-accordion-secondary-text` | `text_secondary` |
| `--op-accordion-accent` | `accent` |

| Слот | Розташування |
| :--- | :--- |
| `header` | Accordion: перед розділами; Collapsible: замінює назву та підзаголовок усередині кнопки перемикання. |
| `actions` | Accordion: перед розділами; Collapsible: зовні кнопки перемикання, перед панеллю. |
| `footer` | Accordion: після розділів; Collapsible: після вмісту всередині анімованої панелі. |
| `trailing` | Лише Collapsible; кінцевий вміст усередині кнопки перемикання. |
| `children` | Розділи в Accordion; вміст панелі в Collapsible. |

### Рядки

Порядок розв'язання: проп, пошук застосунком, англійське типове значення.

| Ключ | Типове значення |
| :--- | :--- |
| `accordion.expand` | `Expand` |
| `accordion.collapse` | `Collapse` |

### Піктограми

| Ключ | Типове значення |
| :--- | :--- |
| `accordion.chevron` | `chevron_right` |