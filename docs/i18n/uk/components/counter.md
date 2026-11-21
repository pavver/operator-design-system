# Counter

Компактний числовий бейдж для непрочитаних повідомлень, тривог або підсумків.

Статус: **reviewed** у вебреалізації.

## Геометрія

| Властивість | Значення |
| :--- | :--- |
| Розмір | `4.5 DU` заввишки, щонайменше `4.5 DU` завширшки. |
| Внутрішній відступ | `1 DU` по горизонталі. |
| Типографіка | розмір `caption`, напівжирний, табличні цифри. |
| Кути | `0 DU`. |
| На елементі керування | Верхній кінцевий кут, зміщення на `1 DU` назовні. |

## Тони

| Тон | Вигляд | Використання |
| :--- | :--- | :--- |
| Accent | фон `accent`, текст `on_accent`. | Нові або непрочитані елементи. |
| Neutral | фон `surface_alt`, контур `border`, текст `text_secondary`. | Звичайні підсумки. |
| Error | фон `error`, текст `on_accent`. | Збої та тривоги. |

## Поведінка

* Значення, більші за `max` (типово 99), відображаються як `99+`.
* При нулі лічильник приховано, якщо не встановлено `show_zero`.

## API

| Компонент | Властивості |
| :--- | :--- |
| `Counter` | `count`, `max` (`99`), `tone` (Accent), `show_zero` (`false`), `aria_label` (необов'язковий). |
| `WithCounter` | `count`, `max`, `tone`, `show_zero`, `aria_label`, `offset`, `children`: елемент керування, на якому розміщено лічильник. |

## Кастомізація

| Метричний проп | Одиниця | Змінна | Типове значення |
| :--- | :--- | :--- | :--- |
| `min_width` | DU | `--op-counter-min-width` | `icon_medium` (`4.5 DU`) |
| `height` | DU | `--op-counter-height` | `icon_medium` (`4.5 DU`) |
| `padding_x` | DU | `--op-counter-padding-x` | `pad_tight` (`1 DU`) |
| `font_size` | TU | `--op-counter-font-size` | `type_caption_size` |
| `offset` | DU | `--op-counter-offset` | `pad_tight` (`1 DU`); у WithCounter |

| Змінна кольору | Типове значення |
| :--- | :--- |
| `--op-counter-background` | `accent` |
| `--op-counter-text` | `on_accent` |
| `--op-counter-neutral-background` | `surface_alt` |
| `--op-counter-neutral-text` | `text_secondary` |
| `--op-counter-neutral-border` | `border` |
| `--op-counter-error-background` | `error` |
| `--op-counter-error-text` | `on_accent` |

`class` і `style` застосовуються до кореня.

### Рядки

| Ключ | Типове значення |
| :--- | :--- |
| `counter.items` | `items` |

## Доступність

Лічильник має доступну назву "`n` items" із точним числом, навіть коли він показує `99+`.
