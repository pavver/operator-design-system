# ConfirmDialog

Модальне підтвердження важливої або деструктивної дії, яка виконується асинхронно всередині діалогу.

Статус: **reviewed** у вебреалізації.

## Композиція

ConfirmDialog є пресетом [Dialog](dialog.md) та має спільні з ним геометрію, кольори й поведінку.
Він задає:

* Ширину: фіксовані `75 DU` або ширину області перегляду за вирахуванням відступів, якщо вона вужча.
* Тіло: повідомлення у стилі `body` та кольорі `text_secondary`.
* Кнопки: Cancel та підтвердження; кнопка підтвердження є небезпечною, якщо встановлено `destructive`.
* Роль: `alertdialog`.

## Поведінка

* Підтвердження виконує дію всередині діалогу зі станами підтвердження, описаними в [Dialog](dialog.md#behavior).
* Успіх закриває діалог через `500 ms`; помилка залишає його відкритим, щоб користувач міг повторити спробу або скасувати дію.
* Cancel, Escape та клік по затемненню закривають діалог, якщо не виконується запит.

## API

| Властивість | Тип | Типове значення |
| :--- | :--- | :--- |
| `open` | read-write bool | обов’язковий |
| `title` | string | обов’язковий |
| `message` | string | обов’язковий |
| `confirm_text` | string | `"Delete"` |
| `cancel_text` | string | `"Cancel"` |
| `destructive` | bool | `true` |
| `on_confirm` | async `() -> ActionResult` | обов’язковий |

Властивості кастомізації є необов’язковими та приймають реактивні значення там, де це вказано.

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `class`, `style` | reactive string | немає | Кореневі хуки. |
| `font_size` | reactive TU metric | значення дизайн-системи | Встановлює `--op-confirm-dialog-font-size`. |
| `line_height` | reactive TU metric | значення дизайн-системи | Встановлює `--op-confirm-dialog-line-height`. |
| `padding` | reactive DU metric | значення дизайн-системи | Встановлює `--op-confirm-dialog-padding`. |
| `header_padding` | reactive DU metric | значення дизайн-системи | Встановлює `--op-confirm-dialog-header-padding`. |
| `title_font_size` | reactive TU metric | значення дизайн-системи | Встановлює `--op-confirm-dialog-title-font-size`. |
| `title_line_height` | reactive TU metric | значення дизайн-системи | Встановлює `--op-confirm-dialog-title-line-height`. |
| `gap` | reactive DU metric | значення дизайн-системи | Встановлює `--op-confirm-dialog-gap`. |
| `button_height` | reactive DU metric | значення дизайн-системи | Встановлює `--op-confirm-dialog-button-height`. |
| `width` | reactive DU metric | значення дизайн-системи | Встановлює `--op-confirm-dialog-width`. |
| `max_height` | reactive DU metric | значення дизайн-системи | Встановлює `--op-confirm-dialog-max-height`. |
| `header` | `Slot` | немає | Замінює вміст заголовка. |
| `actions` | `Slot` | немає | Додаткові дії перед вбудованими кнопками дій. |
| `footer` | `Slot` | немає | Вміст після тіла та дій. |

## Кастомізація

`class` та `style` застосовуються до кореня затемнення Dialog, а панель підтвердження успадковує змінні.
Метричні пропси записують відповідні змінні; явне оголошення `style` має вищий пріоритет.

| Метричний проп | Одиниця | Змінна | Типове значення |
| :--- | :--- | :--- | :--- |
| `font_size` | TU | `--op-confirm-dialog-font-size` | `body_size` |
| `line_height` | TU | `--op-confirm-dialog-line-height` | `body_line` |
| `padding` | DU | `--op-confirm-dialog-padding` | `margin_screen` |
| `header_padding` | DU | `--op-confirm-dialog-header-padding` | `gutter` |
| `title_font_size` | TU | `--op-confirm-dialog-title-font-size` | `screen_title_size` |
| `title_line_height` | TU | `--op-confirm-dialog-title-line-height` | `screen_title_line` |
| `gap` | DU | `--op-confirm-dialog-gap` | `pad_card` |
| `button_height` | DU | `--op-confirm-dialog-button-height` | `row_compact` |
| `width` | DU | `--op-confirm-dialog-width` | `75 DU` |
| `max_height` | DU | `--op-confirm-dialog-max-height` | `calc(100vh - var(--op-confirm-dialog-padding, margin_screen) * 2)` |

| Змінна кольору | Типове значення |
| :--- | :--- |
| `--op-confirm-dialog-text` | `text_secondary` |
| `--op-confirm-dialog-title-text` | `text_primary` |
| `--op-confirm-dialog-background` | `surface` |
| `--op-confirm-dialog-divider` | `divider` |
| `--op-confirm-dialog-scrim` | `rgb(0 0 0 / 55%)` |
| `--op-confirm-dialog-border` | `border` |
| `--op-confirm-dialog-pending` | `pending` |
| `--op-confirm-dialog-success` | `success` |
| `--op-confirm-dialog-error` | `error` |

| Слот | Розташування |
| :--- | :--- |
| `header` | Замінює вміст заголовка. |
| `actions` | Додаткові дії перед вбудованими кнопками дій. |
| `footer` | Вміст після тіла та дій. |

Сімейство підтвердження зіставляє свої змінні контейнера з публічною оболонкою Dialog.
`width` встановлює обидві межі clamp, зберігаючи фіксовану типову ширину.

### Рядки

| Ключ | Типове значення | Проп |
| :--- | :--- | :--- |
| `confirm-dialog.confirm` | `Delete` | `confirm_text` |
| `confirm-dialog.cancel` | `Cancel` | `cancel_text` |

### Піктограми

Цей компонент не має вбудованих піктограм.
