# ActionButton

Прямокутна кнопка, яка запускає асинхронну дію та показує її результат.

Статус: **reviewed** у вебреалізації.

## Анатомія

* **Контейнер:** володіє контуром стану та тлом результату.
* **Підпис:** необов’язкова піктограма, після якої йде текст.
* **Вміст статусу:** спінер або піктограма результату, що малюються на місці підпису.
* **Примарний підпис:** невидима копія підпису, яка зберігає ширину стабільною під час показу вмісту статусу.

## Геометрія

| Властивість | Значення |
| :--- | :--- |
| Висота | `row_standard` (`10 DU`). |
| Ширина | Вміст плюс `margin_screen` (`4 DU`) з кожного боку. |
| Радіус кутів | `0 DU`. |
| Піктограма | `icon_small` (`4 DU`), проміжок `gutter` (`2 DU`) до тексту. |
| Текст | Розмір `body`, насиченість 500, центрований у кожному рядку. |

## Стани

| Стан | Тло | Вміст | Контур |
| :--- | :--- | :--- | :--- |
| `Idle` | `surface` | `text_primary` | `border` |
| `Pressed` | `pressed` | `text_primary` | `border_strong` |
| `Pending` | `surface` | Підпис, потім [спінер із затримкою](../foundations/states.md#delayed-spinner) | Рухомий сегмент, якщо не вимкнено через `show_pending_border` |
| `Success` | `success` із 15% | `success` | Пульсівний `success` |
| `Error` | `error` із 15% | `error` | Пульсівний `error` |
| `Disabled` | `disabled_surface` | `text_disabled` | `disabled_border` |

**Небезпечна** кнопка використовує вміст `error` на `error` із 15% у стані спокою та `error` із 30% при натисканні.

## Поведінка

1. Клік запускає дію та переводить у `Pending`; під час очікування кліки ігноруються.
2. Після завершення кнопка переходить у стан результату на `800 ms`.
   Протягом перших `500 ms` показується лише піктограма результату.
   Протягом решти `300 ms` підпис повертається в кольорі результату.
3. Після цього результат згасає протягом `500 ms`.

Із `keep_flash_icon_on_success` успішне завершення зберігає піктограму галочки протягом усіх `800 ms`.
Небезпечна кнопка ігнорує кліки під час своєї фази результату.

## Режим посилання

Встановлення `href` перетворює кнопку на навігаційне посилання з таким самим виглядом.
Використовуйте його, коли натискання відкриває сторінку або зовнішній ресурс, а не запускає дію.

* Застосовуються лише `Idle`, `Pressed` та `Disabled`.
  `state`, `on_click`, `show_pending_border` та `keep_flash_icon_on_success` ігноруються.
* `new_tab` відкриває ціль у новій вкладці чи вікні, якщо платформа підтримує цю концепцію.
* Вимкнене посилання зберігає свій вигляд, але не здійснює навігацію.

### Примітки щодо платформ

| Платформа | Реалізація |
| :--- | :--- |
| Web | Справжній елемент `<a href>`, тому пошукові роботи переходять за ним, а клік коліщатком, кліки з модифікаторами та контекстне меню посилання працюють. `new_tab` додає `target="_blank"` і `rel="noopener noreferrer"`. Вимкнене посилання скидає `href` і встановлює `aria-disabled`. |
| Native toolkits | Звичайна кнопка, натискання якої відкриває ціль через системний обробник URL платформи, наприклад `open_url` в egui. |

## API

| Властивість | Тип | Типове значення |
| :--- | :--- | :--- |
| `text` | string | `action-button.text` |
| `icon` | icon | немає |
| `state` | `ControlState?` | немає |
| `enabled` | bool | `true` |
| `show_pending_border` | bool | `true` |
| `keep_flash_icon_on_success` | bool | `false` |
| `dangerous` | bool | `false` |
| `on_click` | async `() -> ActionResult` | немає |
| `href` | string | немає |
| `new_tab` | bool | `false` |
| `height` | reactive DU | не встановлено |
| `padding_x` | reactive DU | не встановлено |
| `gap` | reactive DU | не встановлено |
| `icon_size` | reactive DU | не встановлено |
| `font_size` | reactive TU | не встановлено |
| `line_height` | reactive TU | не встановлено |
| `spinner_stroke` | reactive DU | не встановлено |
| `class` | reactive string | немає |
| `style` | reactive string | немає |
| `leading` | slot | немає |
| `trailing` | slot | немає |
| `content` | slot | немає |
| `loading_label` | reactive string | `action-button.loading` |
| `success_label` | reactive string | `action-button.success` |
| `error_label` | reactive string | `action-button.error` |
| `success_icon` | icon | `action-button.success` |
| `error_icon` | icon | `action-button.error` |

## Кастомізація

`class` та `style` застосовуються до кореневого елемента.
Метричні пропси приймають реактивні значення DU або TU; невстановлені пропси зберігають CSS-змінну або її точний запасний варіант із дизайн-системи.
`style` викликача додається після оголошень метрик і має пріоритет.
ActionButton та IconButton спільно використовують змінні `--op-button-*` для метрик і кольорів з однаковим значенням.

| Метричний проп | Одиниця | Змінна | Типове значення |
| :--- | :--- | :--- | :--- |
| `height` | DU | `--op-button-height` | `var(--op-dim-row-standard)` |
| `padding_x` | DU | `--op-button-padding-x` | `var(--op-dim-margin-screen)` |
| `gap` | DU | `--op-button-gap` | `var(--op-dim-gutter)` |
| `icon_size` | DU | `--op-button-icon-size` | `var(--op-dim-icon-small)` |
| `font_size` | TU | `--op-button-font-size` | `var(--op-type-body-size)` |
| `line_height` | TU | `--op-button-line-height` | `var(--op-type-body-line)` |
| `spinner_stroke` | DU | `--op-button-spinner-stroke` | `var(--op-dim-spinner-stroke)` |
| Змінна кольору | Типове значення |
| :--- | :--- |
| `--op-button-accent` | `var(--op-accent)` |
| `--op-button-border` | `var(--op-border)` |
| `--op-button-emphasis-border` | `var(--op-border-strong)` |
| `--op-button-disabled-border` | `var(--op-disabled-border)` |
| `--op-button-disabled-background` | `var(--op-disabled-surface)` |
| `--op-button-error` | `var(--op-error)` |
| `--op-button-hover` | `var(--op-hover)` |
| `--op-button-icon` | `var(--op-icon)` |
| `--op-button-disabled-icon` | `var(--op-icon-disabled)` |
| `--op-button-pending` | `var(--op-pending)` |
| `--op-button-pressed` | `var(--op-pressed)` |
| `--op-button-ripple` | `var(--op-ripple)` |
| `--op-button-selection` | `var(--op-selection)` |
| `--op-button-success` | `var(--op-success)` |
| `--op-button-background` | `var(--op-surface)` |
| `--op-button-disabled-text` | `var(--op-text-disabled)` |
| `--op-button-text` | `var(--op-text-primary)` |
| Слот | Розташування |
| :--- | :--- |
| `leading` | Перед типовим вмістом. |
| `trailing` | Після типового вмісту. |
| `content` | Замінює типовий текст або піктограму; зворотний зв’язок очікування та результату зберігає пріоритет. |

### Рядки

| Проп | Ключ | Типове значення |
| :--- | :--- | :--- |
| `loading_label` | `action-button.loading` | `Loading` |
| `success_label` | `action-button.success` | `Success` |
| `error_label` | `action-button.error` | `Error` |
| `text` | `action-button.text` | порожній рядок |

### Піктограми

| Проп | Ключ | Типове значення |
| :--- | :--- | :--- |
| `success_icon` | `action-button.success` | `check` |
| `error_icon` | `action-button.error` | `close` |

Піктограми, передані як дані елемента або через наявний проп `icon`, залишаються у власності викликача.