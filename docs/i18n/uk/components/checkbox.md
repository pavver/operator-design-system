# AsyncCheckbox

Квадратний прапорець із необов’язковим підписом; значення змінюється лише після того, як бекенд підтвердить його.

Статус: **reviewed** у вебреалізації.

## Геометрія

| Властивість | Значення |
| :--- | :--- |
| Квадрат | Квадрат `5 DU`, радіус кутів `0 DU`. |
| Піктограма позначки | `3.5 DU`; спінер `3 DU`. |
| Проміжок між квадратом і підписом | `gutter` (`2 DU`). |
| Внутрішній відступ області натискання | `pad_small` по вертикалі, `pad_tight` по горизонталі. |
| Підпис | розмір `body`, насиченість 500, `text_primary`. |

## Стани

| Стан | Квадрат |
| :--- | :--- |
| Непозначений | `surface`, контур `border`. |
| Позначений | `accent` із непрозорістю 15%, контур `accent` із товщиною 1.5x, позначка `accent`. |
| `Pending` | Рухомий сегмент `pending`, позначка залишається, потім її замінює [спінер із затримкою](../foundations/states.md#delayed-spinner). |
| `Success` | Пульсівний контур `success`; позначка, якщо показана, має колір `success`. |
| `Error` | Пульсівний контур `error`; відновлена позначка, якщо показана, має колір `error`. |
| `Disabled` | `disabled_surface`, `disabled_border`, позначка `icon_disabled`, підпис `text_disabled`. |

## Поведінка

1. Клік запитує протилежне значення та переводить у стан `Pending`; під час очікування кліки ігноруються.
2. У разі **успіху** показується нове значення.
3. У разі **помилки** залишається попереднє значення.
4. Через `800 ms` результат згасає протягом `500 ms`.

## API

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `checked` | bool | обов’язковий | Підтверджене значення. |
| `text` | string | немає | Підпис поруч із квадратом; увесь рядок клікабельний. |
| `state` | `ControlState?` | немає | Зовнішнє перевизначення стану. |
| `enabled` | bool | `true` | Інтерактивність. |
| `on_change` | async `(bool) -> ActionResult` | немає | Викликається із запитаним значенням. |
| `size` | reactive DU | не встановлено | Перевизначає `--op-checkbox-size`; див. Кастомізація. |
| `gap` | reactive DU | не встановлено | Перевизначає `--op-checkbox-gap`; див. Кастомізація. |
| `padding_x` | reactive DU | не встановлено | Перевизначає `--op-checkbox-padding-x`; див. Кастомізація. |
| `padding_y` | reactive DU | не встановлено | Перевизначає `--op-checkbox-padding-y`; див. Кастомізація. |
| `icon_size` | reactive DU | не встановлено | Перевизначає `--op-checkbox-icon-size`; див. Кастомізація. |
| `spinner_size` | reactive DU | не встановлено | Перевизначає `--op-checkbox-spinner-size`; див. Кастомізація. |
| `font_size` | reactive TU | не встановлено | Перевизначає `--op-checkbox-font-size`; див. Кастомізація. |
| `line_height` | reactive TU | не встановлено | Перевизначає `--op-checkbox-line-height`; див. Кастомізація. |
| `spinner_stroke` | reactive DU | не встановлено | Перевизначає `--op-checkbox-spinner-stroke`; див. Кастомізація. |
| `class` | reactive string | немає | Застосовується до кореневого елемента. |
| `style` | reactive string | немає | Застосовується до кореневого елемента. |
| `label_slot` | slot | немає | Замінює необов’язковий текстовий підпис. |
| `loading_label` | reactive string | `checkbox.loading` | Перевизначення рядка; див. Рядки. |
| `label` | reactive string | `checkbox.label` | Перевизначення рядка; див. Рядки. |
| `checked_icon` | icon | `checkbox.checked` | Перевизначення семантичної піктограми; див. Піктограми. |

## Кастомізація

`class` та `style` застосовуються до кореневого елемента.
Метричні пропси приймають реактивні значення DU або TU; невстановлені пропси зберігають змінну CSS або її точний запасний варіант із дизайн-системи.
`style` викликача додається після оголошень метрик і має перевагу.

| Метричний проп | Одиниця | Змінна | Типове значення |
| :--- | :--- | :--- | :--- |
| `size` | DU | `--op-checkbox-size` | `calc(var(--op-du) * 5)` |
| `gap` | DU | `--op-checkbox-gap` | `var(--op-dim-gutter)` |
| `padding_x` | DU | `--op-checkbox-padding-x` | `var(--op-dim-pad-tight)` |
| `padding_y` | DU | `--op-checkbox-padding-y` | `var(--op-dim-pad-small)` |
| `icon_size` | DU | `--op-checkbox-icon-size` | `calc(var(--op-du) * 3.5)` |
| `spinner_size` | DU | `--op-checkbox-spinner-size` | `calc(var(--op-du) * 3)` |
| `font_size` | TU | `--op-checkbox-font-size` | `var(--op-type-body-size)` |
| `line_height` | TU | `--op-checkbox-line-height` | `var(--op-type-body-line)` |
| `spinner_stroke` | DU | `--op-checkbox-spinner-stroke` | `var(--op-dim-spinner-stroke)` |
| Змінна кольору | Типове значення |
| :--- | :--- |
| `--op-checkbox-accent` | `var(--op-accent)` |
| `--op-checkbox-border` | `var(--op-border)` |
| `--op-checkbox-emphasis-border` | `var(--op-border-strong)` |
| `--op-checkbox-disabled-border` | `var(--op-disabled-border)` |
| `--op-checkbox-disabled-background` | `var(--op-disabled-surface)` |
| `--op-checkbox-error` | `var(--op-error)` |
| `--op-checkbox-hover` | `var(--op-hover)` |
| `--op-checkbox-disabled-icon` | `var(--op-icon-disabled)` |
| `--op-checkbox-pending` | `var(--op-pending)` |
| `--op-checkbox-ripple` | `var(--op-ripple)` |
| `--op-checkbox-success` | `var(--op-success)` |
| `--op-checkbox-background` | `var(--op-surface)` |
| `--op-checkbox-disabled-text` | `var(--op-text-disabled)` |
| `--op-checkbox-text` | `var(--op-text-primary)` |
| Слот | Розташування |
| :--- | :--- |
| `label_slot` | Замінює необов’язковий текстовий підпис. |

### Рядки

| Проп | Ключ | Типове значення |
| :--- | :--- | :--- |
| `loading_label` | `checkbox.loading` | `Loading` |
| `label` | `checkbox.label` | `Checkbox` |

### Піктограми

| Проп | Ключ | Типове значення |
| :--- | :--- | :--- |
| `checked_icon` | `checkbox.checked` | `check` |