# CodeField

Багаторядкове введення коду з підсвічуванням синтаксису, діагностикою та автодоповненням: шаблони, формули, вирази.
Він специфічний для проєкту: фіча `code-field` перебуває в групі `extensions`, а не в `full`.
Мова живе в застосунку, який передає підсвічувач і джерело автодоповнення.

Статус: **чернетка** у вебреалізації.

## Геометрія

| Властивість | Значення |
| :--- | :--- |
| Field | Повна ширина, `surface` зі станами контуру поля; вертикальний відступ `pad_tight` та горизонтальний відступ `pad_card`. |
| Text | Стиль шрифту `console` гарнітурою Operator, перенесення будь-де; поле росте разом із текстом, починаючи з `min_lines` (`2`). |
| Colours | Токени `syntax_*` відповідно до `CodeToken`. |
| Diagnostics | Хвилясте підкреслення `error` або `warning`; перше повідомлення відображається під текстом у стилі `caption`. |
| Completion | Список у стилі [DropdownMenu](dropdown-menu.md) під кареткою, завширшки `50–90 DU`, варіанти з деталями. |

## Поведінка

* Прозорий елемент textarea розташований над підсвіченим текстом, тому редагування, виділення та скасування дій браузера працюють як зазвичай.
* `highlight(text)` повертає елементи `CodeSpan` (діапазон байтів, токен, необов’язкове повідомлення); синтаксичні діапазони забарвлюють текст, діапазони помилок і попереджень підкреслюють його.
* `complete((text, caret))` виконується під час кожного редагування та за натисканням Ctrl+Space; результат (`CodeCompletion`: замінений діапазон байтів та `ComboOption`) відкриває список біля каретки.
* Поки список відкритий, стрілки вгору та вниз переміщують вибір, Enter або Tab виконує вставку, Escape закриває; клік також виконує вставку.
* Вставка здійснюється через `insertText`, тому Ctrl+Z скасовує її.
* Вихід із поля або Ctrl+Enter фіксує значення через `on_commit`, якщо текст змінився.
* Позиція каретки вимірюється в немасштабованому дзеркалі, тому автодоповнення працює всередині масштабованого полотна.

## API

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `value` | string signal | обов’язковий | Текст. |
| `highlight` | callback `(text) -> Vec<CodeSpan>` | немає | Кольори та діагностика. |
| `complete` | callback `((text, caret byte)) -> Option<CodeCompletion>` | немає | Джерело автодоповнення. |
| `on_commit` | callback `(text)` | немає | Втрата фокусу або Ctrl+Enter зі зміненим текстом. |
| `placeholder`, `label` | optional string | `code-field.placeholder`, немає | Плейсхолдер і видима мітка; мітка також іменує textarea. |
| `enabled` | bool | `true` | Дозволяє введення. |
| `min_lines` | integer | `2` | Висота порожнього поля. |

CodeField використовує публічну оболонку [Field](field.md) для мітки, підказки, помилки, слотів, варіанта та поведінки очікування/вимкнення.
Діагностичне повідомлення залишається під полем; якщо надано `diagnostics_hover`, воно стає тригером HoverCard, що може отримувати фокус.

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `aria_label` | optional string | `code-field.label` | Доступна назва, коли мітку не задано. |
| `completion_label` | optional string | `code-field.completions` | Доступна назва списку автодоповнення. |
| `diagnostics_label` | optional string | `code-field.diagnostics` | Доступна назва ховера діагностики. |
| `hint`, `error` | optional string | немає | Допоміжний текст і помилка поля; помилка замінює підказку. |
| `variant` | FieldVariant signal | Outlined | Підтримує вбудоване редагування без меж. |
| `state` | optional ControlState signal | немає | Зовнішній стан очікування та результату; очікування блокує редагування та закриває автодоповнення. |
| `tab_size` | integer | 2 | Ширина табуляції в пробілах; `--op-field-tab-size`. |
| `class`, `style` | MaybeProp string | немає | Зовнішні кореневі хуки. |
| `prepend`, `append`, `before`, `after`, `label_slot`, `hint_slot`, `gutter`, `diagnostics_hover` | Slot | порожній | Іменований вміст, деталізований нижче. |
| `completion_item` | optional ScopedSlot<ComboOption> | немає | Користувацький вміст елемента автодоповнення; вибір та обробка клавіатури залишаються вбудованими. |

