# DataTable

Таблиця текстових комірок із сортуванням стовпців, закріпленим заголовком і вибором рядків.

Статус: **чернетка** у вебреалізації.

## Використання

* Списки записів, які порівнюють за кількома атрибутами: камери, події, користувачі.
* Для записів з одним або двома атрибутами [ListItem](list-item.md) зручніше читати на вузьких екранах.

## Геометрія

| Властивість | Значення |
| :--- | :--- |
| Контейнер | Контур `0.25 DU` у кольорі `border`; прокручується в обох напрямках. |
| Рядок заголовків | `8 DU`, `surface_alt`, стиль `caption` великими літерами в кольорі `text_secondary`, знизу `0.25 DU` у кольорі `border`; закріплений. |
| Рядок даних | `10 DU`, щільний `8 DU`; стиль `body_small`; знизу `0.25 DU` у кольорі `divider`. |
| Внутрішні відступи комірки | `3 DU` по горизонталі. |
| Ширина стовпця | Фіксована в DU або частка решти ширини. |
| Числові стовпці | Вирівнювання до кінця, табличні цифри. |
| Смуга завантаження | [LinearProgress](linear-progress.md) заввишки `0.5 DU` під заголовком. |

## Сортування

* Заголовок із сортуванням є кнопкою; при наведенні показує стрілку `3.5 DU` з непрозорістю 50%, а коли стовпець відсортовано — у кольорі `accent`.
* Кожен клік перемикає порядок: за зростанням, за спаданням, початковий.
* Числові стовпці порівнюють числа; комірки без числа опиняються наприкінці.
* Текст порівнюється без урахування регістру.

## Вибір

* За наявності вибору або обробника рядка рядки можуть отримувати фокус і показують фон `pressed` при наведенні.
* Вибраний рядок має фон `selection` і смугу `0.5 DU` у кольорі `accent` у першій комірці.
* Enter або Space на рядку у фокусі вибирає його.

## Порожній стан і завантаження

* Порожня таблиця показує «No data» або переданий вміст, зазвичай компактний [EmptyState](empty-state.md).
* `loading` показує невизначену смугу прогресу; рядки залишаються видимими.

## API

Поля `Column`: `title`, `width` (DU), `align`, `sortable`, `numeric`.
Поля `TableRow`: `id`, `cells`.

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `columns` | список стовпців | обов’язковий | Стовпці. |
| `rows` | список рядків | обов’язковий | Дані. |
| `selected` | необов'язковий сигнал ідентифікатора | немає | Вмикає одиничний вибір. |
| `on_row_click` | зворотний виклик `(id)` | немає | Активація рядка. |
| `loading` | bool | `false` | Показує смугу. |
| `dense` | bool | `false` | Рядки `8 DU`. |
| `max_height` | DU | немає | Прокручується під закріпленим заголовком. |
| `empty` | view | «No data» | Вміст порожнього стану. |
| `label` | string | немає | Доступна назва. |

### API кастомізації

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `row_height` | необов'язкове число (DU) | `row_standard` | Перевизначає `--op-data-table-row-height`. |
| `dense_row_height` | необов'язкове число (DU) | `row_compact` | Перевизначає `--op-data-table-dense-row-height`. |
| `header_height` | необов'язкове число (DU) | `row_compact` | Перевизначає `--op-data-table-header-height`. |
| `padding_x` | необов'язкове число (DU) | `pad_card` | Перевизначає `--op-data-table-padding-x`. |
| `sort_gap` | необов'язкове число (DU) | `pad_tight` | Перевизначає `--op-data-table-sort-gap`. |
| `sort_padding` | необов'язкове число (DU) | `pad_tight` | Перевизначає `--op-data-table-sort-padding`. |
| `icon_size` | необов'язкове число (DU) | `3.5 DU` | Перевизначає `--op-data-table-icon-size`. |
| `loading_thickness` | необов'язкове число (DU) | `0.5 DU` | Перевизначає `--op-data-table-loading-thickness`. |
| `empty_padding` | необов'язкове число (DU) | `section_gap` | Перевизначає `--op-data-table-empty-padding`. |
| `font_size` | необов'язкове число (TU) | `type_body_small_size` | Перевизначає `--op-data-table-font-size`. |
| `line_height` | необов'язкове число (TU) | `type_body_small_line` | Перевизначає `--op-data-table-line-height`. |
| `header_font_size` | необов'язкове число (TU) | `type_caption_size` | Перевизначає `--op-data-table-header-font-size`. |
| `header_letter_spacing` | необов'язкове число (TU) | `0.04 × type_caption_size` | Перевизначає `--op-data-table-header-letter-spacing`. |
| `max_height` | необов'язкове число (DU) | `none` | Перевизначає `--op-data-table-max-height`. |
| `row` | слот з областю видимості | немає | Із областю видимості `TableRow`; замінює комірки всередині нативного рядка даних; повертає елементи `td`. |
| `cell` | слот з областю видимості | немає | Із областю видимості `TableCell { row, column_index, column, value }`; замінює вміст нативної комірки даних. |
| `header_cell` | слот з областю видимості | немає | Із областю видимості `TableHeaderCell { column_index, column }`; замінює вміст заголовка всередині наявної кнопки сортування, якщо сортування доступне. |
| `empty` | слот | немає | Комірка порожньої таблиці; типово рядок `data-table.empty`. |
| `header` | слот | немає | Перед таблицею з прокручуванням. |
| `footer` | слот | немає | Після таблиці з прокручуванням. |
| `before_options` | слот | немає | Нативний рядок таблиці на всю ширину перед рядками даних. |
| `after_options` | слот | немає | Нативний рядок таблиці на всю ширину після рядків даних. |
| `label` | необов'язковий рядок | `data-table.label` | Перевизначення тексту. |
| `empty_text` | необов'язковий рядок | `data-table.empty` | Перевизначення тексту. |
| `loading_label` | необов'язковий рядок | `data-table.loading` | Перевизначення тексту. |
| `sort_ascending_label` | необов'язковий рядок | `data-table.sort-ascending` | Перевизначення тексту. |
| `sort_descending_label` | необов'язковий рядок | `data-table.sort-descending` | Перевизначення тексту. |
| `sort_clear_label` | необов'язковий рядок | `data-table.sort-clear` | Перевизначення тексту. |
| `sort_ascending_icon` | необов'язкова піктограма | `data-table.sort-ascending` | Перевизначення піктограми. |
| `sort_descending_icon` | необов'язкова піктограма | `data-table.sort-descending` | Перевизначення піктограми. |
| `class` | необов'язковий рядок | немає | Додається до кореневих класів. |
| `style` | необов'язковий рядок | немає | Додається після кореневих оголошень метрик. |

