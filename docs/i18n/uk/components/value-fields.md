# Поля значень

Типізовані текстові поля: NumberField, DurationField, TimeField, DateTimeField, DateRangeField і ColorField, а також редактори ColorPicker і ClockPicker.
Кожне виглядає як [TextField](text-field.md) і редагує одне типізоване значення через його текстову форму.

Статус: **чернетка** у вебреалізації.

## Використання

* NumberField: числа з необов'язковою одиницею (`21.5 °C`, `1200 kWh`), може бути порожнім.
* DurationField: тривалості зі знаком, як-от затримки та зміщення (`1h 30m`, `-15m`).
* TimeField: час доби (`07:30`) із циферблатом.
* ClockPicker: лише циферблат: перетягніть стрілку на годину, потім на хвилину.
* DateTimeField: локальні дата й час (`2026-11-17 07:30`) із календарем.
* DateRangeField: діапазон днів на кшталт `12 лип – 5 лис 2026` зі швидким вибором і двома пов'язаними місяцями.
* ColorField: колір як `#rrggbb` (або `#rrggbbaa` із прозорістю) зі спливним ColorPicker.
* ColorPicker: лише редактор кольору для панелей і діалогів: режими спектра, RGB, HSL і колірної температури, прозорість, код кольору та зразки.

## Геометрія

| Властивість | Значення |
| :--- | :--- |
| Поле | Як у [TextField](text-field.md): `row_standard` (`10 DU`), внутрішній відступ `pad_card`, табличні цифри. |
| Одиниця | Після значення, `text_secondary`, на відстані `pad_tight` від тексту. |
| Кнопка вибору | Завширшки `10 DU`, на всю висоту, `arrow_drop_down`; ColorField додає зразок кольору `icon_field` (`5 DU`) перед стрілкою. |
| Календар | Спливний шар із заголовком `row_compact` (попередній, місяць і рік, наступний), рядком днів тижня `row_dense` і квадратними комірками днів `row_compact`; тижні починаються з понеділка. |
| ColorPicker | Завширшки `80 DU`, внутрішній відступ `pad_content`, між рядками `gutter`; рядок вкладок режимів заввишки `row_compact` із підкресленням `0.5 DU` у кольорі `accent` на вибраній. |
| Вікно діапазону дат | Стовпчик `36 DU` швидких діапазонів `row_compact` зі смужкою `0.5 DU` у кольорі `accent` на відповідному й роздільником, далі два календарі поруч і підказка; на вузькому екрані календарі переносяться один під одний. |
| Сітки місяців і років | Три стовпці й чотири рядки на місці сітки днів, із тим самим заповненням `accent` для вибраного місяця чи року та контуром для поточного. |
| Дні діапазону | Кінці заповнені `accent`; дні між ними підфарбовані `accent` на 22%. |
| ClockPicker | Внутрішній відступ `pad_content`; показ із двох кнопок `row_avatar` розміром `type_screen_title_size` з підкресленням `0.5 DU` у кольорі `accent` на активній стрілці та необов'язковою парою AM/PM; круглий циферблат `64 DU` у `surface_alt` із числами `row_compact`, стрілкою `0.5 DU` у кольорі `accent` і круглим кінчиком `row_compact`. |
| Площина спектра | Заввишки `40 DU`, насиченість по горизонталі, яскравість угору; квадратний маркер `icon_small` (`4 DU`) зі світлим кільцем `0.5 DU` і темним контуром `0.25 DU`. |
| Рядок каналу | Підпис, смуга `row_dense` із маркером `pad_small`, що виступає на `gap_micro`, і числове поле `14 DU` заввишки `row_compact`; смуга прозорості має шахове тло. |
| Код кольору | Поле `row_compact` зі зразком `icon_field` (`5 DU`), кодом і CSS-формою поточного режиму. |
| Зразки | Квадрати `row_compact` на всю ширину з проміжком `gap_micro`, із контуром `0.5 DU` у кольорі `accent` на вибраному. |

## Винятки геометрії

Циферблат ClockPicker, його центр, кінчик стрілки й вибране число круглі; усе інше має кути `0 DU`.

## Поведінка

