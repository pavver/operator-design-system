# Primitives

Малі будівельні блоки, які використовують інші компоненти.

Статус: **reviewed** у вебреалізації.

## Icon

* Відображає одну піктограму з [tokens/icons](../../tokens/icons), див. [iconography.md](../foundations/iconography.md).
* Розмір у DU, типово `icon_large` (`6 DU`).
* Використовує поточний колір вмісту, якщо колір не задано.
* Без підпису прихована від допоміжних технологій; із підписом є зображенням із цією назвою.

## ProgressIndicator

* Кругова дуга, що охоплює 75% кола й лінійно обертається раз на `spinner_lap` (`900 ms`).
* Розмір і штрих у DU; типово `6 DU` зі штрихом `0.5 DU`.
* Усередині елементів керування використовує `spinner_stroke` (`0.375 DU`) і колір `pending`.
* Круглі кінці лінії.
* Асинхронні елементи керування не використовують його безпосередньо під час `Pending`; вони використовують `PendingIndicator`.

## PendingIndicator

* Єдиний спінер, що показується всередині асинхронних елементів керування у стані `Pending`.
* Обгортає `ProgressIndicator` з `spinner_stroke` (`0.375 DU`) і кольором `pending`.
* Розмір у DU, типово `icon_small` (`4 DU`).
* Плавно з'являється за `pending_indicator_fade` (`200 ms`).
* Керується `use_pending_indicator`, який стає `true`, коли `Pending` триває вже `pending_indicator_delay` (`700 ms`).
* Заміна його вигляду змінює спінер одразу в усіх елементах керування.
* Поведінку визначено в [states.md](../foundations/states.md#delayed-spinner).

## Divider

* Горизонтальна лінія на всю ширину, товщина `0.25 DU`, колір `divider`.

## Card

* Прямокутний контейнер із внутрішнім відступом `pad_card` (`3 DU`) і контуром `0.25 DU` у кольорі `border`.
* Типовий фон `surface`; `surface_alt` для заглиблених областей відображення.
* Радіус кута `0 DU`, без тіні.
* Підтримує слоти контейнера у стилі Quasar: `header`, `actions`, `footer`, із типовими дочірніми елементами як тілом.

## API

| Компонент | Властивості |
| :--- | :--- |
| `Icon` | `icon`, `size` (`6 DU`), `color`, `label`. |
| `ProgressIndicator` | `size` (`6 DU`), `stroke` (`0.5 DU`), `color`, `label` (`progress-indicator.loading`). |
| `PendingIndicator` | `size` (`4 DU`), `stroke` (`0.375 DU`). |
| `Divider` | `thickness` (`0.25 DU`). |
| `Card` | `alt` (`false`), `header`, `actions`, `footer`, `padding` (`3 DU`), `gap` (`2 DU`), `border_width` (`0.25 DU`), `children`. |

## Кастомізація

| Метричний проп | Одиниця | Змінна | Типове значення | Компонент |
| :--- | :--- | :--- | :--- | :--- |
| `size` | DU | `--op-icon-size` | `icon_large` (`6 DU`) | Icon |
| `size` | DU | `--op-progress-indicator-size` | `6 DU` | ProgressIndicator |
| `stroke` | DU | `--op-progress-indicator-stroke` | `0.5 DU` | ProgressIndicator |
| `size` | DU | `--op-pending-indicator-size` | `icon_small` (`4 DU`) | PendingIndicator |
| `stroke` | DU | `--op-pending-indicator-stroke` | `spinner_stroke` (`0.375 DU`) | PendingIndicator |
| `thickness` | DU | `--op-divider-thickness` | `border` (`0.25 DU`) | Divider |
| `padding` | DU | `--op-card-padding` | `pad_card` (`3 DU`) | Card |
| `gap` | DU | `--op-card-gap` | `gutter` (`2 DU`) | Card |
| `border_width` | DU | `--op-card-border-width` | `border` (`0.25 DU`) | Card |

| Змінна кольору | Типове значення | Компонент |
| :--- | :--- | :--- |
| `--op-progress-indicator-color` | `accent` | ProgressIndicator |
| `--op-pending-indicator-color` | `pending` | PendingIndicator |
| `--op-divider-color` | `divider` | Divider |
| `--op-card-background` | `surface` | Card |
| `--op-card-alt-background` | `surface_alt` | Card |
| `--op-card-border` | `border` | Card |

| Слот | Розташування | Компонент |
| :--- | :--- | :--- |
| `header` | Розділ заголовка картки над тілом. | Card |
| `actions` | Розділ дій картки під тілом. | Card |
| `footer` | Нижня частина картки під діями. | Card |

`class` і `style` застосовуються до кореня кожного компонента.

### Рядки

| Ключ | Типове значення |
| :--- | :--- |
| `progress-indicator.loading` | `Loading` |