Усі наведені нижче метричні пропси є необов’язковими реактивними значеннями `MaybeProp<f32>`.

| Метричний проп | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `height` | число DU | 0; висота визначається вмістом | Записує змінну в таблиці «Кастомізація». |
| `padding_x` | число DU | pad_card | Записує змінну в таблиці «Кастомізація». |
| `slot_size` | число DU | row_standard | Записує змінну в таблиці «Кастомізація». |
| `gap` | число DU | gap_micro | Записує змінну в таблиці «Кастомізація». |
| `font_size` | число TU | type_console_size для редактора; type_body_size для автодоповнення | Записує змінну в таблиці «Кастомізація». |
| `padding_y` | число DU | pad_tight | Записує змінну в таблиці «Кастомізація». |
| `line_height` | число TU | type_console_line | Записує змінну в таблиці «Кастомізація». |
| `caption_font_size` | число TU | type_caption_size | Записує змінну в таблиці «Кастомізація». |
| `caption_line_height` | число TU | type_caption_line | Записує змінну в таблиці «Кастомізація». |
| `completion_min_width` | число DU | 50 | Записує змінну в таблиці «Кастомізація». |
| `completion_max_width` | число DU | 90 | Записує змінну в таблиці «Кастомізація». |
| `row_height` | число DU | row_control | Записує змінну в таблиці «Кастомізація». |
| `max_height` | число DU | 50 | Записує змінну в таблиці «Кастомізація». |
| `option_padding_x` | число DU | pad_card | Записує змінну в таблиці «Кастомізація». |
| `option_gap` | число DU | pad_tight | Записує змінну в таблиці «Кастомізація». |
| `option_icon_size` | число DU | icon_small | Записує змінну в таблиці «Кастомізація». |
| `detail_font_size` | число TU | type_caption_size | Записує змінну в таблиці «Кастомізація». |
| `detail_line_height` | число TU | type_caption_line | Записує змінну в таблиці «Кастомізація». |
| `section_padding_y` | число DU | pad_tight | Записує змінну в таблиці «Кастомізація». |
| `letter_spacing` | число TU | 0.06em для заголовків розділів автодоповнення | Записує змінну в таблиці «Кастомізація». |
| `gutter_width` | число DU | auto; ширина вмісту | Записує змінну в таблиці «Кастомізація». |
| `gutter_gap` | число DU | 0 | Записує змінну в таблиці «Кастомізація». |

## Кастомізація

`class` та `style` застосовуються до зовнішнього кореня, що оточує оболонку Field, діагностику та автодоповнення.
CodeField використовує змінні `--op-field-*`, спільні з іншими полями.
Елемент textarea та шар підсвічування мають однакові відступи, розмір шрифту, висоту рядка та розмір табуляції, завдяки чому текст і виміряна каретка залишаються вирівняними.
Дзеркало каретки копіює їхню обчислену CSS-геометрію, включно з перевизначеннями всередині масштабованого полотна.
Автодоповнення копіює змінні поля з кореня в портал, а його віртуальний список вимірює висоту рядка та максимальну висоту за тими самими змінними, що використовуються для рендерингу.
`min_lines` записує безрозмірну змінну `--op-field-min-lines` і зберігає свій чинний цілочисельний API.

