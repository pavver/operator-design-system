# VideoStream

Оболонка відтворення живого відеопотоку та потоків камер навколо HTML-елемента `<video>`.

Статус: **draft** на web.

## Призначення

Використовуйте `VideoStream` для відображення живих потоків камер, трансляцій WebRTC/MediaStream або телеметрії спостереження.
Не використовуйте `VideoStream` для статичних вбудованих відеороликів через iframe; використовуйте [Video](video.md).

## Геометрія

| Властивість | Значення |
| :--- | :--- |
| Радіус кутів | `0 DU` (стандартна пряма геометрія). |
| Співвідношення сторін за замовчуванням | `16:9` (`1.7778`). |
| Межа | Стандартна межа `0.25 DU` всередині меж елемента. |

## Поведінка

* Вбудовує нативний елемент `<video>` з апаратним прискоренням відтворення.
* Може підключатися безпосередньо до URL потоку або активного браузерного об'єкта `MediaStream`.
* Автоматично відображає стани підключення, відтворення, зависання буфера та помилок.
* Показує інтерактивний шар керування з пульсуючим бейджем LIVE, перемикачем звуку та кнопкою повного екрана.
* Панель керування автоматично зникає під час неактивності курсора.

## API

| Властивість | Тип | За замовчуванням | Опис |
| :--- | :--- | :--- | :--- |
| `src` | signal string | немає | URL джерела відео. |
| `stream` | signal MediaStream | немає | Об'єкт живого потоку браузера MediaStream. |
| `state` | VideoStreamState | немає | Явне перевизначення стану відтворення. |
| `title` | string | `"Live stream"` | Доступна назва потоку. |
| `ratio` | f32 | `1.7778` | Співвідношення сторін (ширина / висота). |
| `max_width` | DU | немає | Максимальна ширина в DU. |
| `max_height` | DU | немає | Максимальна висота в DU. |
| `muted` | bool | `true` | Стан вимкнення звуку; за замовчуванням увімкнено для автозапуску в браузерах. |
| `autoplay` | bool | `true` | Прапорець автозапуску. |
| `live` | bool | `true` | Показує пульсуючий бейдж LIVE. |
| `on_error` | callback string | немає | Викликається у разі помилки відтворення. |
| `header_slot` | Slot | немає | Користувацький слот у верхній частині панелі. |
| `controls_slot` | Slot | немає | Користувацький слот у нижній панелі керування. |
| `overlay_slot` | Slot | немає | Повноекранний шар оверлею (наприклад, рамки комп'ютерного зору). |
| `connecting_slot` | Slot | немає | Користувацький слот замість стандартного індикатора завантаження. |
| `error_slot` | Slot | немає | Користувацький слот замість стандартного блоку помилки. |
| `connecting_label` | string | `"Connecting stream..."` | Текст стану з'єднання. |
| `stalled_label` | string | `"Stream stalled..."` | Текст стану зависання буфера. |
| `error_label` | string | `"Video stream error"` | Текст заголовка помилки. |
| `load_error_label` | string | `"Failed to load video stream"` | Повідомлення про невдале завантаження. |
| `empty_label` | string | `"No active stream"` | Текст за відсутності активного потоку. |
| `live_label` | string | `"LIVE"` | Текст бейджа прямого ефіру. |
| `mute_label` | string | `"Mute"` | Текст кнопки вимкнення звуку. |
| `fullscreen_label` | string | `"Fullscreen"` | Текст кнопки повноекранного режиму. |
| `class` | string | немає | CSS-класи кореневого елемента. |
| `style` | string | немає | Вбудовані стилі кореневого елемента. |

## Кастомізація

| Проп метрики | Одиниця | Змінна | За замовчуванням |
| :--- | :--- | :--- | :--- |
| `max_width` | DU | `--op-video-stream-max-width` | `100%` |
| `max_height` | DU | `--op-video-stream-max-height` | `100%` |
| `border_width` | DU | `--op-video-stream-border-width` | `0.25 DU` |

| Змінна кольору | За замовчуванням |
| :--- | :--- |
| `--op-video-stream-bg` | `#000000` |
| `--op-video-stream-border` | `var(--op-border)` |

`class` та `style` застосовуються до кореневого контейнера.

### Рядки

| Ключ | За замовчуванням |
| :--- | :--- |
| `video-stream.title` | `"Live stream"` |
| `video-stream.connecting` | `"Connecting stream..."` |
| `video-stream.stalled` | `"Stream stalled..."` |
| `video-stream.error` | `"Video stream error"` |
| `video-stream.load-error` | `"Failed to load video stream"` |
| `video-stream.empty` | `"No active stream"` |
| `video-stream.live` | `"LIVE"` |
| `video-stream.mute` | `"Mute"` |
| `video-stream.fullscreen` | `"Fullscreen"` |

### Піктограми

| Ключ | За замовчуванням |
| :--- | :--- |
| Заглушка відсутності потоку | `icons::VIDEOCAM` |
| Індикатор завантаження | `icons::REFRESH` |
| Помилка | `icons::ERROR` |
| Звук | `icons::VISIBILITY` |
| Повний екран | `icons::OPEN_IN_NEW` |

## Доступність

Контейнер відео має `role="region"` та отримує доступну назву потоку.
