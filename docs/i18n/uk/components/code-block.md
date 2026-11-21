# CodeBlock

Моноширинний блок для коду, конфігурації та виведення журналу, із кнопкою копіювання.

Статус: **reviewed** у вебреалізації.

## Геометрія

| Властивість | Значення |
| :--- | :--- |
| Тло | `surface_alt` із контуром `border` товщиною `0.25 DU`. |
| Заголовок | Необов'язковий, щонайменше `8 DU`: мітка мови у верхньому регістрі `caption`, кнопка копіювання в кінці, `divider` знизу. |
| Тіло | Стиль `console`, відступ `3 DU`. |
| Номери рядків | Стовпець `8 DU` у кольорі `text_disabled`, вирівняний по кінцю, без можливості виділення. |
| Висота | Висота вмісту або `max_height` із подальшим прокручуванням. |

## Підсвічування синтаксису

Властивість `language` також вибирає підсвічувач.
Вбудовані мови: Rust (`rust`, `rs`), Kotlin (`kotlin`, `kt`, `kts`), TOML, JSON (`json`, `jsonc`), YAML (`yaml`, `yml`) та shell (`bash`, `sh`, `shell`, `zsh`, `console`).
Будь-яка інша мова відображається без підсвічування.

| Токен | Колір |
| :--- | :--- |
| Ключові слова | `syntax_keyword` |
| Рядки та символи | `syntax_string` |
| Числа | `syntax_number` |
| Коментарі | `syntax_comment`, курсив |
| Типи та назви з великої літери | `syntax_type` |
| Функції, макроси, вбудовані команди shell | `syntax_function` |
| `true`, `false`, `null`, назви в `ALL_CAPS` | `syntax_constant` |
| Ключі TOML, YAML та JSON; змінні shell | `syntax_property` |
| Атрибути, анотації, лайфтайми, таблиці TOML, якорі YAML | `syntax_meta` |
| Оператори та дужки | `syntax_punctuation` |

Вбудований підсвічувач є невеликим рядковим сканером, тому він не розуміє шаблони рядків, heredoc або блокові скаляри YAML.

## Arborium

Необов'язкова функціональність Cargo `arborium` вмикає підсвічування синтаксису на основі Tree-sitter через крейт arborium.
Крейт operator-ui типово не вмикає граматики жодної мови.
Застосунок вмикає `arborium` разом із конкретними функціональностями `arborium/lang-*`, які йому потрібні, у власному `Cargo.toml`.
Коли arborium не підтримує запитану мову або граматика не скомпільована, підсвічування повертається до вбудованого сканера.
Компіляція arborium для WebAssembly потребує інструментарію LLVM і clang, що підтримує цільову платформу `wasm32-unknown-unknown`.
Увімкнення arborium збільшує розмір скомпільованого релізного бінарного файлу WebAssembly.
У мінімальному застосунку, що рендерить підсвічений блок коду з `arborium/lang-rust`, початковий розмір WebAssembly збільшився приблизно на 3.50 МБ (з 2.28 МБ до 5.78 МБ), а розмір, стиснений gzip, збільшився приблизно на 634 КБ (з 490 КБ до 1.12 МБ).

## Поведінка

* Довгі рядки прокручуються горизонтально; із `wrap` вони переносяться та розриваються будь-де.
* Кнопка копіювання копіює весь текст як асинхронну дію зі станом успіху або помилки.
* Заголовок приховується, коли мову не вказано, а копіювання вимкнено.
* Табуляція має ширину в чотири стовпці.

## API

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `code` | string | обов'язковий | Вміст. |
| `language` | string | немає | Мітка заголовка та підсвічувач. |
| `line_numbers` | bool | `false` | Нумерує кожен рядок. |
| `wrap` | bool | `false` | Переносить довгі рядки. |
| `copy` | bool | `true` | Показує кнопку копіювання. |
| `max_height` | DU | немає | Обмеження висоти. |

