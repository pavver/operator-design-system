# NavigationRail

Вертикальна панель напрямків верхнього рівня біля початкового краю: піктограма над коротким підписом.

Статус: **чернетка** у вебреалізації.

## Використання

* Перемикання між кількома (від трьох до восьми) розділами екрана, як-от категоріями нод або областями застосунку.
* Для більшої кількості напрямків або довших назв використовуйте список чи [Tabs](tabs.md).

## Геометрія

| Властивість | Значення |
| :--- | :--- |
| Ширина | `16 DU`; панель прокручується вертикально без видимої смуги прокручування, коли елементи не вміщуються. |
| Елемент | Висота `row_two_line` (`13 DU`): плитка `7 DU` з піктограмою `4.5 DU` над підписом `caption`, із проміжком `gap_micro`. |
| Підпис | Один рядок, трикрапка для довшого тексту. |
| Вибрана | Фон `pressed`, підпис `text_primary` і смуга `0.5 DU` на початковому краї в кольорі елемента або `accent`. |
| Край | `0.25 DU` у кольорі `divider` на кінцевому краї. |
| Горизонтальна | Рядок верхніх вкладок на всю ширину під панеллю застосунку для вузьких компонувань; елементи ділять ширину (щонайменше `16 DU`, рядок прокручується, якщо вони не вміщуються), смуга вибору та розділювач переходять на нижній край. |

## Поведінка

* Клік вибирає напрямок.
* Arrow Up і Down (Left і Right у горизонтальному режимі) циклічно переміщують вибір; лише вибраний елемент входить у порядок переходу Tab.
* Елемент може мати власний колір (колір категорії); він забарвлює піктограму та смугу вибору.

## API

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `items` | список або сигнал `RailItem` | обов’язковий | `RailItem::new(id, label, icon)`, необов'язковий `color`. |
| `selected` | сигнал рядка | обов’язковий | Ідентифікатор вибраного напрямку. |
| `on_select` | зворотний виклик `(id)` | немає | Викликається при кожному виборі. |
| `label` | string | `navigation-rail.label` | Доступна назва. |
| `horizontal` | сигнал bool | `false` | Компонування верхніх вкладок для вузьких екранів. |
| `width` | DU | `16 DU` | Ширина панелі. |
| `item_height` | DU | `row_two_line` | Висота елемента напрямку. |
| `item_padding_x` | DU | `gap_micro` | Горизонтальний внутрішній відступ елемента напрямку. |
| `gap` | DU | `gap_micro` | Проміжок між плиткою піктограми та підписом. |
| `icon_tile_size` | DU | `7 DU` | Розмір квадратної плитки піктограми. |
| `icon_size` | DU | `icon_medium` | Розмір гліфа піктограми. |
| `indicator_thickness` | DU | `border_emphasis` | Товщина індикатора вибору. |
| `font_size` | TU | `type_caption_size` | Розмір шрифту підпису напрямку. |
| `header` | слот | немає | Слот над списком напрямків. |
| `footer` | слот | немає | Слот під списком напрямків. |
| `option` | слот з областю видимості `(RailItem)` | немає | Власний рендеринг кожного елемента напрямку. |
| `class` | string | немає | Класи кореневого елемента. |
| `style` | string | немає | Інлайн-стилі кореневого елемента. |

## Кастомізація

| Метричний проп | Одиниця | Змінна | Типове значення |
| :--- | :--- | :--- | :--- |
| `width` | DU | `--op-navigation-rail-width` | `16 DU` |
| `item_height` | DU | `--op-navigation-rail-item-height` | `row_two_line` (`13 DU`) |
| `item_padding_x` | DU | `--op-navigation-rail-item-padding-x` | `gap_micro` (`0.5 DU`) |
| `gap` | DU | `--op-navigation-rail-gap` | `gap_micro` (`0.5 DU`) |
| `icon_tile_size` | DU | `--op-navigation-rail-icon-tile-size` | `7 DU` |
| `icon_size` | DU | `--op-navigation-rail-icon-size` | `icon_medium` (`4.5 DU`) |
| `indicator_thickness` | DU | `--op-navigation-rail-indicator-thickness` | `border_emphasis` (`0.5 DU`) |
| `font_size` | TU | `--op-navigation-rail-font-size` | `type_caption_size` |

| Змінна кольору | Типове значення |
| :--- | :--- |
| `--op-navigation-rail-background` | `surface` |
| `--op-navigation-rail-divider` | `divider` |
| `--op-navigation-rail-text` | `text_secondary` |
| `--op-navigation-rail-hover-background` | `pressed` |
| `--op-navigation-rail-hover-text` | `text_primary` |
| `--op-navigation-rail-selected-text` | `text_primary` |
| `--op-navigation-rail-selected-background` | `pressed` |
| `--op-navigation-rail-indicator-color` | `accent` |
| `--op-navigation-rail-selected-icon` | `accent` |

| Слот | Розташування |
| :--- | :--- |
| `header` | Над списком напрямків. |
| `footer` | Під списком напрямків. |
| `option` | Власний вміст усередині кнопки елемента напрямку. |

`class` та `style` застосовуються до кореневого елемента.

### Рядки

| Ключ | Типове значення |
| :--- | :--- |
| `navigation-rail.label` | `Navigation` |

### Піктограми

Компонент не має вбудованих піктограм; напрямки передають власні дані піктограм.

## Доступність

* Панель є `nav` із вертикальним `tablist`; елементи мають роль `tab` із `aria-selected`.
