# SidePanel

Іменована панель біля краю екрана, що згортається до вузької смуги, як-от палітра, структура або панель властивостей поруч із полотном.

Статус: **чернетка** у вебреалізації.

## Геометрія

| Властивість | Значення |
| :--- | :--- |
| Ширина | Типово `75 DU`; у згорнутому стані `row_standard` (`10 DU`). |
| Заголовок | Висота `row_standard`: заголовок у стилі `section_title`, необов'язкові дії та [IconButton](icon-button.md) розміром `row_compact`, що згортає панель; знизу `0.25 DU` у кольорі `divider`. |
| Смуга | Лише кнопка розгортання, угорі. |
| Край | `0.25 DU` у кольорі `divider` на краї, зверненому до вмісту. |
| Тіло | Заповнює решту та прокручується. |

## Поведінка

* Кнопка згортання вказує на край панелі (`chevron_left` для початкової панелі); кнопка розгортання вказує від нього.
* Зміни ширини анімуються рухом `expand`.
* Згорнутий стан є сигналом, тому застосунок може його запам'ятовувати.

## API

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `title` | сигнал рядка | обов’язковий | Текст заголовка й доступна назва. |
| `collapsed` | сигнал bool | обов’язковий | Згорнутий стан. |
| `side` | `PanelSide` | `Start` | `Start` або `End`. |
| `width` | DU | `75 DU` | Ширина в розгорнутому стані. |
| `collapsed_width` | DU | `row_standard` | Ширина в згорнутому стані. |
| `header_height` | DU | `row_standard` | Висота заголовка. |
| `header_padding_start` | DU | `pad_card` | Початковий внутрішній відступ заголовка. |
| `header_padding_end` | DU | `gap_micro` | Кінцевий внутрішній відступ заголовка. |
| `header_gap` | DU | `pad_tight` | Проміжок між елементами заголовка. |
| `strip_padding_top` | DU | `gap_micro` | Верхній внутрішній відступ смуги. |
| `title_font_size` | TU | `type_section_title_size` | Розмір шрифту заголовка. |
| `header` | слот | немає | Власний заголовок замість типового. |
| `actions` | слот | немає | Дії заголовка перед кнопкою згортання. |
| `footer` | слот | немає | Слот під тілом панелі. |
| `collapse_label` | string | `side-panel.collapse` | Доступна назва кнопки згортання. |
| `expand_label` | string | `side-panel.expand` | Доступна назва кнопки розгортання. |
| `collapse_icon` | піктограма | `side-panel.collapse` | Перевизначення піктограми кнопки згортання. |
| `expand_icon` | піктограма | `side-panel.expand` | Перевизначення піктограми кнопки розгортання. |
| `class` | string | немає | Класи кореневого елемента. |
| `style` | string | немає | Інлайн-стилі кореневого елемента. |
| `children` | view | обов’язковий | Вміст панелі. |

## Кастомізація

| Метричний проп | Одиниця | Змінна | Типове значення |
| :--- | :--- | :--- | :--- |
| `width` | DU | `--op-side-panel-width` | `75 DU` |
| `collapsed_width` | DU | `--op-side-panel-collapsed-width` | `row_standard` (`10 DU`) |
| `header_height` | DU | `--op-side-panel-header-height` | `row_standard` (`10 DU`) |
| `header_padding_start` | DU | `--op-side-panel-header-padding-start` | `pad_card` (`2 DU`) |
| `header_padding_end` | DU | `--op-side-panel-header-padding-end` | `gap_micro` (`0.5 DU`) |
| `header_gap` | DU | `--op-side-panel-header-gap` | `pad_tight` (`1 DU`) |
| `strip_padding_top` | DU | `--op-side-panel-strip-padding-top` | `gap_micro` (`0.5 DU`) |
| `title_font_size` | TU | `--op-side-panel-title-font-size` | `type_section_title_size` |

| Змінна кольору | Типове значення |
| :--- | :--- |
| `--op-side-panel-background` | `surface` |
| `--op-side-panel-divider` | `divider` |
| `--op-side-panel-title-color` | `text_primary` |

| Слот | Розташування |
| :--- | :--- |
| `header` | Замінює типову панель заголовка. |
| `actions` | Усередині панелі заголовка, перед кнопкою згортання. |
| `footer` | Під тілом панелі. |

`class` та `style` застосовуються до кореневого елемента.

### Рядки

| Ключ | Типове значення |
| :--- | :--- |
| `side-panel.collapse` | `Collapse` |
| `side-panel.expand` | `Expand` |

### Піктограми

| Ключ | Типове значення |
| :--- | :--- |
| `side-panel.collapse` | `chevron_left` для `Start`, `chevron_right` для `End` |
| `side-panel.expand` | `chevron_right` для `Start`, `chevron_left` для `End` |
