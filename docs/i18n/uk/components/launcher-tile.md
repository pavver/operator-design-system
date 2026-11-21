# LauncherTile

Квадратна плитка запуску для інформаційних панелей, панелей інструментів та сіток застосунків.

Статус: **draft** на web.

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
* Під час виконання асинхронної дії плитка переходить у стан `Pending` та відображає візуальну рамку стану.
* Після завершення плитка відображає пульсуючу рамку `Success` або `Error`.
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
| `loading` | bool | `false` | Прапорець завантаження для ручного спінера очікування. |
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
| `icon_size` | DU | `--op-launcher-tile-icon-size` | `8 DU` |
| `gap` | DU | `--op-launcher-tile-gap` | `2.5 DU` |
| `padding` | DU | `--op-launcher-tile-padding` | `2 DU` |
| `border_width` | DU | `--op-launcher-tile-border-width` | `0.25 DU` |
| `font_size` | TU | `--op-launcher-tile-font-size` | `11 TU` |
| `line_height` | TU | `--op-launcher-tile-line-height` | `14 TU` |

| Змінна кольору | За замовчуванням |
| :--- | :--- |
| `--op-launcher-tile-bg` | `var(--op-surface)` |
| `--op-launcher-tile-color` | `var(--op-text-primary)` |
| `--op-launcher-tile-border` | `var(--op-border)` |
| `--op-launcher-tile-hover-bg` | `var(--op-surface-hover)` |
| `--op-launcher-tile-hover-border` | `var(--op-border-strong)` |
| `--op-launcher-tile-pressed-bg` | `var(--op-surface-active)` |
| `--op-launcher-tile-selected-bg` | `var(--op-surface-selected)` |
| `--op-launcher-tile-disabled-bg` | `var(--op-disabled-surface)` |
| `--op-launcher-tile-disabled-border` | `var(--op-disabled-border)` |
| `--op-launcher-tile-disabled-color` | `var(--op-text-disabled)` |
| `--op-launcher-tile-icon-color` | `var(--op-icon)` |
| `--op-launcher-tile-disabled-icon-color` | `var(--op-text-disabled)` |

`class` та `style` застосовуються до кореневого елемента `<button>` або `<a>`.

### Рядки

| Ключ | За замовчуванням |
| :--- | :--- |
| `launcher-tile.label` | `"Launch"` |

### Піктограми

| Ключ | За замовчуванням |
| :--- | :--- |
| Індикатор очікування | `icons::REFRESH` |

## Доступність

Плитка відображається як нативний елемент `<button>` або `<a>` із відповідними ARIA-атрибутами.
У вимкненому стані встановлюються атрибути `disabled` та `aria-disabled="true"`.
Повністю підтримується клавіатурна активація через `Enter` або `Space`.
