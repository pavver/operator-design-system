# TextArea

Багаторядкове текстове поле, яке зростає разом із вмістом і розмір якого може змінювати користувач.

Статус: **reviewed** у вебреалізації.

## Використання

* Нотатки, повідомлення, описи й інший довільний текст, довший за один рядок.
* Для одного рядка використовуйте [TextField](text-field.md).

## Геометрія

| Властивість | Значення |
| :--- | :--- |
| Ширина | Повна ширина свого контейнера. |
| Внутрішній відступ | `2.5 DU` по вертикалі, `3 DU` по горизонталі. |
| Висота | Зростає від `min_rows` до `max_rows` основних рядків плюс внутрішній відступ, потім прокручується. |
| Тло | `surface`; `disabled_surface` у вимкненому стані. |
| Контур | Контур стану, див. [states.md](../foundations/states.md). |
| Лічильник | Стиль `caption` під кінцевим краєм, на `1 DU` нижче поля. |
| Ручка зміни розміру | Трикутник `2 DU` у нижньому кінцевому куті, діагональні штрихи в `border_strong`; `text_secondary` при наведенні, `accent` під час перетягування. Смуга прокручування закінчується над ним. |

## Поведінка

* Фокус показує контур `Focused`.
* Висота слідує за вмістом при кожній зміні, зокрема зробленій викликачем.
* Зовнішній `state` показує Pending, Success або Error на контурі, як у [TextField](text-field.md).
* Із `max_length` введення зупиняється на межі, а лічильник показує `count / max`.
* Лічильник стає `warning` на 90% ліміту й `error` на ліміті.
* Типово висота визначається вмістом.
* Перетягування ручки задає висоту вручну, від `min_rows` до `resize_max_rows` рядків; довший текст прокручується.
* Задана вручну висота зберігається, доки користувач не зробить подвійний клік на ручці, що повертає визначення висоти за вмістом.
* Ручка прихована, коли поле вимкнене.

## API

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `value` | сигнал рядка | обов’язковий | Текст. |
| `placeholder` | string | порожній | Підказка в порожньому стані. |
| `label` | string | немає | Видимий підпис над полем і доступна назва. |
| `min_rows` | integer | `3` | Початкова висота в рядках. |
| `max_rows` | integer | `8` | Межа висоти до прокручування. |
| `resizable` | bool | `true` | Показує ручку зміни розміру. |
| `resize_max_rows` | integer | `20` | Найбільша висота в рядках, якої можна досягти ручкою. |
| `max_length` | integer | немає | Ліміт символів із лічильником. |
| `state` | стан елемента керування | немає | Зовнішній стан. |
| `enabled` | bool | `true` | Вмикає введення. |

### API кастомізації

Кожне текстоподібне поле введення використовує публічну оболонку [Field](field.md).
Підписи видимі над полем; використовуйте `aria_label` лише для доступної назви.

