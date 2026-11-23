# DropdownMenu

Поле вибору, що відкриває список варіантів безпосередньо під собою.

Статус: **чернетка** у вебреалізації.

## Геометрія

| Властивість | Значення |
| :--- | :--- |
| Ширина | Заповнює батьківський елемент; відкритий список має точно таку саму ширину, як поле. |
| Висота поля | `row_standard` (`10 DU`), внутрішній відступ `pad_card` (`3 DU`). |
| Підпис | Необов'язковий, над полем, стиль `caption`, проміжок `pad_tight`. |
| Список | Безпосередньо під полем, максимальна висота `50 DU`, далі прокручується. |
| Варіант | Мінімальна висота `row_control` (`12 DU`), внутрішній відступ `pad_card`. |
| Радіус заокруглення кутів | `0 DU`. |

## Кольори

| Частина | Колір |
| :--- | :--- |
| Поле | `surface`, `text_primary`, контур стану. |
| Відкрите поле | Стан `Selected`: контур `accent`. |
| Список | `surface` із контуром `accent`; бібліотечний [Scrollbar](scrollbar.md). |
| Вибраний варіант | Фон `accent` із непрозорістю 8%, жирний текст `accent`. |
| Натиснутий варіант | `pressed`. |
| Варіант у фокусі | Рамка `0.5 DU` у кольорі `accent` усередині, тло `hover`, якщо варіант не вибраний. |

## Поведінка

