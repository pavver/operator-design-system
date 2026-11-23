# Combobox

Текстове поле введення з відфільтрованим списком пропозицій.
Це [DropdownMenu](dropdown-menu.md), у якому можна шукати за допомогою введення тексту.

Статус: **чернетка** у вебреалізації.

## Використання

* Вибір одного значення з довгого списку: камери, зони, користувачі.
* З `allow_custom` — введення нового значення або вибір відомого, наприклад тегів.

## Геометрія

* Поле є [TextField](text-field.md) із кінцевою піктограмою `arrow_drop_down`.
* Список є спливним вікном DropdownMenu: під полем, на всю ширину, заввишки щонайбільше `50 DU`, з контуром `accent`.
* Частина кожної опції, що збіглася, відображається напівжирним шрифтом `accent`.
* Активна опція, переміщувана клавіатурою або вказівником, має тло `pressed`.
* Вибрана опція зберігає стиль вибраного елемента DropdownMenu.

## Поведінка

* Прокручування сторінки чи іншого контейнера поза списком закриває його; прокручування всередині списку чи в самому полі — ні.
* Фокус, клацання на полі або Arrow Down відкриває список з усіма опціями, незалежно від тексту в полі.
* Під час відкриття вибрана опція стає активною та прокручується до середини списку.
  Якщо нічого не вибрано, активною натомість стає перша опція, яка містить текст поля, або перша опція списку.
* Будь-яка зміна тексту перемикає список на фільтрацію за підрядком без урахування регістру та робить перший збіг активним.
* Переміщення активної опції клавіатурою прокручує її в область видимості.
* Опція вибирається клацанням або клавішею Enter на активній опції.
* Клавіші зі стрілками переміщують активну опцію із циклічним переходом.
* Escape закриває список і відновлює зафіксоване значення.
* Вихід із поля фіксує введений текст, який точно збігається з опцією.
  Інший текст фіксується лише з `allow_custom`; інакше поле повертається до зафіксованого значення.
* За відсутності збігів список показує "No matches", або "Press Enter to use this value" з `allow_custom`.
* Список пропозицій є [VirtualList](virtual-list.md): рендеряться лише видимі опції, тому списки з тисячами опцій залишаються швидкими.
* Список є шаром [Popover](popover.md) завширшки як поле, тому він не обрізається контейнерами з прокручуванням.

## Опції

Опція є звичайним рядком або `ComboOption`:

| Поле | Опис |
| :--- | :--- |
| `value` | Зафіксоване значення. |
| `label` | Текст у списку та в полі; значення, якщо порожньо. |
| `detail` | Вторинний текст у кінці рядка в стилі `caption`, наприклад тип чи одиниця вимірювання. |
| `icon` | Піктограма `4 DU` перед міткою в кольорі `text_secondary`. |
| `section` | Заголовок над групою; послідовні опції з однаковою секцією мають спільний заголовок. |

* Заголовки секцій є рядками списку однакової висоти в стилі `caption`, у верхньому регістрі кольором `text_secondary`, із лінією `divider` над усіма, крім першого.
* Введення тексту зіставляється з міткою або значенням; секції без збігів зникають.
* Опції можуть бути сигналом, тому список, що завантажується на вимогу, заповнюється, поки список відкритий.

## Типізовані значення

`parse_custom` перетворює введений текст на значення або відхиляє його за допомогою `None` (число в діапазоні, тривалість, ідентифікатор сутності).
Відхилений текст повертається до зафіксованого значення, подібно до невідомого тексту без `allow_custom`.

## ComboboxPopup

Той самий список із полем пошуку в плаваючому шарі, що відкривається в точці, без поля на сторінці: швидке додавання на полотні, меню зв'язків, командна палітра.

* Він відкривається біля якоря (типово в позиції вказівника) з порожнім полем пошуку у фокусі.
* Стрілки переміщують активну опцію, Enter або клацання вибирає її та закриває шар.
* Enter без збігів передає введений текст у `on_custom`, якщо його встановлено.
* Escape та натискання зовні закривають його.

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `open` | bool signal | обов'язковий | Показує шар. |
| `anchor` | `PopoverAnchor` signal | обов'язковий | Де він відкривається. |
| `options` | options | обов'язковий | Див. *Опції*. |
| `on_select` | callback `(ComboOption)` | обов'язковий | Викликається з вибраною опцією. |
| `on_custom` | callback `(string)` | немає | Enter із текстом і без збігів. |
| `placement` | `Placement` | `AtPoint` | Див. [Popover](popover.md). |
| `width` | DU | `80` | Ширина шару. |
| `placeholder`, `label`, `empty_text`, `custom_hint` | string | | Тексти. |

