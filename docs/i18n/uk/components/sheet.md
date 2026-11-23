# Sheet

Панель, що висувається з краю екрана поверх затемнення.
Ліворуч це навігаційна панель; праворуч — панель подробиць; унизу — панель дій.

Статус: **reviewed** у вебреалізації.

## Геометрія

| Властивість | Значення |
| :--- | :--- |
| Ширина бічної панелі | `size` (типово `75 DU`), щонайбільше 85% області перегляду. |
| Висота нижньої панелі | За висотою вмісту, щонайбільше `size` і 85% області перегляду. |
| Тло | `surface`; `0.25 DU` у кольорі `border` на краї, зверненому до вмісту. |
| Заголовок | Необов'язковий, щонайменше `12 DU`: заголовок у стилі `row_title`, кнопка закриття, `divider` знизу. |
| Ручка | Лише нижня панель: смуга `10 DU` × `1 DU` у кольорі `border_strong`, на відстані `2 DU` від верху. |
| Затемнення | Чорний із непрозорістю 55%. |
| Кути | `0 DU`. |

## Поведінка

* Панель висувається зі свого краю за `sheet_slide` (`200 ms`) зі стандартною кривою, поки затемнення проявляється.
* Відкриття переносить фокус у панель.
* Escape, клік на затемненні або кнопка закриття закриває її.
* Тіло прокручується окремо.
* Поки панель відкрита, сторінка позаду не прокручується, зокрема під затемненням.

## API

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `open` | сигнал bool | обов’язковий | Видимість. |
| `side` | Left, Right або Bottom | Left | Край. |
| `title` | string | немає | Заголовок і доступна назва. |
| `size` | реактивна метрика DU | `75` | Ширина або максимальна висота. |
| `children` | view | обов’язковий | Вміст. |

Властивості кастомізації є необов’язковими та приймають реактивні значення там, де це вказано.

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `class`, `style` | реактивний string | немає | Кореневі хуки. |
| `handle_width` | реактивна метрика DU | значення дизайн-системи | Встановлює `--op-sheet-handle-width`. |
| `handle_height` | реактивна метрика DU | значення дизайн-системи | Встановлює `--op-sheet-handle-height`. |
| `gap` | реактивна метрика DU | значення дизайн-системи | Встановлює `--op-sheet-gap`. |
| `header_height` | реактивна метрика DU | значення дизайн-системи | Встановлює `--op-sheet-header-height`. |
| `header_padding_start` | реактивна метрика DU | значення дизайн-системи | Встановлює `--op-sheet-header-padding-start`. |
| `title_font_size` | реактивна метрика TU | значення дизайн-системи | Встановлює `--op-sheet-title-font-size`. |
| `title_line_height` | реактивна метрика TU | значення дизайн-системи | Встановлює `--op-sheet-title-line-height`. |
| `font_size` | реактивна метрика TU | значення дизайн-системи | Встановлює `--op-sheet-font-size`. |
| `line_height` | реактивна метрика TU | значення дизайн-системи | Встановлює `--op-sheet-line-height`. |
| `padding` | реактивна метрика DU | значення дизайн-системи | Встановлює `--op-sheet-padding`. |
| `footer_padding` | реактивна метрика DU | значення дизайн-системи | Встановлює `--op-sheet-footer-padding`. |
| `close_size` | реактивна метрика DU | значення дизайн-системи | Встановлює `--op-sheet-close-size`. |
| `close_icon_size` | реактивна метрика DU | значення дизайн-системи | Встановлює `--op-sheet-close-icon-size`. |
| `header` | `Slot` | немає | Замінює вміст заголовка. |
| `actions` | `Slot` | немає | Дії після тіла. |
| `footer` | `Slot` | немає | Вміст після тіла та дій. |
| `aria_label` | реактивний string | `sheet.label` | Доступна назва; має пріоритет над `title`. |
| `close_label` | реактивний string | `sheet.close` | Доступна назва кнопки закриття. |
| `close_icon` | піктограма | `sheet.close` | Піктограма кнопки закриття. |

## Кастомізація

`class` і `style` застосовуються до кореня затемнення, а змінні успадковуються панеллю Sheet.
Метричні пропси записують відповідні змінні; явне оголошення `style` має перевагу.

| Метричний проп | Одиниця | Змінна | Типове значення |
| :--- | :--- | :--- | :--- |
| `size` | DU | `--op-sheet-size` | `75 DU` |
| `handle_width` | DU | `--op-sheet-handle-width` | `10 DU` |
| `handle_height` | DU | `--op-sheet-handle-height` | `pad_tight` |
| `gap` | DU | `--op-sheet-gap` | `gutter` |
| `header_height` | DU | `--op-sheet-header-height` | `row_control` |
| `header_padding_start` | DU | `--op-sheet-header-padding-start` | `margin_screen` |
| `title_font_size` | TU | `--op-sheet-title-font-size` | `row_title_size` |
| `title_line_height` | TU | `--op-sheet-title-line-height` | `row_title_line` |
| `font_size` | TU | `--op-sheet-font-size` | `body_size` |
| `line_height` | TU | `--op-sheet-line-height` | `body_line` |
| `padding` | DU | `--op-sheet-padding` | `0 DU` |
| `footer_padding` | DU | `--op-sheet-footer-padding` | `0 DU` |
| `close_size` | DU | `--op-sheet-close-size` | `row_compact` |
| `close_icon_size` | DU | `--op-sheet-close-icon-size` | `icon_small` (`4 DU`) |

| Змінна кольору | Типове значення |
| :--- | :--- |
| `--op-sheet-scrim` | `rgb(0 0 0 / 55%)` |
| `--op-sheet-background` | `surface` |
| `--op-sheet-border` | `border` |
| `--op-sheet-handle` | `border_strong` |
| `--op-sheet-divider` | `divider` |
| `--op-sheet-text` | `text_primary` |

| Слот | Розташування |
| :--- | :--- |
| `header` | Замінює вміст заголовка. |
| `actions` | Дії після тіла. |
| `footer` | Вміст після тіла та дій. |

### Рядки

| Ключ | Типове значення | Проп |
| :--- | :--- | :--- |
| `sheet.close` | `Close` | `close_label` |
| `sheet.label` | `empty; falls back to title` | `aria_label` |

### Піктограми

| Ключ | Типове значення | Проп |
| :--- | :--- | :--- |
| `sheet.close` | `close` | `close_icon` |

## Доступність

Роль `dialog` із `aria-modal`.
