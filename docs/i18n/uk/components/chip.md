# Chip

Компактний маркер для фільтра, значення або тегу.

Статус: **reviewed** у вебреалізації.

## Варіанти

| Варіант | Призначення | Взаємодія |
| :--- | :--- | :--- |
| Filter chip | Вмикає або вимикає фільтр. | Перемикається за кліком; показує позначку вибору в увімкненому стані. |
| Input chip | Одне значення поля з кількома значеннями, наприклад тег. | Має кнопку видалення. |
| Action chip | Невелика дія швидкого доступу. | Викликає свій обробник. |
| Static tag | Мітка, така як кодек або роздільна здатність. | Немає. |

## Геометрія

| Властивість | Значення |
| :--- | :--- |
| Висота | `6 DU`. |
| Внутрішній відступ | `2 DU` по горизонталі. |
| Типографіка | Стиль `body_small`. |
| Піктограма та позначка | `3.5 DU`, `1 DU` від тексту. |
| Кнопка видалення | Завширшки `6 DU` наприкінці. |
| Контур | Контур стану. |
| Набір | Чипи переносяться з проміжками `1 DU`. |
| Кути | `0 DU`. |

## Кольори

| Колір | Використання |
| :--- | :--- |
| `Neutral` | Типово: тло `surface`, `text_primary`. |
| `Accent`, `Success`, `Warning`, `Error`, `Info` | Статус значення з використанням відповідного колірного токена. |
| `Custom(color)` | Визначені користувачем теги з власним CSS-кольором. |

* Кольоровий чип має тло з 14% кольору поверх `surface`, текст і піктограми в цьому кольорі та контур із 55% кольору поверх `surface`.
* Власний колір не підлаштовується під окремі режими, тому його текст змішується на 65% із `text_primary`, щоб залишатися читабельним у темному та світлому режимах.
* Вибраний кольоровий чип: тло 28% і контур `1.5×` у кольорі тексту.
* Натиснутий кольоровий чип: 22% кольору поверх `pressed`.
* Вимкнені чипи використовують кольори стану disabled незалежно від їхнього кольору.
* Колір ніколи не несе значення самостійно: тональні чипи також виражають його в тексті або піктограмі.

## Стани

