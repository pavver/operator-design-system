# AsyncSwitch

Капсульний перемикач увімкнення, значення якого змінюється лише після підтвердження бекендом.

Статус: **reviewed** у вебреалізації.

## Використання

* Увімкнення або вимкнення функції пристрою чи сервісу.
* Рядки налаштувань, зазвичай як кінцевий елемент [ListItem](list-item.md).

## Геометрія

Капсульна доріжка та круглий бігунок є дозволеним винятком із прямокутної геометрії.

| Властивість | Значення |
| :--- | :--- |
| Доріжка | `13 DU` × `row_compact` (`8 DU`), повністю заокруглена. |
| Бігунок | Коло `row_dense` (`6 DU`), на відстані `pad_tight` (`1 DU`) від краю доріжки. |
| Хід бігунка | `5 DU` між вимкненим і ввімкненим станами; по центру (`2.5 DU`) під час очікування. |
| Піктограма бігунка | `icon_xsmall` (`3.5 DU`); спінер `3.5 DU`. |
| Контур | Контур стану зі штрихом спінера (`0.375 DU`), що повторює капсулу. |

## Кольори

| Частина | Вимкнено | Увімкнено |
| :--- | :--- | :--- |
| Доріжка | `switch_unchecked_track` | `switch_checked_track` |
| Бігунок | `switch_unchecked_thumb` із піктограмою `close` | `switch_checked_thumb` із піктограмою `check` |
| Контур бігунка | `border_strong` | `border_strong` |

Вимкнений стан навмисно червоний, а ввімкнений зелений, щоб значення можна було зчитати з першого погляду.
Під час очікування доріжка стає `surface_alt`, а бігунок переходить у центр.
Він зберігає заповнення та піктограму до появи [спінера із затримкою](../foundations/states.md#delayed-spinner); потім стає прозорим і показує спінер.
Вимкнений елемент використовує `disabled_surface` для доріжки та `text_disabled` для бігунка.

## Поведінка

1. Клік запитує протилежне значення й переводить у `Pending`.
   Бігунок переходить у центр і показує спінер після затримки; кліки ігноруються.
2. У разі **успіху** бігунок переходить на новий бік, а доріжка пульсує `success`.
3. У разі **помилки** бігунок повертається на початковий бік, а доріжка пульсує `error`.
4. Через `800 ms` перемикач повертається в `Idle`, а результат згасає за `500 ms`.

Зміна зовнішнього значення скидає показане значення.

## API

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `checked` | bool | обов’язковий | Підтверджене значення. |
| `state` | `ControlState?` | немає | Зовнішнє перевизначення стану. |
| `enabled` | bool | `true` | Інтерактивність. |
| `on_toggle` | асинхронний `(bool) -> ActionResult` | немає | Викликається із запитаним значенням. |
| `label` | string | `switch.label` | Доступна назва. |
| `width` | реактивний DU | не встановлено | Перевизначає `--op-switch-width`; див. Кастомізація. |
| `height` | реактивний DU | не встановлено | Перевизначає `--op-switch-height`; див. Кастомізація. |
| `thumb_size` | реактивний DU | не встановлено | Перевизначає `--op-switch-thumb-size`; див. Кастомізація. |
| `thumb_inset` | реактивний DU | не встановлено | Перевизначає `--op-switch-thumb-inset`; див. Кастомізація. |
| `icon_size` | реактивний DU | не встановлено | Перевизначає `--op-switch-icon-size`; див. Кастомізація. |
| `spinner_size` | реактивний DU | не встановлено | Перевизначає `--op-switch-spinner-size`; див. Кастомізація. |
| `spinner_stroke` | реактивний DU | не встановлено | Перевизначає `--op-switch-spinner-stroke`; див. Кастомізація. |
| `class` | реактивний string | немає | Застосовується до кореневого елемента. |
| `style` | реактивний string | немає | Застосовується до кореневого елемента. |
| `content` | слот | немає | Замінює вміст бігунка. |
| `loading_label` | реактивний string | `switch.loading` | Перевизначення рядка; див. Рядки. |
| `checked_icon` | піктограма | `switch.checked` | Перевизначення семантичної піктограми; див. Піктограми. |
| `unchecked_icon` | піктограма | `switch.unchecked` | Перевизначення семантичної піктограми; див. Піктограми. |

## Доступність

Елемент керування має роль `switch`, повідомляє свій позначений стан і зайнятість під час очікування.

## Кастомізація

`class` та `style` застосовуються до кореневого елемента.
Метричні пропси приймають реактивні значення DU або TU; невстановлені пропси зберігають змінну CSS або її точний запасний варіант із дизайн-системи.
`style` викликача додається після оголошень метрик і має вищий пріоритет.

| Метричний проп | Одиниця | Змінна | Типове значення |
| :--- | :--- | :--- | :--- |
| `width` | DU | `--op-switch-width` | `calc(var(--op-du) * 13)` |
| `height` | DU | `--op-switch-height` | `var(--op-dim-row-compact)` |
| `thumb_size` | DU | `--op-switch-thumb-size` | `var(--op-dim-row-dense)` |
| `thumb_inset` | DU | `--op-switch-thumb-inset` | `var(--op-dim-pad-tight)` |
| `icon_size` | DU | `--op-switch-icon-size` | `calc(var(--op-du) * 3)` |
| `spinner_size` | DU | `--op-switch-spinner-size` | `calc(var(--op-du) * 3.5)` |
| `spinner_stroke` | DU | `--op-switch-spinner-stroke` | `var(--op-dim-spinner-stroke)` |
| Змінна кольору | Типове значення |
| :--- | :--- |
| `--op-switch-accent` | `var(--op-accent)` |
| `--op-switch-border` | `var(--op-border)` |
| `--op-switch-emphasis-border` | `var(--op-border-strong)` |
| `--op-switch-checked-thumb` | токен палітри `switch_checked_thumb` |
| `--op-switch-checked-track` | токен палітри `switch_checked_track` |
| `--op-switch-disabled-border` | `var(--op-disabled-border)` |
| `--op-switch-disabled-background` | `var(--op-disabled-surface)` |
| `--op-switch-error` | `var(--op-error)` |
| `--op-switch-pending` | `var(--op-pending)` |
| `--op-switch-ripple` | `var(--op-ripple)` |
| `--op-switch-success` | `var(--op-success)` |
| `--op-switch-background` | `var(--op-surface)` |
| `--op-switch-background-alt` | `var(--op-surface-alt)` |
| `--op-switch-disabled-text` | `var(--op-text-disabled)` |
| `--op-switch-icon` | `var(--op-toggle-cloud)` |
| `--op-switch-unchecked-thumb` | токен палітри `switch_unchecked_thumb` |
| `--op-switch-unchecked-track` | токен палітри `switch_unchecked_track` |
| Слот | Розташування |
| :--- | :--- |
| `content` | Замінює вміст бігунка. |

### Рядки

| Проп | Ключ | Типове значення |
| :--- | :--- | :--- |
| `loading_label` | `switch.loading` | `Loading` |
| `label` | `switch.label` | `Switch` |

### Піктограми

| Проп | Ключ | Типове значення |
| :--- | :--- | :--- |
| `checked_icon` | `switch.checked` | `check` |
| `unchecked_icon` | `switch.unchecked` | `close` |