* Прокручування сторінки чи іншого контейнера поза списком закриває його; прокручування всередині списку чи в самому полі — ні.
* Клік на полі відкриває або закриває список.
* Клік поза списком закриває його.
* Вибір варіанта оновлює значення, викликає зворотний виклик і закриває список.
* У стані `Pending` поле ігнорує введення, а стрілку замінює [спінер із затримкою](../foundations/states.md#delayed-spinner).
* `Disabled` використовує кольори вимкненого стану та ігнорує введення.
* Список варіантів є [VirtualList](virtual-list.md): рендеряться лише видимі варіанти, тому довгі списки залишаються швидкими; відкриття прокручує вибраний варіант до середини.
* Список є шаром [Popover](popover.md) завширшки як поле, тому він не обрізається контейнерами з прокручуванням.

## API

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `options` | список рядків | обов’язковий | Варіанти. |
| `selected` | рядок для читання й запису | обов’язковий | Поточний варіант. |
| `state` | `ControlState?` | немає | Зовнішнє перевизначення стану. |
| `enabled` | bool | `true` | Інтерактивність. |
| `label` | string | немає | Підпис над полем. |
| `on_select` | зворотний виклик `(String)` | немає | Викликається після вибору. |

### API кастомізації

Кожне текстоподібне поле введення використовує публічну оболонку [Field](field.md).
Підписи видимі над полем; використовуйте `aria_label` лише для доступної назви.

| Властивість | Тип | Типове значення | Компоненти | Опис |
| :--- | :--- | :--- | :--- | :--- |
| `state` | `Signal<Option<ControlState>>` | немає | DropdownMenu | Перевизначення зовнішнього стану елемента керування. |
| `label` | `string signal` | немає | DropdownMenu | Видимий підпис і доступна назва. |
| `hint` | `string signal` | немає | DropdownMenu | Допоміжний текст під полем. |
| `error` | `string signal` | немає | DropdownMenu | Текст помилки замінює підказку та малює контур помилки. |
| `variant` | `FieldVariant signal` | outlined | DropdownMenu | Вигляд поля. |
| `aria_label` | `string signal` | ключ рядка | DropdownMenu | Доступна назва, коли підпис не встановлено. |
| `prepend` | `slot` | немає | DropdownMenu | Усередині поля перед елементом керування. |
| `append` | `slot` | немає | DropdownMenu | Усередині поля перед вбудованими елементами розкриття, збереження або вибору. |
| `before` | `slot` | немає | DropdownMenu | Зовні поля перед ним. |
| `after` | `slot` | немає | DropdownMenu | Зовні поля після нього. |
| `label_slot` | `slot` | немає | DropdownMenu | Замінює видиму мітку. |
| `hint_slot` | `slot` | немає | DropdownMenu | Замінює рядок підказки та помилки. |
| `height` | `number` | не встановлено | DropdownMenu | Перевизначення метрики; див. нижче. |
| `padding_x` | `number` | не встановлено | DropdownMenu | Перевизначення метрики; див. нижче. |
| `slot_size` | `number` | не встановлено | DropdownMenu | Перевизначення метрики; див. нижче. |
| `gap` | `number` | не встановлено | DropdownMenu | Перевизначення метрики; див. нижче. |
| `font_size` | `number` | не встановлено | DropdownMenu | Перевизначення метрики; див. нижче. |
| `class` | `string signal` | немає | DropdownMenu | Додатковий клас кореня. |
| `style` | `string signal` | немає | DropdownMenu | Додатковий стиль кореня. |
| `icon` | `Option<IconData>` | ключ піктограми | DropdownMenu | Перевизначення вбудованої піктограми; див. Піктограми. |
| `icon_size` | `number` | не встановлено | DropdownMenu | Перевизначення метрики; див. нижче. |
| `row_height` | `number` | не встановлено | DropdownMenu | Перевизначення метрики; див. нижче. |
| `max_height` | `number` | не встановлено | DropdownMenu | Перевизначення метрики; див. нижче. |
| `option_padding_x` | `number` | не встановлено | DropdownMenu | Перевизначення метрики; див. нижче. |
| `trigger_gap` | `number` | не встановлено | DropdownMenu | Перевизначення метрики; див. нижче. |
| `option` | `Option<ScopedSlot<String>>` | немає | DropdownMenu | Вміст опції з областю видимості: ComboOption для Combobox, String для DropdownMenu. |
| `empty` | `slot` | немає | DropdownMenu | Замінює повідомлення про порожній список. |
| `before_options` | `slot` | немає | DropdownMenu | Перед рядками опцій. |
| `after_options` | `slot` | немає | DropdownMenu | Після рядків опцій. |
| `header` | `slot` | немає | DropdownMenu | Перед опціями спливного вікна; Calendar: над навігацією місяців. |
| `footer` | `slot` | немає | DropdownMenu | Після опцій спливного вікна; Calendar: під сіткою днів. |
| `loading_label` | сигнал рядка | `dropdown-menu.loading` | DropdownMenu | Доступний підпис завантаження. |
| `spinner_size`, `spinner_stroke` | number | не встановлено | DropdownMenu | Розмір і товщина штриха спінера в DU. |

## Кастомізація

`class` та `style` застосовуються до кореня; явні оголошення `style` йдуть після метричних пропсів і мають перевагу.
Невстановлені метричні пропси зберігають значення токенів як запасні варіанти.
Усі текстоподібні поля введення надають ті самі шість слотів поля.

| Метричний проп | Одиниця | Змінна | Типове значення | Компоненти |
| :--- | :--- | :--- | :--- | :--- |
| `height` | DU | `--op-field-height` | `row_standard` | DropdownMenu |
| `padding_x` | DU | `--op-field-padding-x` | `pad_card` | DropdownMenu |
| `slot_size` | DU | `--op-field-slot-size` | `row_standard` | DropdownMenu |
| `gap` | DU | `--op-field-gap` | `gap_micro` | DropdownMenu |
| `font_size` | TU | `--op-field-font-size` | `type_body_size` | DropdownMenu |
| `icon_size` | DU | `--op-field-icon-size` | `icon_small` (`4 DU`); `icon_field` (`5 DU`) для стрілок вибору | DropdownMenu |
| `row_height` | DU | `--op-field-row-height` | `row_control` | DropdownMenu |
| `max_height` | DU | `--op-field-max-height` | `50 DU` | DropdownMenu |
| `option_padding_x` | DU | `--op-field-option-padding-x` | `pad_card` | DropdownMenu |
| `trigger_gap` | DU | `--op-field-trigger-gap` | `gutter` | DropdownMenu |
| `spinner_size` | DU | `--op-field-spinner-size` | `icon_small` (`4 DU`) | DropdownMenu |
| `spinner_stroke` | DU | `--op-field-spinner-stroke` | `spinner_stroke` | DropdownMenu |

| Змінна кольору | Типове значення |
| :--- | :--- |
| `--op-field-background` | `surface` |
| `--op-field-text` | `text_primary` |
| `--op-field-pending` | `pending` |
| `--op-field-pressed` | `pressed` |
| `--op-field-disabled-background` | `disabled_surface` |
| `--op-field-disabled-text` | `text_disabled` |
| `--op-field-secondary-text` | `text_secondary` |
| `--op-field-accent` | `accent` |

| Слот | Розташування |
| :--- | :--- |
| `prepend` | Усередині поля перед елементом керування. |
| `append` | Усередині поля перед вбудованими елементами розкриття, збереження або вибору. |
| `before` | Зовні поля перед ним. |
| `after` | Зовні поля після нього. |
| `label_slot` | Замінює видиму мітку. |
| `hint_slot` | Замінює рядок підказки та помилки. |
| `option` | Вміст опції з областю видимості: ComboOption для Combobox, String для DropdownMenu. |
| `empty` | Замінює повідомлення про порожній список. |
| `before_options` | Перед рядками опцій. |
| `after_options` | Після рядків опцій. |
| `header` | Перед опціями спливного вікна; Calendar: над навігацією місяців. |
| `footer` | Після опцій спливного вікна; Calendar: під сіткою днів. |

Обчислені метрики рядків також оновлюють геометрію віртуального прокручування та розкриття з клавіатури.
Вміст спливного вікна успадковує обчислені змінні `--op-field-*` від кореня поля.

### Рядки

| Ключ | Проп | Типове значення |
| :--- | :--- | :--- |
| `dropdown-menu.loading` | `loading_label` | `Loading` |
| `dropdown-menu.label` | `aria_label` | `Choose a value` |

### Піктограми

| Ключ | Проп | Типове значення |
| :--- | :--- | :--- |
| `dropdown-menu.expand` | `icon` | `arrow_drop_down` |