## Доступність

Нативна семантика таблиці; заголовки із сортуванням мають `aria-sort`, вибрані рядки — `aria-selected`.

## Кастомізація

`class` і `style` застосовуються до кореня.
Явні кореневі стилі мають перевагу над метричними пропсами; невстановлені метрики зберігають початковий рендеринг.
Слот рядка має пріоритет над слотом комірки.
Заголовки стовпців і значення комірок залишаються вмістом викликача.
Назви кнопок сортування поєднують заголовок стовпця з визначеним рядком дії; `aria-sort` зберігає стандартні значення переліку ARIA.

| Метричний проп | Одиниця | Змінна | Типове значення |
| :--- | :--- | :--- | :--- |
| `row_height` | DU | `--op-data-table-row-height` | `row_standard` |
| `dense_row_height` | DU | `--op-data-table-dense-row-height` | `row_compact` |
| `header_height` | DU | `--op-data-table-header-height` | `row_compact` |
| `padding_x` | DU | `--op-data-table-padding-x` | `pad_card` |
| `sort_gap` | DU | `--op-data-table-sort-gap` | `pad_tight` |
| `sort_padding` | DU | `--op-data-table-sort-padding` | `pad_tight` |
| `icon_size` | DU | `--op-data-table-icon-size` | `3.5 DU` |
| `loading_thickness` | DU | `--op-data-table-loading-thickness` | `0.5 DU` |
| `empty_padding` | DU | `--op-data-table-empty-padding` | `section_gap` |
| `font_size` | TU | `--op-data-table-font-size` | `type_body_small_size` |
| `line_height` | TU | `--op-data-table-line-height` | `type_body_small_line` |
| `header_font_size` | TU | `--op-data-table-header-font-size` | `type_caption_size` |
| `header_letter_spacing` | TU | `--op-data-table-header-letter-spacing` | `0.04 × type_caption_size` |
| `max_height` | DU | `--op-data-table-max-height` | `none` |

Товщина меж залишається фіксованою геометрією, вказаною вище.

| Змінна кольору | Типове значення |
| :--- | :--- |
| `--op-data-table-border` | `border` |
| `--op-data-table-header-background` | `surface_alt` |
| `--op-data-table-secondary-text` | `text_secondary` |
| `--op-data-table-text` | `text_primary` |
| `--op-data-table-accent` | `accent` |
| `--op-data-table-divider` | `divider` |
| `--op-data-table-hover` | `hover` |
| `--op-data-table-selected` | `selection` |
| `--op-data-table-loading` | `pending` |
| `--op-data-table-loading-track` | `color-mix(in srgb, pending 20%, surface_alt)` |

| Слот | Розташування |
| :--- | :--- |
| `row` | Із областю видимості `TableRow`; замінює комірки всередині нативного рядка даних; повертає елементи `td`. |
| `cell` | Із областю видимості `TableCell { row, column_index, column, value }`; замінює вміст нативної комірки даних. |
| `header_cell` | Із областю видимості `TableHeaderCell { column_index, column }`; замінює вміст заголовка всередині наявної кнопки сортування, якщо сортування доступне. |
| `empty` | Комірка порожньої таблиці; типово рядок `data-table.empty`. |
| `header` | Перед таблицею з прокручуванням. |
| `footer` | Після таблиці з прокручуванням. |
| `before_options` | Нативний рядок таблиці на всю ширину перед рядками даних. |
| `after_options` | Нативний рядок таблиці на всю ширину після рядків даних. |

### Рядки

Порядок розв'язання: проп, пошук застосунком, англійське типове значення.

| Ключ | Типове значення |
| :--- | :--- |
| `data-table.label` | `empty; preserves the unnamed table` |
| `data-table.empty` | `No data` |
| `data-table.loading` | `Loading rows` |
| `data-table.sort-ascending` | `Sort ascending` |
| `data-table.sort-descending` | `Sort descending` |
| `data-table.sort-clear` | `Clear sorting` |

### Піктограми

| Ключ | Типове значення |
| :--- | :--- |
| `data-table.sort-ascending` | `arrow_upward` |
| `data-table.sort-descending` | `arrow_downward` |