* Прокручування сторінки чи іншого контейнера поза спливним календарем або вибором кольору закриває їх; прокручування всередині них чи в самому полі — ні.
* Введення редагує лише текст; Enter або вихід із поля розбирає його та фіксує значення.
* Коректний текст переформатовується в канонічну форму.
* Некоректний текст відкочується до останнього значення, а поле пульсує станом помилки, як помилка відкочує зміну всюди.
* Escape відновлює текст поточного значення без фіксації.
* Arrow Up і Down змінюють значення кроками: NumberField на `step` у межах `min` і `max`, DurationField на `step_ms`, TimeField на `step_s` із переходом через північ.
* Зовнішні зміни значення відображаються, якщо поле не у фокусі.
* Заголовок календаря відкриває сітку місяців, а повторно — сітку з дванадцяти років; вибір року повертає до його місяців, а вибір місяця — до його днів; стрілки рухають на місяць, рік або дванадцять років.
* Календар зберігає час доби; вибраний день заповнений `accent`, сьогоднішній має контур `accent`.
* Спливні шари є шарами [Popover](popover.md) і закриваються при натисканні зовні або за Escape.
* DateRangeField: перше натискання на день починає діапазон, вказівник чи фокус показують його попередньо, а друге натискання завершує, фіксує й закриває вікно; дні можна вибирати в будь-якому порядку.
* Швидкий діапазон фіксується й закриває вікно одразу; той, що дорівнює значенню, позначено.
* Відкрите вікно показує місяць початку діапазону й наступний; їхні стрілки рухають обидва разом.
* ClockPicker спершу задає годину: перетягування рухає стрілку й значення наживо, відпускання фіксує його та перемикає на хвилини; відпускання хвилинної стрілки фіксує значення й викликає `on_done`, що закриває спливне вікно TimeField.
* У 24-годинному режимі зовнішнє коло містить `12` і `1`–`11`, а внутрішнє — `00` і `13`–`23`; з `hour12` циферблат показує `12` і `1`–`11`, а AM/PM перемикає половину доби.
* Хвилини прив'язуються до `minute_step`; секунди значення зберігаються.
* На циферблаті стрілки змінюють активну стрілку на годину або на `minute_step`, клавіші Page — на п'ять, а Enter переходить від годин до хвилин або завершує вибір.
* ColorPicker оновлює значення наживо під час перетягування площини чи смуги та викликає `on_commit`, коли редагування завершено: після відпускання, Enter у числовому полі чи коді, вибору зразка або натискання клавіші на смузі.
* На площині Arrow Left і Right змінюють насиченість, а Arrow Up і Down — яскравість; смуги змінюються стрілками, на десять кроків — із Shift або клавішами Page, а Home і End переносять на краї.
* Сірі кольори й чорний зберігають попередні відтінок і насиченість, тож перетягування через них не скидає інші смуги.
* Смуга температури йде від `min_kelvin` до `max_kelvin` уздовж кривої чорного тіла; поки колір задано нею, `temperature` містить значення в кельвінах, а будь-яке інше редагування встановлює `None`.
* Без `alpha` значення завжди непрозоре; з ним непрозорий колір лишається `#rrggbb`, а напівпрозорий стає `#rrggbbaa`.
* Некоректний текст у числовому полі чи полі коду відновлює показане значення.

## Текстові форми

| Поле | Приймається | Показується |
| :--- | :--- | :--- |
| NumberField | `1.5`, `1,5`, `12 000`, `-3e2` | До `decimals` десяткових знаків, без кінцевих нулів. |
| DurationField | `1h 30m`, `1.5h`, `90s`, `250ms`, `2d`, `1:30` (хвилини та секунди), `1:02:03`, число без суфікса в `default_unit_ms`, початковий `-` | `-1d 2h 3m 4s 500ms`, нуль як `0s`. |
| TimeField | `7:30`, `0730`, `7:30:15`, `9` | `07:30`, із секундами, коли вони ненульові або встановлено `seconds`. |
| DateTimeField | `2026-11-17 07:30`, `2026-11-17T07:30:15`, лише дата означає північ | `2026-11-17 07:30`. |
| DateRangeField | `12 Jul – 5 Nov 2026`, `3 – 10 Aug 2026`, `12 липня – 5 листопада`, `2026-07-12..2026-11-05`, `12.07 - 5.11.2026`, один день; з'єднані `–`, `—`, ` - `, `..` або ` to `; назви місяців зіставляються за першими трьома літерами; рік кінця без року — поточний, рік початку — рік кінця (на рік раніше, якщо початок інакше йшов би після кінця) | `12 Jul – 5 Nov 2026`, `3 – 10 Aug 2026`, `20 Dec 2026 – 3 Jan 2027`, `5 Nov 2026`. |
| ColorField, код у ColorPicker | `#abc`, `#abcd`, `#aabbcc`, `#aabbccdd`, те саме без `#`, `rgb()`, `rgba()`, `hsl()`, `hsla()`, `hsv()`, температура на кшталт `2700K` | `#aabbcc`, або `#aabbccdd` з `alpha` для напівпрозорих. |

