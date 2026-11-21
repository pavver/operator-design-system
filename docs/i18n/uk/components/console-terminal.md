# ConsoleTerminal

Прокручуване подання журналу із забарвленням рядків на основі тегів.

Статус: **reviewed** у вебреалізації.

## Геометрія

| Властивість | Значення |
| :--- | :--- |
| Ширина | Заповнює батьківський елемент. |
| Висота | `45 DU` за замовчуванням. |
| Внутрішній відступ | `gutter` (`2 DU`). |
| Текст | Стиль `console`. |
| Контур | `0.25 DU` у `border`. |
| Радіус кутів | `0 DU`. |

## Кольори

Термінал використовує лише палітру `term_*` і ніколи не використовує поверхні інтерфейсу чи сигнальні кольори.

Кольори та текстові атрибути надходять із керуючих послідовностей xterm SGR (`ESC [ … m`) у тексті журналу, як у справжньому терміналі.
Термінал ніколи не забарвлює рядок за його вмістом.

| Послідовність | Ефект |
| :--- | :--- |
| `30`–`37`, `40`–`47` | Колір тексту та тло від `term_black` … `term_white`. |
| `90`–`97`, `100`–`107` | Колір тексту та тло від `term_bright_black` … `term_bright_white`. |
| `38;5;n`, `48;5;n` | 256-колірна палітра xterm; `0`–`15` відображаються на токени `term_*`. |
| `38;2;r;g;b`, `48;2;r;g;b` | True color; форми з двокрапкою `38:2::r:g:b` та `38:5:n` також працюють. |
| `39`, `49` | Типовий колір тексту `term_foreground` і типове тло. |
| `1`, `2`, `3`, `4`, `9` | Жирний, приглушений (60% непрозорості), курсив, підкреслений, закреслений. |
| `7` | Інверсія: міняє місцями колір тексту та тло, використовуючи кольори термінала як типові. |
| `22`, `23`, `24`, `27`, `29` | Вимикають відповідний атрибут. |
| `0` або порожньо | Скидання. |

* Стиль переноситься від одного запису до наступного, доки не відбудеться скидання.
* Інші керуючі послідовності (переміщення курсора, стирання, заголовки та посилання OSC, вибір набору символів) видаляються.
* Символ нового рядка всередині запису починає новий рядок.
* Одиночне повернення каретки перезаписує поточний рядок, тому виведення перебігу показує лише свій останній стан.

Тло — `term_background`.

## Поведінка

* Коли додається рядок, подання прокручується до найновішого рядка.
* Користувач може прокручувати історію назад.
* Довгі рядки переносяться.

## API

| Властивість | Тип | Типове значення |
| :--- | :--- | :--- |
| `logs` | список рядків | обов'язковий |
| `height` | DU | `45` |

### API кастомізації

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `gap` | optional number (DU) | `gap_micro` | Перевизначає `--op-console-terminal-gap`. |
| `padding` | optional number (DU) | `gutter` | Перевизначає `--op-console-terminal-padding`. |
| `font_size` | optional number (TU) | `type_console_size` | Перевизначає `--op-console-terminal-font-size`. |
| `line_height` | optional number (TU) | `type_console_line` | Перевизначає `--op-console-terminal-line-height`. |
| `height` | optional number (DU) | `45 DU` | Перевизначає `--op-console-terminal-height`. |
| `header` | slot | none | Перед прокручуваним журналом. |
| `footer` | slot | none | Після прокручуваного журналу. |
| `empty` | slot | none | Коли вхідний журнал порожній; типово залишається порожнім. |
| `label` | optional string | `console-terminal.label` | Перевизначення тексту. |
| `class` | optional string | none | Додається до кореневих класів. |
| `style` | optional string | none | Додається після кореневих оголошень метрик. |

## Кастомізація

`class` і `style` застосовуються до кореня.
Явні кореневі стилі мають перевагу над метричними пропсами; невстановлені метрики зберігають початковий рендеринг.

| Метричний проп | Одиниця | Змінна | Типове значення |
| :--- | :--- | :--- | :--- |
| `gap` | DU | `--op-console-terminal-gap` | `gap_micro` |
| `padding` | DU | `--op-console-terminal-padding` | `gutter` |
| `font_size` | TU | `--op-console-terminal-font-size` | `type_console_size` |
| `line_height` | TU | `--op-console-terminal-line-height` | `type_console_line` |
| `height` | DU | `--op-console-terminal-height` | `45 DU` |

Товщина меж залишається фіксованою геометрією, вказаною вище.

| Змінна кольору | Типове значення |
| :--- | :--- |
| `--op-console-terminal-background` | `term_background` |
| `--op-console-terminal-text` | `term_foreground` |
| `--op-console-terminal-border` | `border` |
| `--op-console-terminal-hover-border` | `border_strong` |
| `--op-console-terminal-black` | `term_black` |
| `--op-console-terminal-bright-black` | `term_bright_black` |
| `--op-console-terminal-red` | `term_red` |
| `--op-console-terminal-bright-red` | `term_bright_red` |
| `--op-console-terminal-green` | `term_green` |
| `--op-console-terminal-bright-green` | `term_bright_green` |
| `--op-console-terminal-yellow` | `term_yellow` |
| `--op-console-terminal-bright-yellow` | `term_bright_yellow` |
| `--op-console-terminal-blue` | `term_blue` |
| `--op-console-terminal-bright-blue` | `term_bright_blue` |
| `--op-console-terminal-magenta` | `term_magenta` |
| `--op-console-terminal-bright-magenta` | `term_bright_magenta` |
| `--op-console-terminal-cyan` | `term_cyan` |
| `--op-console-terminal-bright-cyan` | `term_bright_cyan` |
| `--op-console-terminal-white` | `term_white` |
| `--op-console-terminal-bright-white` | `term_bright_white` |

| Слот | Розташування |
| :--- | :--- |
| `header` | Перед прокручуваним журналом. |
| `footer` | Після прокручуваного журналу. |
| `empty` | Коли вхідний журнал порожній; типово залишається порожнім. |

### Рядки

Порядок розв'язання: проп, пошук застосунком, англійське типове значення.

| Ключ | Типове значення |
| :--- | :--- |
| `console-terminal.label` | `empty; preserves the unnamed log` |

### Піктограми

Немає вбудованих піктограм.
