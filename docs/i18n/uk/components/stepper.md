# Stepper

Елемент керування `− value +` для цілих чисел, що залишається інтерактивним під час очікування, щоб користувач міг швидко змінювати значення кроками.

Статус: **reviewed** у вебреалізації.

## Використання

* Невеликі цілочисельні налаштування: номер набору, кількість, рівень.

## Геометрія

| Властивість | Значення |
| :--- | :--- |
| Ширина | Типово `35 DU`; підпис поруч займає решту місця. |
| Висота | `row_compact` (`8 DU`). |
| Кнопки | Квадрати `8 DU` з обох кінців, піктограми `icon_small` (`4 DU`). |
| Значення | Заповнює середину; фон `surface_alt`, жирні табличні цифри. |
| Радіус заокруглення кутів | `0 DU`. |
| Контур | Контур стану навколо всього елемента керування; кожна частина має власний контур `border`. |

## Стани

| Частина | Вигляд |
| :--- | :--- |
| Кнопка | `surface`, `icon`; при натисканні `pressed` із `border_strong`. |
| Кнопка на межі | `disabled_surface`, `icon_disabled`, ігнорує кліки. |
| Натиснута кнопка під час очікування | Знак, потім [спінер із затримкою](../foundations/states.md#delayed-spinner); послідовні кліки не перезапускають затримку. |
| Натиснута кнопка при результаті | `check` у кольорі `success` або `close` у кольорі `error`. |
| Протилежна кнопка під час очікування | Приглушена як вимкнена кнопка, але на неї все ще можна клікнути. |
| Увесь елемент керування | Рухомий сегмент під час очікування, потім пульсівний контур результату. |

## Поведінка

Цей компонент є задокументованим винятком із правила, що Pending блокує введення.

* Кожен клік обчислює ціль із останнього запитаного значення, а не з останнього підтвердженого.
* Кожен клік починає новий запит і замінює попередній.
* У разі **успіху** показане значення стає цільовим.
* У разі **помилки** запитане значення скидається до останнього підтвердженого.
* Результат утримується протягом `800 ms`, а потім згасає за `500 ms`.
* Без асинхронного обробника значення змінюється негайно.

## API

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `value` | ціле число для читання й запису | обов’язковий | Підтверджене значення. |
| `min` | integer | немає | Нижня межа. |
| `max` | integer | немає | Верхня межа. |
| `step` | integer | `1` | Розмір кроку. |
| `state` | `ControlState?` | немає | Зовнішнє перевизначення стану. |
| `enabled` | bool | `true` | Інтерактивність. |
| `on_async_change` | асинхронний `(i32) -> ActionResult` | немає | Фіксує запитане значення. |
| `width` | DU | `35` | Ширина елемента керування. |
| `height` | реактивний DU | не встановлено | Перевизначає `--op-stepper-height`; див. Кастомізація. |
| `button_size` | реактивний DU | не встановлено | Перевизначає `--op-stepper-button-size`; див. Кастомізація. |
| `icon_size` | реактивний DU | не встановлено | Перевизначає `--op-stepper-icon-size`; див. Кастомізація. |
| `spinner_size` | реактивний DU | не встановлено | Перевизначає `--op-stepper-spinner-size`; див. Кастомізація. |
| `font_size` | реактивний TU | не встановлено | Перевизначає `--op-stepper-font-size`; див. Кастомізація. |
| `line_height` | реактивний TU | не встановлено | Перевизначає `--op-stepper-line-height`; див. Кастомізація. |
| `spinner_stroke` | реактивний DU | не встановлено | Перевизначає `--op-stepper-spinner-stroke`; див. Кастомізація. |
| `class` | реактивний string | немає | Застосовується до кореневого елемента. |
| `style` | реактивний string | немає | Застосовується до кореневого елемента. |
| `content` | слот | немає | Замінює показане значення. |
| `loading_label` | реактивний string | `stepper.loading` | Перевизначення рядка; див. Рядки. |
| `success_label` | реактивний string | `stepper.success` | Перевизначення рядка; див. Рядки. |
| `error_label` | реактивний string | `stepper.error` | Перевизначення рядка; див. Рядки. |
| `increase_label` | реактивний string | `stepper.increase` | Перевизначення рядка; див. Рядки. |
| `decrease_label` | реактивний string | `stepper.decrease` | Перевизначення рядка; див. Рядки. |
| `success_icon` | піктограма | `stepper.success` | Перевизначення семантичної піктограми; див. Піктограми. |
| `error_icon` | піктограма | `stepper.error` | Перевизначення семантичної піктограми; див. Піктограми. |
| `increase_icon` | піктограма | `stepper.increase` | Перевизначення семантичної піктограми; див. Піктограми. |
| `decrease_icon` | піктограма | `stepper.decrease` | Перевизначення семантичної піктограми; див. Піктограми. |

## Кастомізація

`class` та `style` застосовуються до кореневого елемента.
Метричні пропси приймають реактивні значення DU або TU; невстановлені пропси зберігають змінну CSS або її точний запасний варіант із дизайн-системи.
`style` викликача додається після оголошень метрик і має вищий пріоритет.

| Метричний проп | Одиниця | Змінна | Типове значення |
| :--- | :--- | :--- | :--- |
| `width` | DU | `--op-stepper-width` | `calc(var(--op-du) * 35)` |
| `height` | DU | `--op-stepper-height` | `var(--op-dim-row-compact)` |
| `button_size` | DU | `--op-stepper-button-size` | `var(--op-dim-row-compact)` |
| `icon_size` | DU | `--op-stepper-icon-size` | `var(--op-dim-icon-small)` |
| `spinner_size` | DU | `--op-stepper-spinner-size` | `calc(var(--op-du) * 3)` |
| `font_size` | TU | `--op-stepper-font-size` | `var(--op-type-body-size)` |
| `line_height` | TU | `--op-stepper-line-height` | `var(--op-type-body-line)` |
| `spinner_stroke` | DU | `--op-stepper-spinner-stroke` | `var(--op-dim-spinner-stroke)` |

Наявні числові пропси розміру зберігають назви та значення в DU й тепер також приймають реактивні значення.

| Змінна кольору | Типове значення |
| :--- | :--- |
| `--op-stepper-accent` | `var(--op-accent)` |
| `--op-stepper-border` | `var(--op-border)` |
| `--op-stepper-emphasis-border` | `var(--op-border-strong)` |
| `--op-stepper-disabled-border` | `var(--op-disabled-border)` |
| `--op-stepper-disabled-background` | `var(--op-disabled-surface)` |
| `--op-stepper-error` | `var(--op-error)` |
| `--op-stepper-hover` | `var(--op-hover)` |
| `--op-stepper-icon` | `var(--op-icon)` |
| `--op-stepper-disabled-icon` | `var(--op-icon-disabled)` |
| `--op-stepper-pending` | `var(--op-pending)` |
| `--op-stepper-pressed` | `var(--op-pressed)` |
| `--op-stepper-ripple` | `var(--op-ripple)` |
| `--op-stepper-success` | `var(--op-success)` |
| `--op-stepper-background` | `var(--op-surface)` |
| `--op-stepper-background-alt` | `var(--op-surface-alt)` |
| `--op-stepper-disabled-text` | `var(--op-text-disabled)` |
| `--op-stepper-text` | `var(--op-text-primary)` |
| Слот | Розташування |
| :--- | :--- |
| `content` | Замінює показане значення. |

### Рядки

| Проп | Ключ | Типове значення |
| :--- | :--- | :--- |
| `loading_label` | `stepper.loading` | `Loading` |
| `success_label` | `stepper.success` | `Success` |
| `error_label` | `stepper.error` | `Error` |
| `increase_label` | `stepper.increase` | `Increase` |
| `decrease_label` | `stepper.decrease` | `Decrease` |

### Піктограми

| Проп | Ключ | Типове значення |
| :--- | :--- | :--- |
| `success_icon` | `stepper.success` | `check` |
| `error_icon` | `stepper.error` | `close` |
| `increase_icon` | `stepper.increase` | `add` |
| `decrease_icon` | `stepper.decrease` | `remove` |