### API кастомізації

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `header_height` | optional number (DU) | `row_compact` | Перевизначає `--op-code-block-header-height`. |
| `header_gap` | optional number (DU) | `gutter` | Перевизначає `--op-code-block-header-gap`. |
| `header_padding_start` | optional number (DU) | `pad_card` | Перевизначає `--op-code-block-header-padding-start`. |
| `header_padding_end` | optional number (DU) | `pad_tight` | Перевизначає `--op-code-block-header-padding-end`. |
| `padding` | optional number (DU) | `pad_card` | Перевизначає `--op-code-block-padding`. |
| `number_width` | optional number (DU) | `8 DU` | Перевизначає `--op-code-block-number-width`. |
| `number_padding` | optional number (DU) | `pad_card` | Перевизначає `--op-code-block-number-padding`. |
| `copy_size` | optional number (DU) | `row_dense` | Перевизначає `--op-code-block-copy-size`. |
| `icon_size` | optional number (DU) | `3.5 DU` | Перевизначає `--op-code-block-icon-size`. |
| `font_size` | optional number (TU) | `type_console_size` | Перевизначає `--op-code-block-font-size`. |
| `line_height` | optional number (TU) | `type_console_line` | Перевизначає `--op-code-block-line-height`. |
| `header_font_size` | optional number (TU) | `type_caption_size` | Перевизначає `--op-code-block-header-font-size`. |
| `header_letter_spacing` | optional number (TU) | `0.04 × type_caption_size` | Перевизначає `--op-code-block-header-letter-spacing`. |
| `max_height` | optional number (DU) | `100%` | Перевизначає `--op-code-block-max-height`. |
| `header` | slot | немає | Замінює мітку мови; зберігає заголовок і кнопку копіювання. |
| `actions` | slot | немає | Додатковий вміст заголовка перед кнопкою копіювання. |
| `footer` | slot | немає | Під кодом, що прокручується. |
| `content` | slot | немає | Замінює тіло коду, що прокручується. |
| `copy_label` | optional string | `code-block.copy` | Перевизначення тексту. |
| `copy_icon` | optional icon | `code-block.copy` | Перевизначення піктограми. |
| `class` | optional string | немає | Додається до класів кореня. |
| `style` | optional string | немає | Додається після оголошень метрик кореня. |

## Кастомізація

`class` і `style` застосовуються до кореня.
Явні стилі кореня мають перевагу над метричними пропсами; невстановлені метрики зберігають початкове відображення.

| Метричний проп | Одиниця | Змінна | Типове значення |
| :--- | :--- | :--- | :--- |
| `header_height` | DU | `--op-code-block-header-height` | `row_compact` |
| `header_gap` | DU | `--op-code-block-header-gap` | `gutter` |
| `header_padding_start` | DU | `--op-code-block-header-padding-start` | `pad_card` |
| `header_padding_end` | DU | `--op-code-block-header-padding-end` | `pad_tight` |
| `padding` | DU | `--op-code-block-padding` | `pad_card` |
| `number_width` | DU | `--op-code-block-number-width` | `8 DU` |
| `number_padding` | DU | `--op-code-block-number-padding` | `pad_card` |
| `copy_size` | DU | `--op-code-block-copy-size` | `row_dense` |
| `icon_size` | DU | `--op-code-block-icon-size` | `3.5 DU` |
| `font_size` | TU | `--op-code-block-font-size` | `type_console_size` |
| `line_height` | TU | `--op-code-block-line-height` | `type_console_line` |
| `header_font_size` | TU | `--op-code-block-header-font-size` | `type_caption_size` |
| `header_letter_spacing` | TU | `--op-code-block-header-letter-spacing` | `0.04 × type_caption_size` |
| `max_height` | DU | `--op-code-block-max-height` | `100%` |

Товщина меж залишається фіксованою геометрією, зазначеною вище.

| Змінна кольору | Типове значення |
| :--- | :--- |
| `--op-code-block-background` | `surface_alt` |
| `--op-code-block-border` | `border` |
| `--op-code-block-divider` | `divider` |
| `--op-code-block-secondary-text` | `text_secondary` |
| `--op-code-block-text` | `text_primary` |
| `--op-code-block-number` | `text_disabled` |
| `--op-code-block-syntax-keyword` | `syntax_keyword` |
| `--op-code-block-syntax-string` | `syntax_string` |
| `--op-code-block-syntax-number` | `syntax_number` |
| `--op-code-block-syntax-comment` | `syntax_comment` |
| `--op-code-block-syntax-type` | `syntax_type` |
| `--op-code-block-syntax-function` | `syntax_function` |
| `--op-code-block-syntax-constant` | `syntax_constant` |
| `--op-code-block-syntax-property` | `syntax_property` |
| `--op-code-block-syntax-meta` | `syntax_meta` |
| `--op-code-block-syntax-punctuation` | `syntax_punctuation` |

| Слот | Розташування |
| :--- | :--- |
| `header` | Замінює мітку мови; зберігає заголовок і кнопку копіювання. |
| `actions` | Додатковий вміст заголовка перед кнопкою копіювання. |
| `footer` | Під кодом, що прокручується. |
| `content` | Замінює тіло коду, що прокручується. |

### Рядки

Порядок розв'язання: проп, пошук застосунком, англійське типове значення.

| Ключ | Типове значення |
| :--- | :--- |
| `code-block.copy` | `Copy code` |

Кнопка копіювання зберігає ключі зворотного зв'язку IconButton `icon-button.loading`, `icon-button.success` та `icon-button.error`.

### Піктограми

| Ключ | Типове значення |
| :--- | :--- |
| `code-block.copy` | `content_copy` |

Кнопка копіювання зберігає ключі піктограм IconButton `icon-button.success` та `icon-button.error`.