| Метричний проп | Одиниця | Змінна | Типове значення |
| :--- | :--- | :--- | :--- |
| `height` | DU | `--op-field-height` | 0; висота визначається вмістом |
| `padding_x` | DU | `--op-field-padding-x` | pad_card |
| `slot_size` | DU | `--op-field-slot-size` | row_standard |
| `gap` | DU | `--op-field-gap` | gap_micro |
| `font_size` | TU | `--op-field-font-size` | type_console_size для редактора; type_body_size для автодоповнення |
| `padding_y` | DU | `--op-field-padding-y` | pad_tight |
| `line_height` | TU | `--op-field-line-height` | type_console_line |
| `caption_font_size` | TU | `--op-field-caption-font-size` | type_caption_size |
| `caption_line_height` | TU | `--op-field-caption-line-height` | type_caption_line |
| `completion_min_width` | DU | `--op-field-completion-min-width` | 50 |
| `completion_max_width` | DU | `--op-field-completion-max-width` | 90 |
| `row_height` | DU | `--op-field-row-height` | row_control |
| `max_height` | DU | `--op-field-max-height` | 50 |
| `option_padding_x` | DU | `--op-field-option-padding-x` | pad_card |
| `option_gap` | DU | `--op-field-option-gap` | pad_tight |
| `option_icon_size` | DU | `--op-field-option-icon-size` | icon_small |
| `detail_font_size` | TU | `--op-field-detail-font-size` | type_caption_size |
| `detail_line_height` | TU | `--op-field-detail-line-height` | type_caption_line |
| `section_padding_y` | DU | `--op-field-section-padding-y` | pad_tight |
| `letter_spacing` | TU | `--op-field-letter-spacing` | 0.06em для заголовків розділів автодоповнення |
| `gutter_width` | DU | `--op-field-gutter-width` | auto; ширина вмісту |
| `gutter_gap` | DU | `--op-field-gutter-gap` | 0 |

| Змінна кольору | Типове значення |
| :--- | :--- |
| `--op-field-background` | surface |
| `--op-field-text` | text_primary |
| `--op-field-secondary-text` | text_secondary |
| `--op-field-accent` | accent |
| `--op-field-disabled-background` | disabled_surface |
| `--op-field-disabled-text` | text_disabled |
| `--op-field-error` | error |
| `--op-field-warning` | warning |
| `--op-field-pressed` | pressed |
| `--op-field-divider` | divider |
| `--op-field-syntax-keyword` | syntax_keyword |
| `--op-field-syntax-string` | syntax_string |
| `--op-field-syntax-number` | syntax_number |
| `--op-field-syntax-comment` | syntax_comment |
| `--op-field-syntax-type` | syntax_type |
| `--op-field-syntax-function` | syntax_function |
| `--op-field-syntax-constant` | syntax_constant |
| `--op-field-syntax-property` | syntax_property |
| `--op-field-syntax-meta` | syntax_meta |
| `--op-field-syntax-punctuation` | syntax_punctuation |

Спільна оболонка Field також забезпечує свої стандартні стани контуру та результату.

| Слот | Розташування |
| :--- | :--- |
| `prepend, append` | Усередині рамки Field, перед редактором і після нього. |
| `before, after` | За межами рамки. |
| `label_slot` | Замінює видиму мітку. |
| `hint_slot` | Замінює вміст підказки та помилки Field. |
| `gutter` | Необов’язкова початкова колонка поруч із текстом, вирівняна за висотою рядка та вертикальним відступом редактора. |
| `completion_item` | Параметризований вміст елемента, що отримує `ComboOption`, зокрема значення, мітку, детальну інформацію та піктограму. |
| `diagnostics_hover` | Вміст HoverCard, прикріплений до першого діагностичного повідомлення; доступний за наведенням та фокусом клавіатури. |

### Рядки

| Ключ | Проп | Типове значення |
| :--- | :--- | :--- |
| `code-field.placeholder` | `placeholder` | Порожній рядок |
| `code-field.label` | `aria_label` | Code |
| `code-field.completions` | `completion_label` | Completions |
| `code-field.diagnostics` | `diagnostics_label` | Diagnostics |

### Піктограми

CodeField не має вбудованих піктограм або семантичних ключів піктограм.
Піктограми автодоповнення — це дані `ComboOption.icon`, що передаються викликачем, а `completion_item` може замінити їхнє відображення.

## Доступність

* Елемент textarea містить обчислену мітку та `aria-invalid`, коли відображається помилка діагностики або явна помилка.
* Список автодоповнення є іменованим `listbox` з елементами `option`; фокус залишається в textarea.
* Тригер ховера діагностики може отримувати фокус із клавіатури, якщо передано слот.