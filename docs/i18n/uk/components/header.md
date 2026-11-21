# Header

Верхня панель екрана з навігацією назад, необов'язковим аватаром, заголовком, підзаголовком і діями.

Статус: **reviewed** у вебреалізації.

## Геометрія

| Властивість | Значення |
| :--- | :--- |
| Ширина | Заповнює екран. |
| Висота | `13.5 DU` за замовчуванням. |
| Нижня лінія | `0.25 DU` у `border`. |
| Кнопка повернення | Ширина `row_control` (`12 DU`), на всю висоту, піктограма `arrow_back` розміром `icon_medium` (`4.5 DU`). |
| Початковий внутрішній відступ | `margin_screen` (`4 DU`), коли кнопки повернення немає; `pad_small` після неї. |
| Аватар | `9 DU`, після нього проміжок `pad_content` (`2.5 DU`). |
| Назва | Стиль `app_title`, жирний; розмір `row_title` за наявності підзаголовка. |
| Підзаголовок | Стиль `caption`, `text_secondary`, один рядок із трикрапкою. |
| Дії | Слот із вирівнюванням праворуч. |
| Тло | `surface`. |

## Поведінка

* Кнопка повернення викликає свій зворотний виклик і показує хвилю натискання.
* За наявності зворотного виклику заголовка на область аватара та заголовка можна клікнути, наприклад, щоб відкрити профіль.
* Заголовок і підзаголовок усікаються, не витісняючи дії за межі панелі.

## API

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `title` | сигнал рядка | обов’язковий | Текст заголовка. |
| `subtitle` | сигнал рядка | немає | Текст підзаголовка. |
| `on_back` | зворотний виклик | немає | Зворотний виклик натискання кнопки повернення. |
| `avatar` | `HeaderAvatar` | немає | Аватар перед заголовком. |
| `on_title_click` | зворотний виклик | немає | Зворотний виклик кліку на області заголовка. |
| `actions` | слот | немає | Слот дій у кінці. |
| `height` | DU | `13.5` | Висота заголовка. |
| `back_width` | DU | `row_control` | Ширина кнопки повернення. |
| `gap_back` | DU | `pad_small` | Проміжок після кнопки повернення. |
| `padding_start` | DU | `margin_screen` | Початковий внутрішній відступ за відсутності кнопки повернення. |
| `gap_avatar` | DU | `pad_content` | Проміжок після аватара. |
| `title_size` | TU | `type_app_title_size` | Розмір шрифту заголовка. |
| `title_line_height` | TU | `type_app_title_line` | Висота рядка заголовка. |
| `title_small_size` | TU | `type_row_title_size` | Розмір шрифту заголовка за наявності підзаголовка. |
| `title_small_line_height` | TU | `type_row_title_line` | Висота рядка заголовка за наявності підзаголовка. |
| `subtitle_size` | TU | `type_caption_size` | Розмір шрифту підзаголовка. |
| `subtitle_line_height` | TU | `type_caption_line` | Висота рядка підзаголовка. |
| `actions_gap` | DU | `pad_tight` | Проміжок між елементами дій. |
| `actions_padding_end` | DU | `gutter` | Правий внутрішній відступ дій. |
| `back_label` | string | `header.back` | Доступна назва кнопки повернення. |
| `back_icon` | піктограма | `header.back` | Перевизначення піктограми кнопки повернення. |

## Кастомізація

| Метричний проп | Одиниця | Змінна | Типове значення |
| :--- | :--- | :--- | :--- |
| `height` | DU | `--op-header-height` | `13.5 DU` |
| `back_width` | DU | `--op-header-back-width` | `row_control` |
| `gap_back` | DU | `--op-header-gap-back` | `pad_small` |
| `padding_start` | DU | `--op-header-padding-start` | `margin_screen` |
| `gap_avatar` | DU | `--op-header-gap-avatar` | `pad_content` |
| `title_size` | TU | `--op-header-title-size` | `type_app_title_size` |
| `title_line_height` | TU | `--op-header-title-line-height` | `type_app_title_line` |
| `title_small_size` | TU | `--op-header-title-small-size` | `type_row_title_size` |
| `title_small_line_height` | TU | `--op-header-title-small-line-height` | `type_row_title_line` |
| `subtitle_size` | TU | `--op-header-subtitle-size` | `type_caption_size` |
| `subtitle_line_height` | TU | `--op-header-subtitle-line-height` | `type_caption_line` |
| `actions_gap` | DU | `--op-header-actions-gap` | `pad_tight` |
| `actions_padding_end` | DU | `--op-header-actions-padding-end` | `gutter` |

| Змінна кольору | Типове значення |
| :--- | :--- |
| `--op-header-background` | `surface` |
| `--op-header-border` | `border` |
| `--op-header-back-color` | `text_primary` |
| `--op-header-title-color` | `text_primary` |
| `--op-header-subtitle-color` | `text_secondary` |

| Слот | Розташування |
| :--- | :--- |
| `actions` | Область дій із вирівнюванням праворуч. |

`class` і `style` застосовуються до кореневого елемента заголовка.

### Рядки

| Ключ | Типове значення |
| :--- | :--- |
| `header.back` | `Back` |

### Піктограми

| Ключ | Типове значення |
| :--- | :--- |
| `header.back` | `arrow_back` |
