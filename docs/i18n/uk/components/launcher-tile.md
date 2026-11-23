# LauncherTile

Квадратна плитка запуску для інформаційних панелей, панелей інструментів та сіток застосунків.

Статус: **reviewed** у вебреалізації.

## Призначення

Використовуйте `LauncherTile` під час організації набору ярликів запуску програм, модулів або інструментів у сітці дашборду.
Не використовуйте `LauncherTile` для звичайних тригерів форм або панелей інструментів; використовуйте [ActionButton](action-button.md) або [IconButton](icon-button.md).

## Геометрія

| Властивість | Значення |
| :--- | :--- |
| Радіус кутів | `0 DU` (стандартна пряма геометрія). |
| Розміри за замовчуванням | `24 DU` на `24 DU`. |
| Межа | Стандартна межа `0.25 DU` всередині меж елемента. |
| Внутрішні відступи | `2 DU`. |
| Проміжок | `2.5 DU` між піктограмою та міткою. |

## Поведінка

* Клік на плитку викликає `on_click` або запускає `on_async_click`.
* Під час виконання асинхронної дії плитка дотримується [Станів](../foundations/states.md): рамка стану показує біжучий сегмент, а після затримки [відкладений спінер](../foundations/states.md#delayed-spinner) замінює піктограму в її розмірі; підпис лишається.
* Результат спалахує на рамці й замінює піктограму піктограмою успіху чи помилки в її кольорі, після чого плитка повертається до спокою.
* `loading` показує плитку в очікуванні ззовні.
* Підтримує клавіатурну навігацію клавішами `Enter` та `Space`.
* Якщо передано `href`, рендериться як посилання-якір, стилізоване ідентично до кнопки.

## API

| Властивість | Тип | За замовчуванням | Опис |
| :--- | :--- | :--- | :--- |
| `label` | string | `"Launch"` | Текст підпису під піктограмою. |
| `icon` | Icon | немає | Основна піктограма. |
| `icon_slot` | Slot | немає | Користувацький слот піктограми, що перевизначає `icon`. |
| `badge_slot` | Slot | немає | Слот для бейджа у верхньому правому куті (лічильник чи статус). |
| `content_slot` | Slot | немає | Користувацький контент замість стандартної піктограми та мітки. |
| `state` | ControlState | немає | Явне перевизначення стану елемента керування. |
| `enabled` | bool | `true` | Прапорець доступності; вимкнена плитка ігнорує введення. |
| `loading` | bool | `false` | Показує плитку в очікуванні ззовні. |
| `loading_label`, `success_label`, `error_label` | string | `launcher-tile.loading`, `.success`, `.error` | Доступні назви спінера й піктограм результату. |
| `success_icon`, `error_icon` | піктограма | `launcher-tile.success`, `.error` | Піктограми результату. |
| `selected` | bool | `false` | Прапорець вибору для виділення плитки в сітці. |
| `on_click` | callback `()` | немає | Синхронний зворотний виклик кліку. |
| `on_async_click` | AsyncAction | немає | Асинхронна дія кліку. |
| `href` | string | немає | URL цільової навігації. |
| `new_tab` | bool | `false` | Відкриває посилання у новій вкладці браузера. |
| `size` | DU | `24.0` | Зовнішній розмір у DU (`--op-launcher-tile-size`). |
| `icon_size` | DU | `8.0` | Розмір піктограми в DU (`--op-launcher-tile-icon-size`). |
| `gap` | DU | `2.5` | Проміжок між піктограмою та підписом у DU (`--op-launcher-tile-gap`). |
| `padding` | DU | `2.0` | Внутрішній відступ у DU (`--op-launcher-tile-padding`). |
| `border_width` | DU | `0.25` | Товщина межі в DU (`--op-launcher-tile-border-width`). |
| `font_size` | TU | `11.0` | Розмір шрифту підпису в TU (`--op-launcher-tile-font-size`). |
| `line_height` | TU | `14.0` | Висота рядка підпису в TU (`--op-launcher-tile-line-height`). |
| `class` | string | немає | CSS-класи кореневого елемента. |
| `style` | string | немає | Вбудовані стилі кореневого елемента. |

## Кастомізація

| Проп метрики | Одиниця | Змінна | За замовчуванням |
| :--- | :--- | :--- | :--- |
| `size` | DU | `--op-launcher-tile-size` | `24 DU` |
| `icon_size` | DU | `--op-launcher-tile-icon-size` | `icon_xlarge` (`8 DU`) |
| `gap` | DU | `--op-launcher-tile-gap` | `2.5 DU` |
| `padding` | DU | `--op-launcher-tile-padding` | `gutter` (`2 DU`) |
| `border_width` | DU | `--op-launcher-tile-border-width` | `0.25 DU` |
| `font_size` | TU | `--op-launcher-tile-font-size` | `11 TU` |
| `line_height` | TU | `--op-launcher-tile-line-height` | `14 TU` |

| Змінна кольору | За замовчуванням |
| :--- | :--- |
| `--op-launcher-tile-background` | `surface` |
| `--op-launcher-tile-text` | `text_primary` |
| `--op-launcher-tile-border` | `border` |
| `--op-launcher-tile-hover` | `hover` |
| `--op-launcher-tile-pressed` | `pressed` |
| `--op-launcher-tile-selection` | `selection` |
| `--op-launcher-tile-disabled-background` | `disabled_surface` |
| `--op-launcher-tile-disabled-text` | `text_disabled` |
| `--op-launcher-tile-icon` | `icon` |
| `--op-launcher-tile-disabled-icon` | `icon_disabled` |
| `--op-launcher-tile-success` | `success` |
| `--op-launcher-tile-error` | `error` |
| `--op-launcher-tile-pending` | `pending` |
| `--op-launcher-tile-accent` | `accent` |

`class` та `style` застосовуються до кореневого елемента `<button>` або `<a>`.

### Рядки

| Ключ | За замовчуванням |
| :--- | :--- |
| `launcher-tile.label` | `"Launch"` |
| `launcher-tile.loading` | `"Loading"` |
| `launcher-tile.success` | `"Success"` |
| `launcher-tile.error` | `"Error"` |

### Піктограми

| Ключ | За замовчуванням |
| :--- | :--- |
| `launcher-tile.success` | `check` |
| `launcher-tile.error` | `close` |

## Доступність

Плитка відображається як нативний елемент `<button>` або `<a>` із відповідними ARIA-атрибутами.
Поки виконується дія, встановлюється `aria-busy`; коли плитку не можна натиснути, — `aria-disabled="true"`.
Повністю підтримується клавіатурна активація через `Enter` або `Space`.