Розбір і форматування містяться в `value_field::parse` і `value_field::color_math` та не потребують бібліотек дат чи кольорів.
Застосунки, яким потрібні локалізовані форми, обгортають [`ParsedField`](#parsedfield) власними `parse` і `format`.

## API

| Компонент | Значення | Властивості |
| :--- | :--- | :--- |
| `NumberField` | `Option<f64>` | `min`, `max`, `step` (`1`), `decimals` (`3`), `unit`, `allow_empty`. |
| `DurationField` | `i64` мілісекунд | `signed` (`true`), `default_unit_ms` (`1000`), `step_ms` (`1000`). |
| `TimeField` | `u32` секунд від півночі | `seconds`, `step_s` (`60`), `hour12`, `minute_step` (`1`), `no_picker`. |
| `ClockPicker` | `u32` секунд від півночі | `hour12`, `minute_step` (`1`), `enabled`, `on_commit`, `on_done`. |
| `DateTimeField` | `LocalDateTime` | `seconds`, `month_names`, `weekday_names`. |
| `ColorField` | `String` `#rrggbb` або `#rrggbbaa` | `swatches`, `alpha`, `modes`, `min_kelvin` (`2000`), `max_kelvin` (`6500`), `temperature`. |
| `ColorPicker` | `String` `#rrggbb` або `#rrggbbaa` | `swatches`, `alpha`, `modes` (`ColorMode::Spectrum`, `Rgb`, `Hsl`, `Temperature`; порожній означає всі), `min_kelvin`, `max_kelvin`, `temperature: Option<RwSignal<Option<u32>>>`, `enabled`, `on_commit`. |
| `DateRangeField` | `Option<DateRange>` | `presets` (`DateRangePreset::defaults()`), `allow_empty`, `short_month_names`, `month_names`, `weekday_names`. |
| `Calendar` | | `selected`, `on_pick`, `range`, `on_hover`, `month`, `hide_previous`, `hide_next`, `month_names`, `weekday_names`. |

`DateRange` містить включні дні `start` і `end` як `LocalDate` `(рік, місяць, день)`; `DateRangePreset` охоплює сьогодні, учора, цей і минулий тиждень, місяць і рік, останні `n` днів і власні діапазони.

Кожне поле, зокрема ColorField і ParsedField, приймає `label`, `placeholder`, `enabled`, `state` і `on_commit`.

## ParsedField

Спільна основа: `value`, `format` (значення в текст), `parse` (текст у значення або `None`), необов'язкові `on_step`, представлення `trailing` і `input_mode`.

### API кастомізації

Кожне текстоподібне поле введення використовує публічну оболонку [Field](field.md).
Підписи видимі над полем; використовуйте `aria_label` лише для доступної назви.

| Властивість | Тип | Типове значення | Компоненти | Опис |
| :--- | :--- | :--- | :--- | :--- |
| `placeholder` | `string signal` | ключ рядка | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Перекладний текст; див. Рядки. |
| `label` | `string signal` | немає | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Видимий підпис і доступна назва. |
| `hint` | `string signal` | немає | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Допоміжний текст під полем. |
| `error` | `string signal` | немає | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Текст помилки замінює підказку та малює контур помилки. |
| `variant` | `FieldVariant signal` | outlined | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Вигляд поля. |
| `aria_label` | `string signal` | ключ рядка | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Доступна назва, коли підпис не встановлено. |
| `prepend` | `slot` | немає | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Усередині поля перед елементом керування. |
| `append` | `slot` | немає | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Усередині поля перед вбудованими елементами розкриття, збереження або вибору. |
| `before` | `slot` | немає | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Зовні поля перед ним. |
| `after` | `slot` | немає | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Зовні поля після нього. |
| `label_slot` | `slot` | немає | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Замінює видиму мітку. |
| `hint_slot` | `slot` | немає | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Замінює рядок підказки та помилки. |
| `height` | `number` | не встановлено | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Перевизначення метрики; див. нижче. |
| `padding_x` | `number` | не встановлено | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Перевизначення метрики; див. нижче. |
| `slot_size` | `number` | не встановлено | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Перевизначення метрики; див. нижче. |
| `gap` | `number` | не встановлено | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Перевизначення метрики; див. нижче. |
| `font_size` | `number` | не встановлено | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField, Calendar | Перевизначення метрики; див. нижче. |
| `class` | `string signal` | немає | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField, Calendar | Додатковий клас кореня. |
| `style` | `string signal` | немає | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField, Calendar | Додатковий стиль кореня. |
| `unit_padding_x` | `number` | не встановлено | NumberField | Перевизначення метрики; див. нижче. |
| `unit_gap` | `number` | не встановлено | NumberField | Перевизначення метрики; див. нижче. |
| `state` | `Signal<Option<ControlState>>` | немає | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField | Перевизначення зовнішнього стану елемента керування. |
| `day_unit` | `string signal` | ключ рядка | DurationField | Перекладний текст; див. Рядки. |
| `hour_unit` | `string signal` | ключ рядка | DurationField | Перекладний текст; див. Рядки. |
| `minute_unit` | `string signal` | ключ рядка | DurationField | Перекладний текст; див. Рядки. |
| `second_unit` | `string signal` | ключ рядка | DurationField | Перекладний текст; див. Рядки. |
| `millisecond_unit` | `string signal` | ключ рядка | DurationField | Перекладний текст; див. Рядки. |
| `picker_label` | `string signal` | ключ рядка | DateTimeField, DateRangeField, ColorField, TimeField | Перекладний текст; див. Рядки. |
| `popup_label` | `string signal` | ключ рядка | DateTimeField, DateRangeField, ColorField, TimeField | Перекладний текст; див. Рядки. |
| `icon` | `Option<IconData>` | ключ піктограми | DateTimeField, DateRangeField, ColorField, TimeField | Перевизначення вбудованої піктограми; див. Піктограми. |
| `icon_size` | `number` | не встановлено | DateTimeField, DateRangeField, ColorField, TimeField, Calendar | Перевизначення метрики; див. нижче. |
| `popup_shadow_y` | `number` | не встановлено | DateTimeField, DateRangeField, ColorField, TimeField | Перевизначення метрики; див. нижче. |
| `popup_shadow_blur` | `number` | не встановлено | DateTimeField, DateRangeField, ColorField, TimeField | Перевизначення метрики; див. нижче. |
| `calendar_padding` | `number` | не встановлено | DateTimeField | Перевизначення метрики; див. нижче. |
| `calendar_cell_size` | `number` | не встановлено | DateTimeField | Перевизначення метрики; див. нижче. |
| `calendar_header_height` | `number` | не встановлено | DateTimeField | Перевизначення метрики; див. нижче. |
| `calendar_weekday_height` | `number` | не встановлено | DateTimeField | Перевизначення метрики; див. нижче. |
| `preview_size` | `number` | не встановлено | ColorField | Перевизначення метрики; див. нижче. |
| `picker_gap` | `number` | не встановлено | ColorField | Перевизначення метрики; див. нижче. |
| `picker_padding_x` | `number` | не встановлено | ColorField | Перевизначення метрики; див. нижче. |
| `popup_gap` | `number` | не встановлено | ColorField | Перевизначення метрики; див. нижче. |
| `popup_padding` | `number` | не встановлено | ColorField | Перевизначення метрики; див. нижче. |
| `swatch_size` | `number` | не встановлено | ColorField | Перевизначення метрики; див. нижче. |
| `picker_width` | `number` | не встановлено | ColorField | Перевизначення метрики; див. нижче. |
| `plane_height` | `number` | не встановлено | ColorField | Перевизначення метрики; див. нижче. |
| `bar_height` | `number` | не встановлено | ColorField | Перевизначення метрики; див. нижче. |
| `spectrum_label` | `string signal` | ключ рядка | ColorPicker | Перекладний текст; див. Рядки. |
| `rgb_label` | `string signal` | ключ рядка | ColorPicker | Перекладний текст; див. Рядки. |
| `hsl_label` | `string signal` | ключ рядка | ColorPicker | Перекладний текст; див. Рядки. |
| `temperature_label` | `string signal` | ключ рядка | ColorPicker | Перекладний текст; див. Рядки. |
| `plane_label` | `string signal` | ключ рядка | ColorPicker | Перекладний текст; див. Рядки. |
| `hue_label` | `string signal` | ключ рядка | ColorPicker | Перекладний текст; див. Рядки. |
| `saturation_label` | `string signal` | ключ рядка | ColorPicker | Перекладний текст; див. Рядки. |
| `brightness_label` | `string signal` | ключ рядка | ColorPicker | Перекладний текст; див. Рядки. |
| `lightness_label` | `string signal` | ключ рядка | ColorPicker | Перекладний текст; див. Рядки. |
| `red_label` | `string signal` | ключ рядка | ColorPicker | Перекладний текст; див. Рядки. |
| `green_label` | `string signal` | ключ рядка | ColorPicker | Перекладний текст; див. Рядки. |
| `blue_label` | `string signal` | ключ рядка | ColorPicker | Перекладний текст; див. Рядки. |
| `alpha_label` | `string signal` | ключ рядка | ColorPicker | Перекладний текст; див. Рядки. |
| `hex_label` | `string signal` | ключ рядка | ColorPicker | Перекладний текст; див. Рядки. |
| `swatches_label` | `string signal` | ключ рядка | ColorPicker | Перекладний текст; див. Рядки. |
| `width` | `number` | не встановлено | ColorPicker | Перевизначення метрики; див. нижче. |
| `padding` | `number` | не встановлено | ColorPicker | Перевизначення метрики; див. нижче. |
| `gap` | `number` | не встановлено | ColorPicker | Перевизначення метрики; див. нижче. |
| `plane_height` | `number` | не встановлено | ColorPicker | Перевизначення метрики; див. нижче. |
| `bar_height` | `number` | не встановлено | ColorPicker | Перевизначення метрики; див. нижче. |
| `thumb_size` | `number` | не встановлено | ColorPicker | Перевизначення метрики; див. нижче. |
| `tab_height` | `number` | не встановлено | ColorPicker | Перевизначення метрики; див. нижче. |
| `number_width` | `number` | не встановлено | ColorPicker | Перевизначення метрики; див. нижче. |
| `swatch_size` | `number` | не встановлено | ColorPicker | Перевизначення метрики; див. нижче. |
| `font_size` | `number` | не встановлено | ColorPicker | Перевизначення метрики; див. нижче. |
| `header` | `slot` | немає | ColorPicker | Над вкладками режимів. |
| `footer` | `slot` | немає | ColorPicker | Під зразками. |
| `presets_label` | `string signal` | ключ рядка | DateRangeField | Перекладний текст; див. Рядки. |
| `start_hint` | `string signal` | ключ рядка | DateRangeField | Перекладний текст; див. Рядки. |
| `end_hint` | `string signal` | ключ рядка | DateRangeField | Перекладний текст; див. Рядки. |
| `calendar_cell_size` | `number` | не встановлено | DateRangeField | Перевизначення метрики; див. нижче. |
| `presets_width` | `number` | не встановлено | DateRangeField | Перевизначення метрики; див. нижче. |
| `preset_height` | `number` | не встановлено | DateRangeField | Перевизначення метрики; див. нижче. |
| `clock_size` | `number` | не встановлено | TimeField | Перевизначення метрики; див. нижче. |
| `hours_label` | `string signal` | ключ рядка | ClockPicker | Перекладний текст; див. Рядки. |
| `minutes_label` | `string signal` | ключ рядка | ClockPicker | Перекладний текст; див. Рядки. |
| `am_label` | `string signal` | ключ рядка | ClockPicker | Перекладний текст; див. Рядки. |
| `pm_label` | `string signal` | ключ рядка | ClockPicker | Перекладний текст; див. Рядки. |
| `size` | `number` | не встановлено | ClockPicker | Перевизначення метрики; див. нижче. |
| `padding` | `number` | не встановлено | ClockPicker | Перевизначення метрики; див. нижче. |
| `gap` | `number` | не встановлено | ClockPicker | Перевизначення метрики; див. нижче. |
| `number_size` | `number` | не встановлено | ClockPicker | Перевизначення метрики; див. нижче. |
| `hand_width` | `number` | не встановлено | ClockPicker | Перевизначення метрики; див. нижче. |
| `readout_font_size` | `number` | не встановлено | ClockPicker | Перевизначення метрики; див. нижче. |
| `font_size` | `number` | не встановлено | ClockPicker | Перевизначення метрики; див. нижче. |
| `header` | `slot` | немає | ClockPicker | Над показом часу. |
| `footer` | `slot` | немає | ClockPicker | Під циферблатом. |
| `previous_label` | `string signal` | ключ рядка | Calendar | Перекладний текст; див. Рядки. |
| `next_label` | `string signal` | ключ рядка | Calendar | Перекладний текст; див. Рядки. |
| `jump_label` | `string signal` | ключ рядка | Calendar | Перекладний текст; див. Рядки. |
| `previous_year_label` | `string signal` | ключ рядка | Calendar | Перекладний текст; див. Рядки. |
| `next_year_label` | `string signal` | ключ рядка | Calendar | Перекладний текст; див. Рядки. |
| `previous_years_label` | `string signal` | ключ рядка | Calendar | Перекладний текст; див. Рядки. |
| `next_years_label` | `string signal` | ключ рядка | Calendar | Перекладний текст; див. Рядки. |
| `previous_icon` | `Option<IconData>` | ключ піктограми | Calendar | Перевизначення вбудованої піктограми; див. Піктограми. |
| `next_icon` | `Option<IconData>` | ключ піктограми | Calendar | Перевизначення вбудованої піктограми; див. Піктограми. |
| `padding` | `number` | не встановлено | Calendar | Перевизначення метрики; див. нижче. |
| `cell_size` | `number` | не встановлено | Calendar | Перевизначення метрики; див. нижче. |
| `header_height` | `number` | не встановлено | Calendar | Перевизначення метрики; див. нижче. |
| `weekday_height` | `number` | не встановлено | Calendar | Перевизначення метрики; див. нижче. |
| `caption_font_size` | `number` | не встановлено | Calendar | Перевизначення метрики; див. нижче. |
| `line_height` | `number` | не встановлено | Calendar | Перевизначення метрики; див. нижче. |
| `caption_line_height` | `number` | не встановлено | Calendar | Перевизначення метрики; див. нижче. |
| `header` | `slot` | немає | Calendar | Перед опціями спливного вікна; Calendar: над навігацією місяців. |
| `footer` | `slot` | немає | Calendar | Після опцій спливного вікна; Calendar: під сіткою днів. |

## Доступність

* Поле введення має підпис; `aria-invalid` встановлено, поки показується пульсація помилки.
* Кнопки вибору мають підписи та `aria-expanded`; спливні шари мають роль `dialog`.
* Швидкі діапазони DateRangeField — підписаний `listbox`; підказка під календарями є ввічливою живою областю.
* Режими ColorPicker утворюють `tablist`; площина й смуги є `slider` з `aria-valuetext`; числові поля й поле коду мають назви каналів; зразки — підписаний `listbox`.

## Кастомізація

`class` та `style` застосовуються до кореня; явні оголошення `style` йдуть після метричних пропсів і мають перевагу.
Невстановлені метричні пропси зберігають значення токенів як запасні варіанти.
Усі текстоподібні поля введення надають ті самі шість слотів поля.

| Метричний проп | Одиниця | Змінна | Типове значення | Компоненти |
| :--- | :--- | :--- | :--- | :--- |
| `height` | DU | `--op-field-height` | `row_standard` | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField |
| `padding_x` | DU | `--op-field-padding-x` | `pad_card` | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField |
| `slot_size` | DU | `--op-field-slot-size` | `row_standard` | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField |
| `gap` | DU | `--op-field-gap` | `gap_micro` | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField |
| `font_size` | TU | `--op-field-font-size` | `type_body_size` | NumberField, DurationField, TimeField, DateTimeField, DateRangeField, ColorField, ParsedField, Calendar |
| `unit_padding_x` | DU | `--op-field-unit-padding-x` | `pad_card` | NumberField |
| `unit_gap` | DU | `--op-field-unit-gap` | `pad_tight` | NumberField |
| `icon_size` | DU | `--op-field-icon-size` | `icon_small` (`4 DU`); `icon_field` (`5 DU`) для стрілок вибору | DateTimeField, DateRangeField, ColorField, TimeField, Calendar |
| `popup_shadow_y` | DU | `--op-field-popup-shadow-y` | `2 DU` | DateTimeField, DateRangeField, ColorField, TimeField |
| `popup_shadow_blur` | DU | `--op-field-popup-shadow-blur` | `6 DU` | DateTimeField, DateRangeField, ColorField, TimeField |
| `calendar_padding` | DU | `--op-field-calendar-padding` | `pad_tight` | DateTimeField |
| `calendar_cell_size` | DU | `--op-field-calendar-cell-size` | `row_compact` | DateTimeField, DateRangeField |
| `calendar_header_height` | DU | `--op-field-calendar-header-height` | `row_compact` | DateTimeField |
| `calendar_weekday_height` | DU | `--op-field-calendar-weekday-height` | `row_dense` | DateTimeField |
| `preview_size` | DU | `--op-field-preview-size` | `icon_field` (`5 DU`) | ColorField |
| `picker_gap` | DU | `--op-field-picker-gap` | `pad_tight` | ColorField |
| `picker_padding_x` | DU | `--op-field-picker-padding-x` | `pad_tight` | ColorField |
| `popup_gap`, `gap` | DU | `--op-field-popup-gap` | `gutter` | ColorField, ColorPicker |
| `popup_padding`, `padding` | DU | `--op-field-popup-padding` | `pad_content` | ColorField, ColorPicker |
| `swatch_size` | DU | `--op-field-swatch-size` | `row_compact` | ColorField, ColorPicker |
| `picker_width`, `width` | DU | `--op-field-color-width` | `80 DU` | ColorField, ColorPicker |
| `plane_height` | DU | `--op-field-color-plane-height` | `40 DU` | ColorField, ColorPicker |
| `bar_height` | DU | `--op-field-color-bar-height` | `row_dense` | ColorField, ColorPicker |
| `thumb_size` | DU | `--op-field-color-thumb-size` | `icon_small` (`4 DU`) | ColorPicker |
| `tab_height` | DU | `--op-field-color-tab-height` | `row_compact` | ColorPicker |
| `number_width` | DU | `--op-field-color-number-width` | `14 DU` | ColorPicker |
| `font_size` | TU | `--op-field-font-size` | `type_body_size` | ColorPicker |
| `presets_width` | DU | `--op-field-range-presets-width` | `36 DU` | DateRangeField |
| `preset_height` | DU | `--op-field-range-preset-height` | `row_compact` | DateRangeField |
| `clock_size`, `size` | DU | `--op-field-clock-size` | `64 DU` | TimeField, ClockPicker |
| `number_size` | DU | `--op-field-clock-number-size` | `row_compact` | ClockPicker |
| `hand_width` | DU | `--op-field-clock-hand-width` | `border_thick` | ClockPicker |
| `readout_font_size` | TU | `--op-field-clock-readout-font-size` | `type_screen_title_size` | ClockPicker |
| `padding`, `gap`, `font_size` | DU, DU, TU | `--op-field-popup-padding`, `--op-field-popup-gap`, `--op-field-font-size` | `pad_content`, `gutter`, `type_body_size` | ClockPicker |
| `padding` | DU | `--op-field-calendar-padding` | `pad_tight` | Calendar |
| `cell_size` | DU | `--op-field-calendar-cell-size` | `row_compact` | Calendar |
| `header_height` | DU | `--op-field-calendar-header-height` | `row_compact` | Calendar |
| `weekday_height` | DU | `--op-field-calendar-weekday-height` | `row_dense` | Calendar |
| `caption_font_size` | TU | `--op-field-caption-font-size` | `type_caption_size` | Calendar |
| `line_height` | TU | `--op-field-line-height` | `type_body_line` | Calendar |
| `caption_line_height` | TU | `--op-field-caption-line-height` | `type_caption_line` | Calendar |

| Змінна кольору | Типове значення |
| :--- | :--- |
| `--op-field-background` | `surface` |
| `--op-field-text` | `text_primary` |
| `--op-field-secondary-text` | `text_secondary` |
| `--op-field-popup-border` | `accent` для рамки спливного шару, `border_strong` для попереднього перегляду кольору |
| `--op-field-pressed` | `pressed` |
| `--op-field-accent` | `accent` |
| `--op-field-on-accent` | `on_accent` |
| `--op-field-popup-shadow` | `existing black at 35% opacity` |
| `--op-field-swatch-border` | `existing black at 25% opacity` |
| `--op-field-surface-alt` | `surface_alt` для циферблата й показу часу ClockPicker |
| `--op-field-range` | `accent` на 22% для днів усередині діапазону |
| `--op-field-border` | `border` для числових полів і поля коду ColorPicker |
| `--op-field-hover` | `hover` для вкладок ColorPicker |
| `--op-field-divider` | `divider` під вкладками ColorPicker |
| `--op-field-checker` | `border` для шахового тла під напівпрозорими кольорами |
| `--op-field-color-thumb` | `on_media` для маркерів площини та смуг |
| `--op-field-color-thumb-outline` | `shadow` навколо маркерів площини та смуг |

| Слот | Розташування |
| :--- | :--- |
| `prepend` | Усередині поля перед елементом керування. |
| `append` | Усередині поля перед вбудованими елементами розкриття, збереження або вибору. |
| `before` | Зовні поля перед ним. |
| `after` | Зовні поля після нього. |
| `label_slot` | Замінює видиму мітку. |
| `hint_slot` | Замінює рядок підказки та помилки. |
| `header` | Перед опціями спливного вікна; Calendar: над навігацією місяців. |
| `footer` | Після опцій спливного вікна; Calendar: під сіткою днів; ColorPicker: під зразками. |

ParsedField зберігає наявні `trailing: Option<ChildrenFn>`; `append` відображається перед цими представленнями.
Спливні шари вибору отримують перевизначення кореневих метрик і успадковані змінні `--op-field-*`.
Calendar визначає назви місяців і днів тижня незалежно, використовуючи запасні значення для відсутніх елементів масивів.
DurationField приймає перекладені суфікси поряд із канонічними англійськими формами.

### Рядки

| Ключ | Проп | Типове значення |
| :--- | :--- | :--- |
| `number-field.placeholder` | `placeholder` | `(empty)` |
| `number-field.label` | `aria_label` | `Number` |
| `duration-field.placeholder` | `placeholder` | `(empty)` |
| `duration-field.label` | `aria_label` | `Duration` |
| `duration-field.day-unit` | `day_unit` | `d` |
| `duration-field.hour-unit` | `hour_unit` | `h` |
| `duration-field.minute-unit` | `minute_unit` | `m` |
| `duration-field.second-unit` | `second_unit` | `s` |
| `duration-field.millisecond-unit` | `millisecond_unit` | `ms` |
| `time-field.placeholder` | `placeholder` | `(empty)` |
| `time-field.label` | `aria_label` | `Time` |
| `time-field.pick` | `picker_label` | `Pick a time` |
| `time-field.popup` | `popup_label` | `Clock` |
| `clock-picker.hours` | `hours_label` | `Hours` |
| `clock-picker.minutes` | `minutes_label` | `Minutes` |
| `clock-picker.am` | `am_label` | `AM` |
| `clock-picker.pm` | `pm_label` | `PM` |
| `date-range-field.placeholder` | `placeholder` | `(empty)` |
| `date-range-field.label` | `aria_label` | `Date range` |
| `date-range-field.pick` | `picker_label` | `Pick dates` |
| `date-range-field.popup` | `popup_label` | `Date range` |
| `date-range-field.presets` | `presets_label` | `Quick ranges` |
| `date-range-field.start-hint` | `start_hint` | `Pick the first day` |
| `date-range-field.end-hint` | `end_hint` | `Pick the last day` |
| `date-range-field.months` | `short_month_names` | `Jan,Feb,Mar,Apr,May,Jun,Jul,Aug,Sep,Oct,Nov,Dec` |
| `date-range-field.today` | `presets` | `Today` |
| `date-range-field.yesterday` | `presets` | `Yesterday` |
| `date-range-field.this-week` | `presets` | `This week` |
| `date-range-field.last-week` | `presets` | `Last week` |
| `date-range-field.this-month` | `presets` | `This month` |
| `date-range-field.last-month` | `presets` | `Last month` |
| `date-range-field.last-days` | `presets` | `Last {n} days` |
| `date-range-field.this-year` | `presets` | `This year` |
| `date-range-field.last-year` | `presets` | `Last year` |
| `date-time-field.placeholder` | `placeholder` | `(empty)` |
| `date-time-field.label` | `aria_label` | `Date and time` |
| `date-time-field.pick` | `picker_label` | `Pick a date` |
| `date-time-field.popup` | `popup_label` | `Calendar` |
| `color-field.placeholder` | `placeholder` | `(empty)` |
| `color-field.label` | `aria_label` | `Colour` |
| `color-field.pick` | `picker_label` | `Pick a colour` |
| `color-field.popup` | `popup_label` | `Colours` |
| `color-picker.spectrum` | `spectrum_label` | `Spectrum` |
| `color-picker.rgb` | `rgb_label` | `RGB` |
| `color-picker.hsl` | `hsl_label` | `HSL` |
| `color-picker.temperature` | `temperature_label` | `Temperature` |
| `color-picker.plane` | `plane_label` | `Saturation and brightness` |
| `color-picker.hue` | `hue_label` | `Hue` |
| `color-picker.saturation` | `saturation_label` | `Saturation` |
| `color-picker.brightness` | `brightness_label` | `Brightness` |
| `color-picker.lightness` | `lightness_label` | `Lightness` |
| `color-picker.red` | `red_label` | `Red` |
| `color-picker.green` | `green_label` | `Green` |
| `color-picker.blue` | `blue_label` | `Blue` |
| `color-picker.alpha` | `alpha_label` | `Opacity` |
| `color-picker.hex` | `hex_label` | `Colour code` |
| `color-picker.swatches` | `swatches_label` | `Presets` |
| `parsed-field.placeholder` | `placeholder` | `(empty)` |
| `parsed-field.label` | `aria_label` | `Value` |
| `calendar.previous` | `previous_label` | `Previous month` |
| `calendar.next` | `next_label` | `Next month` |
| `calendar.jump` | `jump_label` | `Choose month and year` |
| `calendar.previous-year` | `previous_year_label` | `Previous year` |
| `calendar.next-year` | `next_year_label` | `Next year` |
| `calendar.previous-years` | `previous_years_label` | `Previous years` |
| `calendar.next-years` | `next_years_label` | `Next years` |
| `calendar.january` | `month_names` | `January` |
| `calendar.february` | `month_names` | `February` |
| `calendar.march` | `month_names` | `March` |
| `calendar.april` | `month_names` | `April` |
| `calendar.may` | `month_names` | `May` |
| `calendar.june` | `month_names` | `June` |
| `calendar.july` | `month_names` | `July` |
| `calendar.august` | `month_names` | `August` |
| `calendar.september` | `month_names` | `September` |
| `calendar.october` | `month_names` | `October` |
| `calendar.november` | `month_names` | `November` |
| `calendar.december` | `month_names` | `December` |
| `calendar.monday` | `weekday_names` | `Mo` |
| `calendar.tuesday` | `weekday_names` | `Tu` |
| `calendar.wednesday` | `weekday_names` | `We` |
| `calendar.thursday` | `weekday_names` | `Th` |
| `calendar.friday` | `weekday_names` | `Fr` |
| `calendar.saturday` | `weekday_names` | `Sa` |
| `calendar.sunday` | `weekday_names` | `Su` |

### Піктограми

| Ключ | Проп | Типове значення |
| :--- | :--- | :--- |
| `time-field.expand` | `icon` | `arrow_drop_down` |
| `date-range-field.expand` | `icon` | `arrow_drop_down` |
| `date-time-field.expand` | `icon` | `arrow_drop_down` |
| `color-field.expand` | `icon` | `arrow_drop_down` |
| `calendar.previous` | `previous_icon` | `chevron_left` |
| `calendar.next` | `next_icon` | `chevron_right` |
