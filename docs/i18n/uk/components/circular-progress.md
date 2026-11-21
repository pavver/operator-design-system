# CircularProgress

Кільцевий індикатор, що відображає відсоток виконання завдання або невизначену фонову активність.
Він підтримує необов’язкові числові мітки по центру або довільні слоти.

Статус: **чернетка** у вебреалізації.

## Використання

Використовуйте `CircularProgress` для відображення поступу на інформаційних панелях, у віджетах статистики або компактних картках, де лінійні смуги займали б забагато горизонтального простору.
Не використовуйте `CircularProgress` для смуг завантаження на всю сторінку; натомість використовуйте [LoadingBar](loading-bar.md) або [LinearProgress](linear-progress.md).

## Геометрія

| Властивість | Значення |
| :--- | :--- |
| Типовий розмір | `10 DU` (`--op-circular-progress-size`). |
| Товщина | `0.75 DU` (`--op-circular-progress-thickness`). |

## Поведінка

* Коли надано `value`, кільце рендериться детерміновано від 0% до 100%.
* Коли `value` пропущено, кільце безперервно обертається в невизначеному режимі.
* Колір кільця відображає семантичний `state` (`Pending`, `Success`, `Error`).
* Числова мітка по центру масштабується пропорційно до розміру кільця або може бути явно перевизначена через `font_size`.

## API

| Властивість | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- |
| `value` | option signal f32 | `None` | Значення поступу від `0.0` до `max`. |
| `max` | signal f32 | `100.0` | Максимальне значення поступу. |
| `state` | signal `ControlState` | немає | Стан керування для тонування. |
| `show_value` | bool | `false` | Показує мітку поступу по центру. |
| `format` | CircularProgressFormat | `Percent` | Форматує як `Percent` або `Count`. |
| `size` | DU | `10 DU` | Зовнішній діаметр кільця. |
| `thickness` | DU | `0.75 DU` | Товщина штриха. |
| `font_size` | TU | `--op-circular-progress-font-size` | Розмір шрифту тексту по центру. |
| `label` | string | `circular-progress.label` | Доступний опис. |
| `center_slot` | Slot | немає | Користувацький слот у центрі кільця. |
| `class` | string | немає | CSS-класи на кореневому елементі. |
| `style` | string | немає | Інлайн-CSS на кореневому елементі. |

## Кастомізація

| Метричний проп | Одиниця | Змінна | Типове значення |
| :--- | :--- | :--- | :--- |
| `size` | DU | `--op-circular-progress-size` | `10 DU` |
| `thickness` | DU | `--op-circular-progress-thickness` | `0.75 DU` |
| `font_size` | TU | `--op-circular-progress-font-size` | `0.75 * size` |

| Змінна кольору | Типове значення |
| :--- | :--- |
| `--op-circular-progress-color` | `var(--op-accent)` |
| `--op-circular-progress-track-color` | `var(--op-surface-raised)` |
| `--op-circular-progress-text` | `var(--op-text-primary)` |

`class` та `style` застосовуються до кореневого `div`.

### Рядки

| Ключ | Типове значення |
| :--- | :--- |
| `circular-progress.label` | `"Progress"` |

### Піктограми

`CircularProgress` не рендерить піктограм.

## Доступність

`CircularProgress` реалізує `role="progressbar"` з атрибутами `aria-valuenow`, `aria-valuemin` та `aria-valuemax`.
Програми зчитування з екрана оголошують мітку поступу та числові зміни стану.