# StatusBadge

Малий круглий індикатор присутності або активності, що використовується окремо або на [Avatar](avatar.md).

Статус: **reviewed** у вебреалізації.

## Геометрія

* Коло діаметром `3 DU` за замовчуванням.
* Контур `0.25 DU` у кольорі `status_border` відокремлює бейдж від будь-якого фону.
* Кожна анімація залишається в межах бейджа; бейдж ніколи не змінює компонування.

## Варіанти

Усі кольори походять із токенів `status_*`.

| Статус | Вигляд |
| :--- | :--- |
| `Online` | Суцільний `status_online`. |
| `Offline` | Суцільний `status_offline`. |
| `Alarm` | `status_alarm`, що блимає з півперіодом `250 ms`. |
| `Error` | Суцільний `status_error`. |
| `Pending` | Спінер у кольорі `status_activity` зі штрихом `0.3 DU`. |
| `Typing` | Три крапки `status_activity`, що підстрибують хвилею з періодом `800 ms`. |
| `RecordingAudio` | Крапка джерела з трьома звуковими дугами, що пульсують зліва направо протягом `900 ms`. |
| `SendingFile` | Три шеврони, що пульсують зліва направо протягом `900 ms`. |

### Геометрія RecordingAudio

Значення відносні до розміру бейджа.

* Крапка джерела на `18%` ширини та `50%` висоти, радіус `11%`, пульсація між `90%` і `100%`.
* Три дуги з радіусами `28%`, `51.5%` і `75%`, кожна пульсує між `90%` і `100%`.
* Кожна дуга охоплює `64°` із центром у напрямку праворуч (від `-32°` до `+32°`).
* Штрих `9%`, пульсує між `85%` і `100%`, із круглими кінцями.
* Непрозорість дуг пульсує між `25%` і `100%`; сусідні дуги зсунуті на `0.18` циклу.
* Вертикальні смуги еквалайзера для цього стану не використовуються.

### Геометрія SendingFile

* Три нерухомі шеврони з двох штрихів із круглими кінцями кожен.
* Непрозорість і товщина штриха пульсують за кривою smoothstep зліва направо.
* Звичайний спінер для цього стану не використовується.

## API

| Властивість | Тип | Типове значення |
| :--- | :--- | :--- |
| `status` | `PresenceStatus` | обов’язковий |
| `size` | DU | `3` |
| `text` | string | немає; необов'язковий підпис поруч із бейджем |
| `text_slot` | слот | немає; власний вміст підпису |
| `gap` | DU | `2` |
| `font_size` | TU | `type_caption_size` |
| `line_height` | TU | `type_caption_line` |
| `aria_label` | string | назва статусу |

## Кастомізація

| Метричний проп | Одиниця | Змінна | Типове значення |
| :--- | :--- | :--- | :--- |
| `size` | DU | `--op-status-badge-size` | `pad_card` (`3 DU`) |
| `gap` | DU | `--op-status-badge-gap` | `pad_small` (`2 DU`) |
| `font_size` | TU | `--op-status-badge-font-size` | `type_caption_size` |
| `line_height` | TU | `--op-status-badge-line-height` | `type_caption_line` |

| Змінна кольору | Типове значення |
| :--- | :--- |
| `--op-status-badge-online` | `status_online` |
| `--op-status-badge-offline` | `status_offline` |
| `--op-status-badge-alarm` | `status_alarm` |
| `--op-status-badge-error` | `status_error` |
| `--op-status-badge-fallback` | `status_fallback` |
| `--op-status-badge-activity` | `status_activity` |
| `--op-status-badge-border` | `status_border` |
| `--op-status-badge-text` | `text_primary` |

| Слот | Розташування |
| :--- | :--- |
| `text_slot` | Підпис поруч із бейджем. |

`class` і `style` застосовуються до кореня.

### Рядки

| Ключ | Типове значення |
| :--- | :--- |
| `status-badge.online` | `Online` |
| `status-badge.offline` | `Offline` |
| `status-badge.alarm` | `Alarm` |
| `status-badge.pending` | `Pending` |
| `status-badge.error` | `Error` |
| `status-badge.typing` | `Typing` |
| `status-badge.recording-audio` | `Recording audio` |
| `status-badge.sending-file` | `Sending file` |
