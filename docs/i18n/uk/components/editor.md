# Editor

Редагування форматованого тексту в браузері з доступною панеллю інструментів і обмеженим HTML на виході.
Надає WYSIWYG-редагування на основі оболонки Field з очищеною розміткою на виході.

Статус: **чернетка** у вебреалізації.

## Використання

* Форматування нотаток, коментарів, статей або документації.
* Створення очищеного HTML за допомогою інтерактивних команд форматування тексту.

## Геометрія

| Властивість | Значення |
| :--- | :--- |
| Мінімальна висота | `40 DU`. |
| Максимальна висота | `100 DU`. |
| Внутрішній відступ | `pad_small` (`1.5 DU`). |
| Проміжок | `gap_micro` (`0.5 DU`). |
| Радіус заокруглення кутів | `0 DU`. |

## Поведінка

* Область редагування використовує контейнер contenteditable з очищенням вставленого вмісту.
* Кнопки панелі інструментів виконують команди форматування (жирний, курсив, списки, заголовки, посилання, скасування та повторення).
* Активні стани кнопок панелі інструментів відстежують поточне виділення тексту.
* Додавання посилання відкриває вбудований діалог, який перевіряє URL за дозволеними протоколами.
* Зміни значення очищують вихідний HTML і передають його через `on_change`.

## API

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `value` | сигнал рядка | обов’язковий | Сигнал рядка HTML-вмісту. |
| `toolbar` | `Vec<EditorCommand>` | усі | Список увімкнених команд панелі інструментів. |
| `on_change` | зворотний виклик `(String)` | немає | Сповіщає про оновлення очищеного HTML. |
| `enabled` | bool | `true` | Вмикає редагування та кнопки панелі інструментів. |
| `readonly` | bool | `false` | Переводить редактор у режим лише для перегляду. |
| `state` | `ControlState` | немає | Явне перевизначення стану елемента керування. |
| `label` | string | `editor.label` | Видимий підпис і доступна назва. |
| `placeholder` | string | `editor.placeholder` | Плейсхолдер порожнього поля. |
| `link_label` | string | `editor.url` | Доступний підпис поля введення посилання. |
| `apply_label` | string | `editor.apply` | Підпис кнопки застосування посилання. |
| `cancel_label` | string | `editor.cancel` | Підпис кнопки скасування посилання. |
| `invalid_link_label` | string | `editor.invalid-link` | Повідомлення про помилку для некоректних URL. |
| `header` | `Slot` | немає | Слот над панеллю інструментів. |
| `actions` | `Slot` | немає | Власні кнопки всередині панелі інструментів. |
| `footer` | `Slot` | немає | Слот під областю редагування. |
| `min_height` | метрика DU | `40 DU` | Мінімальна висота області редактора. |
| `max_height` | метрика DU | `100 DU` | Максимальна висота до появи прокручування. |
| `padding` | метрика DU | `pad_small` | Внутрішній відступ редактора. |
| `gap` | метрика DU | `gap_micro` | Проміжок між елементами панелі інструментів. |
| `font_size` | метрика TU | `type_body_size` | Розмір основного шрифту. |
| `line_height` | метрика TU | `type_body_line` | Висота рядка основного тексту. |
| `class`, `style` | string | немає | Кореневі хуки. |

## Кастомізація

| Метричний проп | Одиниця | Змінна | Типове значення |
| :--- | :--- | :--- | :--- |
| `min_height` | DU | `--op-editor-min-height` | `40 DU` |
| `max_height` | DU | `--op-editor-max-height` | `100 DU` |
| `padding` | DU | `--op-editor-padding` | `pad_small` (`1.5 DU`) |
| `gap` | DU | `--op-editor-gap` | `gap_micro` (`0.5 DU`) |
| `font_size` | TU | `--op-editor-font-size` | `type_body_size` |
| `line_height` | TU | `--op-editor-line-height` | `type_body_line` |

| Змінна кольору | Типове значення |
| :--- | :--- |
| `--op-editor-background` | `surface` |
| `--op-editor-text` | `text_primary` |
| `--op-editor-border` | `border` |
| `--op-editor-accent` | `accent` |
| `--op-editor-selected-background` | `hover` |
| `--op-editor-disabled-text` | `text_disabled` |

| Слот | Розташування |
| :--- | :--- |
| `header` | Над панеллю інструментів. |
| `actions` | Усередині рядка панелі інструментів. |
| `footer` | Під областю вмісту. |

`class` і `style` застосовуються до кореневого елемента Field.

### Рядки

| Ключ | Типове значення |
| :--- | :--- |
| `editor.label` | `Rich text` |
| `editor.placeholder` | `Write something` |
| `editor.url` | `Link URL` |
| `editor.apply` | `Apply link` |
| `editor.cancel` | `Cancel` |
| `editor.invalid-link` | `Enter an HTTP, HTTPS, mail or relative URL` |
| `editor.bold` | `Bold` |
| `editor.italic` | `Italic` |
| `editor.underline` | `Underline` |
| `editor.strike` | `Strikethrough` |
| `editor.bullets` | `Bullet list` |
| `editor.numbers` | `Numbered list` |
| `editor.link` | `Link` |
| `editor.paragraph` | `Paragraph` |
| `editor.heading1` | `Heading 1` |
| `editor.heading2` | `Heading 2` |
| `editor.undo` | `Undo` |
| `editor.redo` | `Redo` |

### Піктограми

Editor не визначає вбудованих піктограм.

## Доступність

Область вмісту редактора використовує `role="textbox"` з `aria-multiline="true"` та `aria-readonly`.
Панель інструментів має `role="toolbar"`, а кнопки команд є щільними квадратними кнопками з піктограмами, згрупованими розділювачами та зі спливними підказками.
Помилки посилань повідомляються через `role="alert"`.