## API

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `options` | рядки, `ComboOption` або їхній сигнал | обов'язковий | Див. *Опції*. |
| `selected` | string signal | обов'язковий | Зафіксоване значення. |
| `placeholder` | string | порожньо | Підказка. |
| `label` | string | немає | Підпис над полем і доступна назва. |
| `allow_custom` | bool | `false` | Приймає значення поза списком. |
| `parse_custom` | callback `(string) -> option<string>` | немає | Валідує та нормалізує введені значення; передбачає `allow_custom`. |
| `on_select` | callback `(string)` | немає | Викликається під час кожної фіксації. |
| `enabled` | bool | `true` | Уможливлює введення. |
| `empty_text` | string | `No matches` | Рядок, що відображається, коли немає збігів. |
| `custom_hint` | string | `Press Enter to use this value` | Рядок, що відображається, коли немає збігів, але введений текст приймається. |

### API кастомізації

Кожне текстоподібне поле введення використовує публічну оболонку [Field](field.md).
Підписи видимі над полем; використовуйте `aria_label` лише для доступної назви.

| Властивість | Тип | Типове значення | Компоненти | Опис |
| :--- | :--- | :--- | :--- | :--- |
| `placeholder` | `string signal` | ключ рядка | Combobox, ComboboxPopup | Перекладний текст; див. Рядки. |
| `label` | `string signal` | немає | Combobox, ComboboxPopup | Видимий підпис і доступна назва. |
| `hint` | `string signal` | немає | Combobox, ComboboxPopup | Допоміжний текст під полем. |
| `error` | `string signal` | немає | Combobox, ComboboxPopup | Текст помилки замінює підказку та малює контур помилки. |
| `variant` | `FieldVariant signal` | outlined | Combobox, ComboboxPopup | Вигляд поля. |
| `aria_label` | `string signal` | ключ рядка | Combobox, ComboboxPopup | Доступна назва, коли підпис не встановлено. |
| `prepend` | `slot` | немає | Combobox, ComboboxPopup | Усередині поля перед елементом керування. |
| `append` | `slot` | немає | Combobox, ComboboxPopup | Усередині поля перед вбудованими елементами розкриття, збереження або вибору. |
| `before` | `slot` | немає | Combobox, ComboboxPopup | Зовні поля перед ним. |
| `after` | `slot` | немає | Combobox, ComboboxPopup | Зовні поля після нього. |
| `label_slot` | `slot` | немає | Combobox, ComboboxPopup | Замінює видимий підпис. |
| `hint_slot` | `slot` | немає | Combobox, ComboboxPopup | Замінює рядок підказки та помилки. |
| `height` | `number` | не встановлено | Combobox, ComboboxPopup | Перевизначення метрики; див. нижче. |
| `padding_x` | `number` | не встановлено | Combobox, ComboboxPopup | Перевизначення метрики; див. нижче. |
| `slot_size` | `number` | не встановлено | Combobox, ComboboxPopup | Перевизначення метрики; див. нижче. |
| `gap` | `number` | не встановлено | Combobox, ComboboxPopup | Перевизначення метрики; див. нижче. |
| `font_size` | `number` | не встановлено | Combobox, ComboboxPopup | Перевизначення метрики; див. нижче. |
| `class` | `string signal` | немає | Combobox, ComboboxPopup | Додатковий клас кореня. |
| `style` | `string signal` | немає | Combobox, ComboboxPopup | Додатковий стиль кореня. |
| `state` | `Signal<Option<ControlState>>` | немає | Combobox | Перевизначення зовнішнього стану елемента керування. |
| `icon` | `Option<IconData>` | ключ піктограми | Combobox | Перевизначення вбудованої піктограми; див. Піктограми. |
| `icon_size` | `number` | не встановлено | Combobox | Перевизначення метрики; див. нижче. |
| `row_height` | `number` | не встановлено | Combobox, ComboboxPopup | Перевизначення метрики; див. нижче. |
| `max_height` | `number` | не встановлено | Combobox, ComboboxPopup | Перевизначення метрики; див. нижче. |
| `option_gap` | `number` | не встановлено | Combobox, ComboboxPopup | Перевизначення метрики; див. нижче. |
| `option_padding_x` | `number` | не встановлено | Combobox, ComboboxPopup | Перевизначення метрики; див. нижче. |
| `option_icon_size` | `number` | не встановлено | Combobox, ComboboxPopup | Перевизначення метрики; див. нижче. |
| `detail_line_height` | `number` | не встановлено | Combobox, ComboboxPopup | Перевизначення метрики; див. нижче. |
| `letter_spacing` | `number` | не встановлено | Combobox, ComboboxPopup | Перевизначення метрики; див. нижче. |
| `detail_font_size` | `number` | не встановлено | Combobox, ComboboxPopup | Перевизначення метрики; див. нижче. |
| `section_padding_y` | `number` | не встановлено | Combobox, ComboboxPopup | Перевизначення метрики; див. нижче. |
| `option` | `Option<ScopedSlot<ComboOption>>` | немає | Combobox, ComboboxPopup | Вміст опції з областю видимості: ComboOption для Combobox, String для DropdownMenu. |
| `empty` | `slot` | немає | Combobox, ComboboxPopup | Замінює повідомлення про порожній список. |
| `before_options` | `slot` | немає | Combobox, ComboboxPopup | Перед рядками опцій. |
| `after_options` | `slot` | немає | Combobox, ComboboxPopup | Після рядків опцій. |
| `header` | `slot` | немає | Combobox, ComboboxPopup | Перед опціями спливного вікна; Calendar: над навігацією місяців. |
| `footer` | `slot` | немає | Combobox, ComboboxPopup | Після опцій спливного вікна; Calendar: під сіткою днів. |
| `width` | `f32` | `80 DU` | ComboboxPopup | Перевизначення метрики; див. нижче. |
| `screen_margin` | `number` | не встановлено | ComboboxPopup | Перевизначення метрики; див. нижче. |