| Властивість | Тип | Типове значення | Компоненти | Опис |
| :--- | :--- | :--- | :--- | :--- |
| `placeholder` | `string signal` | ключ рядка | TextArea | Перекладний текст; див. Рядки. |
| `label` | `string signal` | немає | TextArea | Видимий підпис і доступна назва. |
| `hint` | `string signal` | немає | TextArea | Допоміжний текст під полем. |
| `error` | `string signal` | немає | TextArea | Текст помилки замінює підказку та малює контур помилки. |
| `variant` | `FieldVariant signal` | outlined | TextArea | Вигляд поля. |
| `aria_label` | `string signal` | ключ рядка | TextArea | Доступна назва, коли підпис не встановлено. |
| `prepend` | `slot` | немає | TextArea | Усередині поля перед елементом керування. |
| `append` | `slot` | немає | TextArea | Усередині поля перед вбудованими елементами розкриття, збереження або вибору. |
| `before` | `slot` | немає | TextArea | Зовні поля перед ним. |
| `after` | `slot` | немає | TextArea | Зовні поля після нього. |
| `label_slot` | `slot` | немає | TextArea | Замінює видиму мітку. |
| `hint_slot` | `slot` | немає | TextArea | Замінює рядок підказки та помилки. |
| `height` | `number` | не встановлено | TextArea | Перевизначення метрики; див. нижче. |
| `padding_x` | `number` | не встановлено | TextArea | Перевизначення метрики; див. нижче. |
| `slot_size` | `number` | не встановлено | TextArea | Перевизначення метрики; див. нижче. |
| `gap` | `number` | не встановлено | TextArea | Перевизначення метрики; див. нижче. |
| `font_size` | `number` | не встановлено | TextArea | Перевизначення метрики; див. нижче. |
| `class` | `string signal` | немає | TextArea | Додатковий клас кореня. |
| `style` | `string signal` | немає | TextArea | Додатковий стиль кореня. |
| `padding_y` | `number` | не встановлено | TextArea | Перевизначення метрики; див. нижче. |
| `line_height` | `number` | не встановлено | TextArea | Перевизначення метрики; див. нижче. |
| `grip_size` | `number` | не встановлено | TextArea | Перевизначення метрики; див. нижче. |
| `caption_font_size` | `number` | не встановлено | TextArea | Перевизначення метрики; див. нижче. |
| `caption_line_height` | `number` | не встановлено | TextArea | Перевизначення метрики; див. нижче. |
| `counter_gap` | `number` | не встановлено | TextArea | Перевизначення метрики; див. нижче. |
| `resize_label` | `string signal` | ключ рядка | TextArea | Перекладний текст; див. Рядки. |
| `counter_text` | `string signal` | ключ рядка | TextArea | Перекладний текст; див. Рядки. |
| `state` | `Signal<Option<ControlState>>` | немає | TextArea | Перевизначення зовнішнього стану елемента керування. |

## Кастомізація

`class` та `style` застосовуються до кореня; явні оголошення `style` йдуть після метричних пропсів і мають перевагу.
Невстановлені метричні пропси зберігають значення токенів як запасні варіанти.
Усі текстоподібні поля введення надають ті самі шість слотів поля.

| Метричний проп | Одиниця | Змінна | Типове значення | Компоненти |
| :--- | :--- | :--- | :--- | :--- |
| `height` | DU | `--op-field-height` | `content height; optional minimum box height` | TextArea |
| `padding_x` | DU | `--op-field-padding-x` | `pad_card` | TextArea |
| `slot_size` | DU | `--op-field-slot-size` | `row_standard` | TextArea |
| `gap` | DU | `--op-field-gap` | `gap_micro` | TextArea |
| `font_size` | TU | `--op-field-font-size` | `type_body_size` | TextArea |
| `padding_y` | DU | `--op-field-padding-y` | `pad_content` | TextArea |
| `line_height` | TU | `--op-field-line-height` | `type_body_line` | TextArea |
| `grip_size` | DU | `--op-field-grip-size` | `gutter` | TextArea |
| `counter_gap` | DU | `--op-field-counter-gap` | `pad_tight` | TextArea |
| `caption_font_size` | TU | `--op-field-caption-font-size` | `type_caption_size` | TextArea |
| `caption_line_height` | TU | `--op-field-caption-line-height` | `type_caption_line` | TextArea |

| Змінна кольору | Типове значення |
| :--- | :--- |
| `--op-field-background` | `surface` |
| `--op-field-text` | `text_primary` |
| `--op-field-disabled-background` | `disabled_surface` |
| `--op-field-accent` | `accent` |
| `--op-field-disabled-text` | `text_disabled` |
| `--op-field-grip` | `border_strong` |
| `--op-field-secondary-text` | `text_secondary` |
| `--op-field-warning` | `warning` |
| `--op-field-error` | `error` |

| Слот | Розташування |
| :--- | :--- |
| `prepend` | Усередині поля перед елементом керування. |
| `append` | Усередині поля перед вбудованими елементами розкриття, збереження або вибору. |
| `before` | Зовні поля перед ним. |
| `after` | Зовні поля після нього. |
| `label_slot` | Замінює видиму мітку. |
| `hint_slot` | Замінює рядок підказки та помилки. |

### Рядки

| Ключ | Проп | Типове значення |
| :--- | :--- | :--- |
| `text-area.placeholder` | `placeholder` | `(empty)` |
| `text-area.label` | `aria_label` | `Text` |
| `text-area.resize` | `resize_label` | `Drag to resize, double-click to fit the text` |
| `text-area.counter` | `counter_text` | `{count} / {max}` |

### Піктограми

Цей компонент не має вбудованих піктограм.
