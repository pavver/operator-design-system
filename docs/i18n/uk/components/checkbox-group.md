# CheckboxGroup

Вертикальний список доступних для вибору плиток на всю ширину; кожна плитка зберігає свій вибір асинхронно.

Статус: **reviewed** у вебреалізації.

## Використання

* Множинний вибір із короткого списку: канали сповіщень, увімкнені зони, призначені групи.

## Геометрія

| Властивість | Значення |
| :--- | :--- |
| Ширина | Заповнює батьківський елемент. |
| Висота плитки | `row_control` (`12 DU`). |
| Внутрішній відступ плитки | `gutter` (`2 DU`) по горизонталі. |
| Проміжок між плитками | `0.75 DU`. |
| Піктограма | Необов’язкова, перед назвою з проміжком `gutter` (`2 DU`); розмір задається через `icon_size`. |
| Бокс індикатора | Квадрат `icon_large` (`6 DU`) біля правого краю, галочка та спінер — `icon_small` (`4 DU`). |
| Радіус заокруглення кутів | `0 DU`. |

### Розміри піктограм

| Розмір | Значення | Початковий внутрішній відступ плитки |
| :--- | :--- | :--- |
| `Small` | `icon_small` (`4 DU`) | `gutter` (`2 DU`) |
| `Medium` | `icon_large` (`6 DU`) | `gutter` (`2 DU`) |
| `Large` | Висота плитки мінус `pad_tight` з кожного боку (`10 DU`) | `pad_tight` (`1 DU`), щоб піктограма мала однаковий відступ із трьох боків |

## Стани