## Доступність

Поле введення має роль `combobox` з `aria-expanded`; спливне вікно є `listbox` з елементами `option`.

## Кастомізація

`class` та `style` застосовуються до кореня; явні оголошення `style` йдуть після метричних пропсів і мають перевагу.
Невстановлені метричні пропси зберігають значення токенів як запасні варіанти.
Усі текстоподібні поля введення надають ті самі шість слотів поля.

| Метричний проп | Одиниця | Змінна | Типове значення | Компоненти |
| :--- | :--- | :--- | :--- | :--- |
| `height` | DU | `--op-field-height` | `row_standard; ComboboxPopup uses row_control` | Combobox, ComboboxPopup |
| `padding_x` | DU | `--op-field-padding-x` | `pad_card` | Combobox, ComboboxPopup |
| `slot_size` | DU | `--op-field-slot-size` | `row_standard` | Combobox, ComboboxPopup |
| `gap` | DU | `--op-field-gap` | `gap_micro` | Combobox, ComboboxPopup |
| `font_size` | TU | `--op-field-font-size` | `type_body_size` | Combobox, ComboboxPopup |
| `icon_size` | DU | `--op-field-icon-size` | `icon_small; 5 DU for picker arrows` | Combobox |
| `row_height` | DU | `--op-field-row-height` | `row_control` | Combobox, ComboboxPopup |
| `max_height` | DU | `--op-field-max-height` | `50 DU` | Combobox, ComboboxPopup |
| `option_gap` | DU | `--op-field-option-gap` | `pad_tight` | Combobox, ComboboxPopup |
| `option_padding_x` | DU | `--op-field-option-padding-x` | `pad_card` | Combobox, ComboboxPopup |
| `detail_font_size` | TU | `--op-field-detail-font-size` | `type_caption_size` | Combobox, ComboboxPopup |
| `detail_line_height` | TU | `--op-field-detail-line-height` | `type_caption_line` | Combobox, ComboboxPopup |
| `letter_spacing` | TU | `--op-field-letter-spacing` | `0.06em` | Combobox, ComboboxPopup |
| `option_icon_size` | DU | `--op-field-option-icon-size` | `icon_small` | Combobox, ComboboxPopup |
| `section_padding_y` | DU | `--op-field-section-padding-y` | `pad_tight` | Combobox, ComboboxPopup |
| `width` | DU | `--op-field-popup-width` | `80 DU` | ComboboxPopup |
| `screen_margin` | DU | `--op-field-screen-margin` | `margin_screen` | ComboboxPopup |

| Змінна кольору | Типове значення |
| :--- | :--- |
| `--op-field-background` | `surface` |
| `--op-field-text` | `text_primary` |
| `--op-field-pressed` | `pressed` |
| `--op-field-secondary-text` | `text_secondary` |
| `--op-field-accent` | `accent` |
| `--op-field-divider` | `divider` |
| `--op-field-input-background` | `surface_alt` |
| `--op-field-popup-border` | `accent` |
| `--op-field-disabled-background` | `disabled_surface` |
| `--op-field-disabled-text` | `text_disabled` |

| Слот | Розташування |
| :--- | :--- |
| `prepend` | Усередині поля перед елементом керування. |
| `append` | Усередині поля перед вбудованими елементами розкриття, збереження або вибору. |
| `before` | Зовні поля перед ним. |
| `after` | Зовні поля після нього. |
| `label_slot` | Замінює видимий підпис. |
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
| `combobox.placeholder` | `placeholder` | `(empty)` |
| `combobox.label` | `aria_label` | `Choose a value` |
| `combobox.empty` | `empty_text` | `No matches` |
| `combobox.custom-hint` | `custom_hint` | `Press Enter to use this value` |
| `combobox-popup.placeholder` | `placeholder` | `(empty)` |
| `combobox-popup.label` | `aria_label` | `Search` |

### Піктограми

| Ключ | Проп | Типове значення |
| :--- | :--- | :--- |
| `combobox.expand` | `icon` | `arrow_drop_down` |