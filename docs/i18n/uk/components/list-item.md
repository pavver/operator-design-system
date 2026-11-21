# ListItem

Базовий рядок кожного списку: заголовок, необов'язковий підзаголовок, початковий і кінцевий слоти.

Статус: **reviewed** у вебреалізації.

## Геометрія

| Властивість | Значення |
| :--- | :--- |
| Ширина | Заповнює батьківський елемент. |
| Внутрішній відступ | `pad_card` (`3 DU`) по горизонталі, `gutter` (`2 DU`) по вертикалі. |
| Мінімальна висота | `row_avatar` (`14 DU`) із початковим вмістом, `row_two_line` (`13 DU`) із підзаголовком, інакше `row_standard` (`10 DU`). |
| Початковий проміжок | `pad_card` між початковим слотом і текстом. |
| Назва | Стиль `row_title`, насиченість 500, типово 1 рядок. |
| Підзаголовок | Стиль `body_small`, `text_secondary`, типово 2 рядки. |
| Розділювач | Лінія `0.25 DU` у кольорі `divider` під рядком. |
| Радіус заокруглення кутів | `0 DU`. |

Рядки зростають разом із вмістом; усічений текст закінчується трикрапкою.
Рядок з одним рядком тексту центрує вміст по вертикалі; із кількома — вирівнює його до верху.

## Стани

| Стан | Тло | Текст |
| :--- | :--- | :--- |
| Стан спокою | `background` | `text_primary` / `text_secondary` |
| Натиснутий | `pressed` із хвилею натискання | — |
| Вибраний | `selection` | — |
| Вимкнений | `disabled_surface` | `text_disabled` |

## Поведінка

* На рядок можна клікнути, лише коли він має обробник кліку й увімкнений.
* Кінцеві елементи керування, як-от перемикачі, обробляють введення незалежно від рядка.

## API

| Властивість | Тип | Типове значення |
| :--- | :--- | :--- |
| `title` | string | обов’язковий |
| `subtitle` | string | немає |
| `leading` | view | немає |
| `trailing` | view | немає |
| `selected` | bool | `false` |
| `enabled` | bool | `true` |
| `title_max_lines` | integer | `1` |
| `subtitle_max_lines` | integer | `2` |
| `on_click` | зворотний виклик | немає |

### API кастомізації

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `min_height` | необов'язкове число (DU) | `row_avatar` із початковим вмістом, `row_two_line` із підзаголовком, інакше `row_standard` | Перевизначає `--op-list-item-min-height`. |
| `padding_x` | необов'язкове число (DU) | `pad_card` | Перевизначає `--op-list-item-padding-x`. |
| `padding_y` | необов'язкове число (DU) | `gutter` | Перевизначає `--op-list-item-padding-y`. |
| `leading_padding` | необов'язкове число (DU) | `gap_micro` | Перевизначає `--op-list-item-leading-padding`. |
| `gap` | необов'язкове число (DU) | `pad_card` | Перевизначає `--op-list-item-gap`. |
| `text_padding` | необов'язкове число (DU) | `border` | Перевизначає `--op-list-item-text-padding`. |
| `subtitle_gap` | необов'язкове число (DU) | `0.75 DU` | Перевизначає `--op-list-item-subtitle-gap`. |
| `title_font_size` | необов'язкове число (TU) | `type_row_title_size` | Перевизначає `--op-list-item-title-font-size`. |
| `title_line_height` | необов'язкове число (TU) | `type_row_title_line` | Перевизначає `--op-list-item-title-line-height`. |
| `subtitle_font_size` | необов'язкове число (TU) | `type_body_small_size` | Перевизначає `--op-list-item-subtitle-font-size`. |
| `subtitle_line_height` | необов'язкове число (TU) | `type_body_small_line` | Перевизначає `--op-list-item-subtitle-line-height`. |
| `leading` | слот | немає | Перед текстом; зберігає обгортку початкового слота. |
| `trailing` | слот | немає | Після тексту; зберігає вирівнювання рядків із підзаголовком. |
| `content` | слот | немає | Замінює заголовок і підзаголовок усередині обгортки тексту. |
| `class` | необов'язковий рядок | немає | Додається до кореневих класів. |
| `style` | необов'язковий рядок | немає | Додається після кореневих оголошень метрик. |

## Кастомізація

`class` і `style` застосовуються до кореня.
Явні кореневі стилі мають перевагу над метричними пропсами; невстановлені метрики зберігають початковий рендеринг.

| Метричний проп | Одиниця | Змінна | Типове значення |
| :--- | :--- | :--- | :--- |
| `min_height` | DU | `--op-list-item-min-height` | `row_avatar` із початковим вмістом, `row_two_line` із підзаголовком, інакше `row_standard` |
| `padding_x` | DU | `--op-list-item-padding-x` | `pad_card` |
| `padding_y` | DU | `--op-list-item-padding-y` | `gutter` |
| `leading_padding` | DU | `--op-list-item-leading-padding` | `gap_micro` |
| `gap` | DU | `--op-list-item-gap` | `pad_card` |
| `text_padding` | DU | `--op-list-item-text-padding` | `border` |
| `subtitle_gap` | DU | `--op-list-item-subtitle-gap` | `0.75 DU` |
| `title_font_size` | TU | `--op-list-item-title-font-size` | `type_row_title_size` |
| `title_line_height` | TU | `--op-list-item-title-line-height` | `type_row_title_line` |
| `subtitle_font_size` | TU | `--op-list-item-subtitle-font-size` | `type_body_small_size` |
| `subtitle_line_height` | TU | `--op-list-item-subtitle-line-height` | `type_body_small_line` |

Товщина меж залишається фіксованою геометрією, вказаною вище.

| Змінна кольору | Типове значення |
| :--- | :--- |
| `--op-list-item-background` | `background` |
| `--op-list-item-pressed` | `pressed` |
| `--op-list-item-selected` | `selection` |
| `--op-list-item-disabled-background` | `disabled_surface` |
| `--op-list-item-text` | `text_primary` |
| `--op-list-item-disabled-text` | `text_disabled` |
| `--op-list-item-secondary-text` | `text_secondary` |
| `--op-list-item-divider` | `divider` |
| `--op-list-item-hover` | `hover` |

| Слот | Розташування |
| :--- | :--- |
| `leading` | Перед текстом; зберігає обгортку початкового слота. |
| `trailing` | Після тексту; зберігає вирівнювання рядків із підзаголовком. |
| `content` | Замінює заголовок і підзаголовок усередині обгортки тексту. |

### Рядки

Немає вбудованих рядків.
Видимий текст надає викликач; він може визначатися через пошук рядків застосунком.

### Піктограми

Немає вбудованих піктограм.