| Плитка | Тло | Текст | Контур | Індикатор |
| :--- | :--- | :--- | :--- | :--- |
| Невибрана | `surface` | `text_primary`, 500; піктограма в `icon` | `border` | Порожній бокс із контуром `border` |
| Вибрана | `accent` на 15% | `accent`, 700; піктограма в `accent` | `accent` із товщиною 1.5x | Бокс `accent` із галочкою `accent` |
| Натиснута | `pressed` | — | `border_strong` | — |
| `Pending` | — | — | Рухомий сегмент | [спінер із затримкою](../foundations/states.md#delayed-spinner) |
| `Success` / `Error` | — | — | Пульсівний контур результату | Галочка в кольорі результату |

## Поведінка

* Кожна плитка виконує власний запит; кілька плиток можуть перебувати в стані очікування одночасно.
* Плитка в стані очікування ігнорує кліки; інші плитки залишаються інтерактивними.
* Зворотний зв’язок про результат не блокує введення, а застарілі таймери результату не можуть скасувати новіший запит.
* Батьківський компонент оновлює список вибраного в разі успіху; плитка лише показує результат.
* Результат утримується протягом `800 ms`, а потім згасає за `500 ms`.

## API

| Властивість | Тип | Опис |
| :--- | :--- | :--- |
| `items` | список `GroupItem { id, name, icon? }` | Варіанти вибору; `icon` — необов’язкова початкова піктограма. |
| `selected` | список id | Підтверджений вибір. |
| `on_change` | async `((id, bool)) -> ActionResult` | Викликається з id елемента та запитаним значенням. |
| `icon_size` | `Small`, `Medium`, `Large` | Розмір піктограм елементів; типово `Small`. |
| `gap` | реактивний DU | Перевизначає `--op-checkbox-group-gap`; див. Кастомізація; типово: не встановлено. |
| `height` | реактивний DU | Перевизначає `--op-checkbox-group-height`; див. Кастомізація; типово: не встановлено. |
| `padding_x` | реактивний DU | Перевизначає `--op-checkbox-group-padding-x`; див. Кастомізація; типово: не встановлено. |
| `item_gap` | реактивний DU | Перевизначає `--op-checkbox-group-item-gap`; див. Кастомізація; типово: не встановлено. |
| `large_icon_inset` | реактивний DU | Перевизначає `--op-checkbox-group-large-icon-inset`; див. Кастомізація; типово: не встановлено. |
| `box_size` | реактивний DU | Перевизначає `--op-checkbox-group-box-size`; див. Кастомізація; типово: не встановлено. |
| `mark_size` | реактивний DU | Перевизначає `--op-checkbox-group-mark-size`; див. Кастомізація; типово: не встановлено. |
| `item_icon_size` | реактивний DU | Перевизначає `--op-checkbox-group-item-icon-size`; див. Кастомізація; типово: не встановлено. |
| `font_size` | реактивний TU | Перевизначає `--op-checkbox-group-font-size`; див. Кастомізація; типово: не встановлено. |
| `line_height` | реактивний TU | Перевизначає `--op-checkbox-group-line-height`; див. Кастомізація; типово: не встановлено. |
| `spinner_stroke` | реактивний DU | Перевизначає `--op-checkbox-group-spinner-stroke`; див. Кастомізація; типово: не встановлено. |
| `width` | реактивний DU | Перевизначає `--op-checkbox-group-width`; див. Кастомізація; типово: не встановлено. |
| `class` | реактивний string | Застосовується до кореневого елемента; типово: немає. |
| `style` | реактивний string | Застосовується до кореневого елемента; типово: немає. |
| `option` | слот з областю видимості | Замінює мітку варіанта; отримує елемент; типово: немає. |
| `before_options` | слот | Перед варіантами вибору; типово: немає. |
| `after_options` | слот | Після варіантів вибору; типово: немає. |
| `loading_label` | реактивний string | Перевизначення рядка; див. Рядки; типово: `checkbox-group.loading`. |
| `label` | реактивний string | Перевизначення рядка; див. Рядки; типово: `checkbox-group.label`. |
| `checked_icon` | піктограма | Перевизначення семантичної піктограми; див. Піктограми; типово: `checkbox-group.checked`. |

## Кастомізація

`class` та `style` застосовуються до кореневого елемента.
Метричні пропси приймають реактивні значення DU або TU; невстановлені пропси зберігають змінну CSS або її точний запасний варіант із дизайн-системи.
`style` викликача додається після оголошень метрик і має вищий пріоритет.

| Метричний проп | Одиниця | Змінна | Типове значення |
| :--- | :--- | :--- | :--- |
| `gap` | DU | `--op-checkbox-group-gap` | `calc(var(--op-du) * 0.75)` |
| `height` | DU | `--op-checkbox-group-height` | `var(--op-dim-row-control)` |
| `padding_x` | DU | `--op-checkbox-group-padding-x` | `var(--op-dim-gutter)` |
| `item_gap` | DU | `--op-checkbox-group-item-gap` | `var(--op-dim-gutter)` |
| `large_icon_inset` | DU | `--op-checkbox-group-large-icon-inset` | `var(--op-dim-pad-tight)` |
| `box_size` | DU | `--op-checkbox-group-box-size` | `var(--op-dim-icon-large)` |
| `mark_size` | DU | `--op-checkbox-group-mark-size` | `var(--op-dim-icon-small)` |
| `item_icon_size` | DU | `--op-checkbox-group-item-icon-size` | `var(--op-dim-icon-small)` |
| `font_size` | TU | `--op-checkbox-group-font-size` | `var(--op-type-body-size)` |
| `line_height` | TU | `--op-checkbox-group-line-height` | `var(--op-type-body-line)` |
| `spinner_stroke` | DU | `--op-checkbox-group-spinner-stroke` | `var(--op-dim-spinner-stroke)` |
| `width` | DU | `--op-checkbox-group-width` | `100%` |

`item_icon_size` перевизначає наявний enum `icon_size`; якщо його не встановлено, Small використовує `icon_small`, Medium використовує `icon_large`, а Large використовує обчислену `height` мінус два значення `large_icon_inset`.

| Змінна кольору | Типове значення |
| :--- | :--- |
| `--op-checkbox-group-accent` | `var(--op-accent)` |
| `--op-checkbox-group-border` | `var(--op-border)` |
| `--op-checkbox-group-emphasis-border` | `var(--op-border-strong)` |
| `--op-checkbox-group-disabled-border` | `var(--op-disabled-border)` |
| `--op-checkbox-group-error` | `var(--op-error)` |
| `--op-checkbox-group-hover` | `var(--op-hover)` |
| `--op-checkbox-group-icon` | `var(--op-icon)` |
| `--op-checkbox-group-pending` | `var(--op-pending)` |
| `--op-checkbox-group-pressed` | `var(--op-pressed)` |
| `--op-checkbox-group-ripple` | `var(--op-ripple)` |
| `--op-checkbox-group-success` | `var(--op-success)` |
| `--op-checkbox-group-background` | `var(--op-surface)` |
| `--op-checkbox-group-text` | `var(--op-text-primary)` |
| Слот | Розташування |
| :--- | :--- |
| `option` | Замінює мітку варіанта; отримує елемент. |
| `before_options` | Перед варіантами вибору. |
| `after_options` | Після варіантів вибору. |

### Рядки

| Проп | Ключ | Типове значення |
| :--- | :--- | :--- |
| `loading_label` | `checkbox-group.loading` | `Loading` |
| `label` | `checkbox-group.label` | `Options` |

### Піктограми

| Проп | Ключ | Типове значення |
| :--- | :--- | :--- |
| `checked_icon` | `checkbox-group.checked` | `check` |

Піктограми, передані як дані елемента або через наявний проп `icon`, залишаються у власності викликача.