* Selected: тло `selection`, текст `accent` та контур `Selected`; кольорові чипи підпорядковуються розділу [Кольори](#colors).
* Pressed: тло `pressed`.
* Disabled: кольори стану disabled.
* Кнопка видалення стає `error` під час наведення.

## API

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `text` | string | `chip.text` | Мітка. |
| `icon` | icon | немає | Початкова піктограма. |
| `selected` | bool signal | немає | Створює фільтр-чип. |
| `on_click` | callback | немає | Створює чип дії. |
| `on_remove` | callback | немає | Додає кнопку видалення. |
| `enabled` | bool | `true` | Вмикає взаємодію. |
| `color` | `ChipColor` | `Neutral` | Тональний або власний колір. |
| `height` | reactive DU | не встановлено | Перевизначає `--op-chip-height`; див. Кастомізація. |
| `gap` | reactive DU | не встановлено | Перевизначає `--op-chip-gap`; див. Кастомізація. |
| `padding_x` | reactive DU | не встановлено | Перевизначає `--op-chip-padding-x`; див. Кастомізація. |
| `remove_size` | reactive DU | не встановлено | Перевизначає `--op-chip-remove-size`; див. Кастомізація. |
| `remove_overlap` | reactive DU | не встановлено | Перевизначає `--op-chip-remove-overlap`; див. Кастомізація. |
| `icon_size` | reactive DU | не встановлено | Перевизначає `--op-chip-icon-size`; див. Кастомізація. |
| `font_size` | reactive TU | не встановлено | Перевизначає `--op-chip-font-size`; див. Кастомізація. |
| `line_height` | reactive TU | не встановлено | Перевизначає `--op-chip-line-height`; див. Кастомізація. |
| `class` | reactive string | немає | Застосовується до кореневого елемента. |
| `style` | reactive string | немає | Застосовується до кореневого елемента. |
| `leading` | slot | немає | Перед типовим вмістом. |
| `trailing` | slot | немає | Після типового вмісту. |
| `content` | slot | немає | Замінює типовий текст або піктограму; зворотний зв’язок очікування та результату зберігає пріоритет. |
| `remove_label` | reactive string | `chip.remove` | Перевизначення рядка; див. Рядки. |
| `checked_icon` | icon | `chip.checked` | Перевизначення семантичної піктограми; див. Піктограми. |
| `remove_icon` | icon | `chip.remove` | Перевизначення семантичної піктограми; див. Піктограми. |

`ChipSet` компонує чипи в ряди з перенесенням.

### API ChipSet

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `children` | children | обов’язковий | Чипи. |
| `gap` | reactive DU | не встановлено | Перевизначає `--op-chip-set-gap`, яка має запасним варіантом `pad_tight`. |
| `class`, `style` | reactive string | немає | Застосовується до кореня набору. |

## Доступність

* Фільтр-чип є `checkbox` з `aria-checked`.
* Кнопка видалення використовує локалізований шаблон `chip.remove` зі вставленим текстом чипа.

## Кастомізація

`class` та `style` застосовуються до кореневого елемента.
Метричні пропси приймають реактивні значення DU або TU; невстановлені пропси зберігають змінну CSS або її точний запасний варіант із дизайн-системи.
`style` викликача додається після оголошень метрик і має пріоритет.

| Метричний проп | Одиниця | Змінна | Типове значення |
| :--- | :--- | :--- | :--- |
| `height` | DU | `--op-chip-height` | `var(--op-dim-row-dense)` |
| `gap` | DU | `--op-chip-gap` | `var(--op-dim-pad-tight)` |
| `padding_x` | DU | `--op-chip-padding-x` | `var(--op-dim-gutter)` |
| `remove_size` | DU | `--op-chip-remove-size` | `var(--op-dim-row-dense)` |
| `remove_overlap` | DU | `--op-chip-remove-overlap` | `var(--op-dim-pad-tight)` |
| `icon_size` | DU | `--op-chip-icon-size` | `calc(var(--op-du) * 3.5)` |
| `font_size` | TU | `--op-chip-font-size` | `var(--op-type-body-small-size)` |
| `line_height` | TU | `--op-chip-line-height` | `var(--op-type-body-small-line)` |
| Змінна кольору | Типове значення |
| :--- | :--- |
| `--op-chip-accent` | `var(--op-accent)` |
| `--op-chip-border` | `var(--op-border)` |
| `--op-chip-emphasis-border` | `var(--op-border-strong)` |
| `--op-chip-disabled-border` | `var(--op-disabled-border)` |
| `--op-chip-disabled-background` | `var(--op-disabled-surface)` |
| `--op-chip-error` | `var(--op-error)` |
| `--op-chip-hover` | `var(--op-hover)` |
| `--op-chip-info` | `var(--op-info)` |
| `--op-chip-pending` | `var(--op-pending)` |
| `--op-chip-pressed` | `var(--op-pressed)` |
| `--op-chip-ripple` | `var(--op-ripple)` |
| `--op-chip-selection` | `var(--op-selection)` |
| `--op-chip-success` | `var(--op-success)` |
| `--op-chip-background` | `var(--op-surface)` |
| `--op-chip-disabled-text` | `var(--op-text-disabled)` |
| `--op-chip-text` | `var(--op-text-primary)` |
| `--op-chip-secondary-text` | `var(--op-text-secondary)` |
| `--op-chip-warning` | `var(--op-warning)` |
| Слот | Розташування |
| :--- | :--- |
| `leading` | Перед типовим вмістом. |
| `trailing` | Після типового вмісту. |
| `content` | Замінює типовий текст або піктограму; зворотний зв’язок очікування та результату зберігає пріоритет. |

ChipSet також приймає кореневі `class` та `style`, а також проп `gap` у DU, підтримуваний змінною `--op-chip-set-gap` із `pad_tight` як запасним варіантом.
Наявне значення `ChipColor::Custom` записується перед стилем викликача, тому викликач може його перевизначити.

### Рядки

| Проп | Ключ | Типове значення |
| :--- | :--- | :--- |
| `text` | `chip.text` | порожній рядок |
| `remove_label` | `chip.remove` | `Remove {text}` |

`remove_label` є повним шаблоном; `{text}` замінюється текстом чипа, щоб переклади могли обирати порядок слів.

### Піктограми

| Проп | Ключ | Типове значення |
| :--- | :--- | :--- |
| `checked_icon` | `chip.checked` | `check` |
| `remove_icon` | `chip.remove` | `close` |

Піктограми, надані як дані елемента або через наявний проп `icon`, залишаються у власності викликача.
