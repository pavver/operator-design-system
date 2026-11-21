# PropertyList

Список пар підписів і значень, як-от відомостей про пристрій або налаштувань з'єднання.

Статус: **чернетка** у вебреалізації.

## Геометрія

| Властивість | Значення |
| :--- | :--- |
| Рядок | Щонайменше `10 DU`, вертикальний внутрішній відступ `1 DU`, знизу `0.25 DU` у кольорі `divider`. |
| Стовпці | Підпис 40%, значення 60%, проміжок `2 DU`. |
| Підпис | `text_secondary`. |
| Значення | `text_primary`; довгі значення переносяться й можуть розриватися будь-де. |
| Кнопка копіювання | [IconButton](icon-button.md) розміром `6 DU` із `content_copy`, у кінці значення. |

Коли список вужчий за `60 DU`, підпис переміщується над значенням у стилі `caption`.

## Поведінка

* Кнопка копіювання записує значення в буфер обміну як асинхронна дія, тому показує позначку успіху або пульсацію помилки.
* Значення може бути довільним представленням, наприклад [StatusBadge](status-badge.md).

## API

| Компонент | Властивості |
| :--- | :--- |
| `PropertyList` | `children`: властивості. |
| `Property` | `label`, `value` (рядок) або `children` (представлення), `copyable` (`false`). |

### API кастомізації

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `row_height` | необов'язкове число (DU) | `row_standard` | Перевизначає `--op-property-list-row-height`. |
| `padding_y` | необов'язкове число (DU) | `pad_tight` | Перевизначає `--op-property-list-padding-y`. |
| `gap` | необов'язкове число (DU) | `gutter` | Перевизначає `--op-property-list-gap`. |
| `value_gap` | необов'язкове число (DU) | `pad_tight` | Перевизначає `--op-property-list-value-gap`. |
| `copy_size` | необов'язкове число (DU) | `row_dense` | Перевизначає `--op-property-list-copy-size`. |
| `icon_size` | необов'язкове число (DU) | `3.5 DU` | Перевизначає `--op-property-list-icon-size`. |
| `stack_width` | необов'язкове число (DU) | `60 DU` | Перевизначає `--op-property-list-stack-width`. |
| `font_size` | необов'язкове число (TU) | `inherit` | Перевизначає `--op-property-list-font-size`. |
| `line_height` | необов'язкове число (TU) | `inherit` | Перевизначає `--op-property-list-line-height`. |
| `label_font_size` | необов'язкове число (TU) | `type_caption_size` | Перевизначає `--op-property-list-label-font-size`. |
| `label_line_height` | необов'язкове число (TU) | `type_caption_line` | Перевизначає `--op-property-list-label-line-height`. |
| `option` | слот з областю видимості | немає | Із областю видимості `PropertyValue { label, value }`; замінює терміни й описи рядка Property; повертає елементи `dt` і `dd`. |
| `value_slot` | слот з областю видимості | немає | Із областю видимості `PropertyValue`; замінює вміст значення всередині `dd`; копіювання далі використовує початковий сигнал значення. |
| `empty` | слот | немає | Використовується, коли PropertyList не має дочірніх елементів; типовий стан залишається порожнім. |
| `header` | слот | немає | Над властивостями. |
| `footer` | слот | немає | Під властивостями. |
| `before_options` | слот | немає | Перед властивостями. |
| `after_options` | слот | немає | Після властивостей. |
| `copy_label` | необов'язковий рядок | `property-list.copy` | Перевизначення тексту. |
| `copy_icon` | необов'язкова піктограма | `property-list.copy` | Перевизначення піктограми. |
| `class` | необов'язковий рядок | немає | Додається до кореневих класів. |
| `style` | необов'язковий рядок | немає | Додається після кореневих оголошень метрик. |

## Доступність

Список описів: підписи є термінами, значення — описами.

## Кастомізація

`class` і `style` застосовуються до кореня.
Явні кореневі стилі мають перевагу над метричними пропсами; невстановлені метрики зберігають початковий рендеринг.
Метрика рядка й кореневі хуки доступні як у PropertyList, так і в Property.
Метрика `stack_width` належить PropertyList.
Список надає успадковані зворотні виклики `option` і `value_slot`; Property може перевизначити `value_slot`.
`copy_label` і `copy_icon` належать Property.
`stack_width` керує точкою переходу списку; задавайте її на PropertyList.
Внутрішній ResizeObserver порівнює виміряну ширину списку з точкою переходу в DU та зберігає початкове вузьке компонування.

| Метричний проп | Одиниця | Змінна | Типове значення |
| :--- | :--- | :--- | :--- |
| `row_height` | DU | `--op-property-list-row-height` | `row_standard` |
| `padding_y` | DU | `--op-property-list-padding-y` | `pad_tight` |
| `gap` | DU | `--op-property-list-gap` | `gutter` |
| `value_gap` | DU | `--op-property-list-value-gap` | `pad_tight` |
| `copy_size` | DU | `--op-property-list-copy-size` | `row_dense` |
| `icon_size` | DU | `--op-property-list-icon-size` | `3.5 DU` |
| `stack_width` | DU | `--op-property-list-stack-width` | `60 DU` |
| `font_size` | TU | `--op-property-list-font-size` | `inherit` |
| `line_height` | TU | `--op-property-list-line-height` | `inherit` |
| `label_font_size` | TU | `--op-property-list-label-font-size` | `type_caption_size` |
| `label_line_height` | TU | `--op-property-list-label-line-height` | `type_caption_line` |

Товщина меж залишається фіксованою геометрією, вказаною вище.

| Змінна кольору | Типове значення |
| :--- | :--- |
| `--op-property-list-divider` | `divider` |
| `--op-property-list-label` | `text_secondary` |
| `--op-property-list-text` | `text_primary` |

| Слот | Розташування |
| :--- | :--- |
| `option` | Із областю видимості `PropertyValue { label, value }`; замінює терміни й описи рядка Property; повертає елементи `dt` і `dd`. |
| `value_slot` | Із областю видимості `PropertyValue`; замінює вміст значення всередині `dd`; копіювання далі використовує початковий сигнал значення. |
| `empty` | Використовується, коли PropertyList не має дочірніх елементів; типовий стан залишається порожнім. |
| `header` | Над властивостями. |
| `footer` | Під властивостями. |
| `before_options` | Перед властивостями. |
| `after_options` | Після властивостей. |

### Рядки

Порядок розв'язання: проп, пошук застосунком, англійське типове значення.

| Ключ | Типове значення |
| :--- | :--- |
| `property-list.copy` | `Copy value` |

Кнопка копіювання зберігає ключі зворотного зв'язку IconButton `icon-button.loading`, `icon-button.success` та `icon-button.error`.

### Піктограми

| Ключ | Типове значення |
| :--- | :--- |
| `property-list.copy` | `content_copy` |

Кнопка копіювання зберігає ключі піктограм IconButton `icon-button.success` та `icon-button.error`.
