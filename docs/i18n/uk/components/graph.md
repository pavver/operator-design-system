# Graph

Частини редактора графа нод: GraphCanvas, NodeCard із NodeRow і Socket, Wire, GraphFrame та Minimap.
Вони залежать від проєкту: фіча `graph` належить до групи `extensions`, а не до `full`.
Вони нічого не знають про рушій або його типи; застосунок зберігає граф, розташовує ноди та пов'язує взаємодії.

Статус: **чернетка** у вебреалізації.

## GraphCanvas

* Шар світових координат, який переміщується за допомогою `Camera` (`screen = world × zoom + offset`), та екранний оверлей для панелей інструментів і мінімапи.
* Коліщатко масштабує навколо вказівника між `min_zoom` (`0.25`) і `max_zoom` (`2`); середня кнопка панорамує будь-де; перетягування основною кнопкою на порожньому фоні панорамує, коли встановлено `pan_on_background`.
* `on_background_down` повідомляє про натискання на порожньому фоні для рамок виділення.
* Сітка малюється лініями `divider` через кожні `grid` DU (`5`) і слідує за камерою.
* `Camera` має `to_world`, `to_screen`, `zoom_at`, `pan`, `visible`, `centered_on` і `fit` (масштабування до вмісту або виділення).

## NodeCard

| Властивість | Значення |
| :--- | :--- |
| Заголовок | `row_compact` (`8 DU`), `surface_alt`, жирний заголовок, текст категорії та верхня лінія `0.5 DU` у кольорі категорії, піктограма `info` із [HoverCard](hover-card.md) для довідки, дії в кінці. |
| Тіло | Сітка із двох стовпців, спільна для всіх рядків: підписи займають `fit-content(40%)`, елементи керування — решту. |
| Рядок | `row_standard` (`10 DU`); рядок `content` зростає разом зі своїм елементом керування (текстом, списками) від цього мінімуму. |
| Сокет | Квадрат `2.5 DU` із контуром `0.5 DU` у кольорі типу, заповнений при з'єднанні; сокети виконання мають форму трикутника. Центрований на краю картки посередині першого рядка тексту. |
| Вибрана | Контур `0.5 DU` у кольорі `accent`. |
| Помилка | Лінія `0.5 DU` у кольорі `error` під заголовком; рядки з помилками мають підписи `error`. |

* Картка вимірює центри своїх сокетів у світових одиницях і повідомляє їх через `on_sockets` при зміні геометрії картки, заголовка, рядка чи сокета, зокрема після вставлення слотів і зміни реактивних метрик.
  З'єднання слідують за виміряною геометрією DOM без окремих констант розташування рядків у Rust.
* `on_header_down` повідомляє про натискання на заголовку для перетягування; піктограма довідки та дії не починають перетягування.

## Wire

* Кубічна крива від виходу до входу, що виходить і входить горизонтально, через необов'язкові точки маршруту з плавними дотичними.
* Штрих `0.5 DU` у кольорі типу (`0.75 DU` для виконання); `selected` використовує `accent`, `preview` і `detached` є штриховими.
* Ширший невидимий контур для влучання спрощує натискання на тонкі з'єднання.
* Необов'язковий підпис розміщується в рамці посередині маршруту.

## GraphFrame

Іменована область позаду нод із тонованим заповненням і заголовком `row_compact` у її кольорі; `on_header_down` перетягує її.

## Minimap

Огляд прямокутників елементів і видимої області (контур `accent`), `50 × 35 DU` у правому нижньому куті оверлею.
Натискання або перетягування центрує камеру на цій точці.

## API

Усі компоненти графа приймають реактивні `class` і `style` (`MaybeProp<String>`).
Метричні пропси є реактивними `MaybeProp<f32>`, якщо не зазначено інше; нижче перелічено їхніх власників, одиниці, змінні та типові значення.
`grid` зберігає значення в DU та приймає реактивні перевизначення; `size` залишається парою значень у DU.
Позиції, `NodeCard.width`, `WorldRect` та кінцеві точки з'єднань зберігають одиниці світових координат для сумісності з камерою та виміряними координатами сокетів.
Ці вхідні дані геометрії застосунку відокремлені від метрик компонентів у DU.

| Компонент | Проп | Тип | Типове значення | Опис |
| :--- | :--- | :--- | :--- | :--- |
| GraphCanvas | `camera` | `RwSignal<Camera>` | обов’язковий | Камера світових координат. |
| GraphCanvas | `min_zoom`, `max_zoom` | `f64` | `0.25`, `2` | Межі масштабування. |
| GraphCanvas | `pan_on_background` | bool | true | Панорамування основною кнопкою. |
| GraphCanvas | `on_background_down` | зворотний виклик вказівника | немає | Натискання на фоні. |
| GraphCanvas | `children`, `overlay` | children, необов'язкове замикання дочірнього вмісту | обов'язковий, немає | Вміст у світових та екранних координатах. |
| GraphCanvas | `is_empty`, `empty` | сигнал bool, Slot | false, порожній | Показує слот порожнього полотна, коли викликач не має нод. |
| NodeCard | `x`, `y`, `width`, `title` | числові сигнали, сигнал рядка | обов’язковий | Прямокутник у світових координатах і заголовок. |
| NodeCard | `category`, `color` | необов'язкові сигнали рядка | немає | Категорія та відтінок типу; колір записує `--op-graph-node-color`. |
| NodeCard | `selected`, `error` | сигнали bool | false | Візуальні стани. |
| NodeCard | `help`, `help_label`, `help_icon` | view, string, icon | немає, `graph.node-help`, `graph.node-help` | Довідковий вміст і семантичні перевизначення. |
| NodeCard | `actions`, `children` | необов'язкові замикання дочірнього вмісту | немає | Дії заголовка та рядки тіла. |
| NodeCard | `on_header_down`, `on_sockets` | зворотні виклики | немає | Перетягування та виміряні центри сокетів `(id, side, y)`. |
| NodeCard | `header`, `body`, `footer` | Слот | порожній | Іменовані перевизначення вмісту. |
| NodeRow | `label`, `end_label` | необов'язкові сигнали рядка | немає | Підписи рядків, надані викликачем. |
| NodeRow | `input`, `output` | Слот | порожній | Сокети; наявні викликачі ViewFn залишаються підтримуваними. |
| NodeRow | `content`, `error`, `children` | bool, сигнал bool, необов'язкові дочірні елементи | false, false, немає | Рядки зі змінною висотою, стан та елементи керування. |
| Сокет | `id`, `side` | string, SocketSide | обов’язковий | Ідентифікатор з'єднання та край картки. |
| Сокет | `kind`, `connected`, `highlighted`, `missing` | SocketKind, сигнали bool | Data, false | Форма та стани. |
| Сокет | `color`, `on_pointerdown` | необов'язковий сигнал рядка, зворотний виклик | немає | Відтінок типу (`--op-graph-socket-color`) та взаємодія. |
| WireLayer | `children` | children | обов’язковий | SVG-з'єднання. |
| Wire | `from`, `to`, `points` | сигнали точок, сигнал точок маршруту | обов'язковий, обов'язковий, порожній | Маршрут у світових координатах. |
| Wire | `color`, `state`, `exec` | необов'язковий сигнал рядка, сигнал WireState, bool | немає, Normal, false | Відтінок типу (`--op-graph-wire-color`) та вигляд. |
| Wire | `label`, `on_pointerdown` | необов'язковий сигнал рядка, зворотний виклик | немає | Підпис маршруту та взаємодія. |
| GraphFrame | `rect`, `title` | сигнал WorldRect, сигнал рядка | обов’язковий | Область і запасний заголовок. |
| GraphFrame | `color`, `selected` | необов'язковий сигнал рядка, сигнал bool | немає, false | Відтінок (`--op-graph-frame-color`) та вибір. |
| GraphFrame | `on_header_down`, `actions`, `title_slot` | зворотний виклик, необов'язкове замикання дочірнього вмісту, Slot | немає, немає, порожній | Перетягування, дії та вміст заголовка. |
| Minimap | `items`, `size`, `label` | сигнал елементів, `(f32, f32)`, string | обов'язковий, `(50, 35)`, `graph.minimap` | Прямокутники, розміри в DU та доступна назва. |
| Усі | `class`, `style` | MaybeProp string | немає | Кореневі хуки; явний стиль іде після метрик. |

| GraphCanvas | `grid` | MaybeProp, число в DU | 5 | Перевизначення геометрії або типографіки. |
| GraphCanvas | `grid_width` | MaybeProp, число в DU | border (`0.25 DU`) | Перевизначення геометрії або типографіки. |
| WireLayer | `layer_size` | MaybeProp, число в DU | 0.25 | Перевизначення геометрії або типографіки. |
| NodeCard | `header_height` | MaybeProp, число в DU | row_compact (`8 DU`) | Перевизначення геометрії або типографіки. |
| NodeCard | `header_padding_start` | MaybeProp, число в DU | pad_card | Перевизначення геометрії або типографіки. |
| NodeCard | `header_padding_end` | MaybeProp, число в DU | pad_tight | Перевизначення геометрії або типографіки. |
| NodeCard | `header_gap` | MaybeProp, число в DU | pad_tight | Перевизначення геометрії або типографіки. |
| NodeCard | `body_padding_y` | MaybeProp, число в DU | gap_micro | Перевизначення геометрії або типографіки. |
| NodeCard | `label_width` | MaybeProp, число в DU | 40% картки | Перевизначення геометрії або типографіки. |
| NodeCard, NodeRow | `row_height` | MaybeProp, число в DU | row_standard (`10 DU`) | Перевизначення геометрії або типографіки. |
| NodeCard, NodeRow | `row_padding_x` | MaybeProp, число в DU | pad_card | Перевизначення геометрії або типографіки. |
| NodeCard, NodeRow | `row_padding_y` | MaybeProp, число в DU | gap_micro; рядки вмісту | Перевизначення геометрії або типографіки. |
| NodeCard, NodeRow | `row_gap` | MaybeProp, число в DU | pad_tight | Перевизначення геометрії або типографіки. |
| NodeCard, Socket | `socket_size` | MaybeProp, число в DU | 2.5 | Перевизначення геометрії або типографіки. |
| Сокет | `socket_border_width` | MaybeProp, число в DU | border_emphasis (`0.5 DU`) | Перевизначення геометрії або типографіки. |
| Сокет | `socket_outline_width` | MaybeProp, число в DU | border (`0.25 DU`) | Перевизначення геометрії або типографіки. |
| Сокет | `socket_outline_offset` | MaybeProp, число в DU | border (`0.25 DU`) | Перевизначення геометрії або типографіки. |
| NodeCard, Wire, GraphFrame, Minimap | `border_width` | MaybeProp, число в DU | border (`0.25 DU`) | Перевизначення геометрії або типографіки. |
| NodeCard, GraphFrame | `emphasis_width` | MaybeProp, число в DU | border_emphasis (`0.5 DU`) | Перевизначення геометрії або типографіки. |
| NodeCard | `icon_size` | MaybeProp, число в DU | `icon_small` (`4 DU`) | Перевизначення геометрії або типографіки. |
| NodeCard, NodeRow, GraphFrame | `font_size` | MaybeProp, число в TU | type_body_size | Перевизначення геометрії або типографіки. |
| NodeCard, NodeRow, GraphFrame | `line_height` | MaybeProp, число в TU | type_body_line | Перевизначення геометрії або типографіки. |
| NodeCard, Wire | `caption_font_size` | MaybeProp, число в TU | type_caption_size | Перевизначення геометрії або типографіки. |
| NodeCard, Wire | `caption_line_height` | MaybeProp, число в TU | type_caption_line | Перевизначення геометрії або типографіки. |
| NodeCard | `footer_padding_x` | MaybeProp, число в DU | pad_card | Перевизначення геометрії або типографіки. |
| NodeCard | `footer_padding_y` | MaybeProp, число в DU | pad_tight | Перевизначення геометрії або типографіки. |
| Wire | `wire_width` | MaybeProp, число в DU | border_emphasis (`0.5 DU`) | Перевизначення геометрії або типографіки. |
| Wire | `wire_exec_width` | MaybeProp, число в DU | 0.75 | Перевизначення геометрії або типографіки. |
| Wire | `wire_hit_width` | MaybeProp, число в DU | 3 | Перевизначення геометрії або типографіки. |
| Wire | `wire_dash` | MaybeProp, число в DU | 2 | Перевизначення геометрії або типографіки. |
| Wire | `wire_dash_gap` | MaybeProp, число в DU | 1 | Перевизначення геометрії або типографіки. |
| Wire | `wire_detached_dash` | MaybeProp, число в DU | 1 | Перевизначення геометрії або типографіки. |
| Wire | `wire_label_width` | MaybeProp, число в DU | 100 | Перевизначення геометрії або типографіки. |
| Wire | `wire_label_height` | MaybeProp, число в DU | 6 | Перевизначення геометрії або типографіки. |
| Wire | `wire_label_padding_x` | MaybeProp, число в DU | pad_tight | Перевизначення геометрії або типографіки. |
| Wire | `wire_handle_min` | MaybeProp, число в DU | 10; обчислення маршруту в Rust використовують проп | Перевизначення геометрії або типографіки. |
| GraphFrame | `frame_header_height` | MaybeProp, число в DU | row_compact (`8 DU`) | Перевизначення геометрії або типографіки. |
| GraphFrame | `frame_padding_x` | MaybeProp, число в DU | pad_card | Перевизначення геометрії або типографіки. |
| GraphFrame | `frame_gap` | MaybeProp, число в DU | pad_tight | Перевизначення геометрії або типографіки. |
| Minimap | `minimap_width` | MaybeProp, число в DU | size.0 (`50 DU`) | Перевизначення геометрії або типографіки. |
| Minimap | `minimap_height` | MaybeProp, число в DU | size.1 (`35 DU`) | Перевизначення геометрії або типографіки. |
| Minimap | `minimap_offset_x` | MaybeProp, число в DU | margin_screen | Перевизначення геометрії або типографіки. |
| Minimap | `minimap_offset_y` | MaybeProp, число в DU | margin_screen | Перевизначення геометрії або типографіки. |
| Minimap | `minimap_item_min_size` | MaybeProp, число в DU | 0.25 | Перевизначення геометрії або типографіки. |

## Кастомізація

`class` і `style` застосовуються до кореня кожного компонента, зокрема кореневого SVG-елемента WireLayer та SVG-групи Wire.
Кожен метричний проп записує відповідну змінну `--op-graph-*`; нащадки успадковують спільні метрики, якщо не задають власних пропсів.
Прямокутна геометрія, поведінка станів і наявні типові розміри залишаються типовими.
Пропси `color` і `MiniItem.color` залишаються доступними для кольорів типів застосунку й записують змінні кольорів сімейства.

| Метричний проп | Компонент | Одиниця | Змінна | Типове значення |
| :--- | :--- | :--- | :--- | :--- |
| `grid` | GraphCanvas | DU | `--op-graph-grid` | 5 |
| `grid_width` | GraphCanvas | DU | `--op-graph-grid-width` | border (`0.25 DU`) |
| `layer_size` | WireLayer | DU | `--op-graph-layer-size` | 0.25 |
| `header_height` | NodeCard | DU | `--op-graph-header-height` | row_compact (`8 DU`) |
| `header_padding_start` | NodeCard | DU | `--op-graph-header-padding-start` | pad_card |
| `header_padding_end` | NodeCard | DU | `--op-graph-header-padding-end` | pad_tight |
| `header_gap` | NodeCard | DU | `--op-graph-header-gap` | pad_tight |
| `body_padding_y` | NodeCard | DU | `--op-graph-body-padding-y` | gap_micro |
| `label_width` | NodeCard | DU | `--op-graph-label-width` | 40% картки |
| `row_height` | NodeCard, NodeRow | DU | `--op-graph-row-height` | row_standard (`10 DU`) |
| `row_padding_x` | NodeCard, NodeRow | DU | `--op-graph-row-padding-x` | pad_card |
| `row_padding_y` | NodeCard, NodeRow | DU | `--op-graph-row-padding-y` | gap_micro; рядки вмісту |
| `row_gap` | NodeCard, NodeRow | DU | `--op-graph-row-gap` | pad_tight |
| `socket_size` | NodeCard, Socket | DU | `--op-graph-socket-size` | 2.5 |
| `socket_border_width` | Socket | DU | `--op-graph-socket-border-width` | border_emphasis (`0.5 DU`) |
| `socket_outline_width` | Socket | DU | `--op-graph-socket-outline-width` | border (`0.25 DU`) |
| `socket_outline_offset` | Socket | DU | `--op-graph-socket-outline-offset` | border (`0.25 DU`) |
| `border_width` | NodeCard, Wire, GraphFrame, Minimap | DU | `--op-graph-border-width` | border (`0.25 DU`) |
| `emphasis_width` | NodeCard, GraphFrame | DU | `--op-graph-emphasis-width` | border_emphasis (`0.5 DU`) |
| `icon_size` | NodeCard | DU | `--op-graph-icon-size` | `icon_small` (`4 DU`) |
| `font_size` | NodeCard, NodeRow, GraphFrame | TU | `--op-graph-font-size` | type_body_size |
| `line_height` | NodeCard, NodeRow, GraphFrame | TU | `--op-graph-line-height` | type_body_line |
| `caption_font_size` | NodeCard, Wire | TU | `--op-graph-caption-font-size` | type_caption_size |
| `caption_line_height` | NodeCard, Wire | TU | `--op-graph-caption-line-height` | type_caption_line |
| `footer_padding_x` | NodeCard | DU | `--op-graph-footer-padding-x` | pad_card |
| `footer_padding_y` | NodeCard | DU | `--op-graph-footer-padding-y` | pad_tight |
| `wire_width` | Wire | DU | `--op-graph-wire-width` | border_emphasis (`0.5 DU`) |
| `wire_exec_width` | Wire | DU | `--op-graph-wire-exec-width` | 0.75 |
| `wire_hit_width` | Wire | DU | `--op-graph-wire-hit-width` | 3 |
| `wire_dash` | Wire | DU | `--op-graph-wire-dash` | 2 |
| `wire_dash_gap` | Wire | DU | `--op-graph-wire-dash-gap` | 1 |
| `wire_detached_dash` | Wire | DU | `--op-graph-wire-detached-dash` | 1 |
| `wire_label_width` | Wire | DU | `--op-graph-wire-label-width` | 100 |
| `wire_label_height` | Wire | DU | `--op-graph-wire-label-height` | 6 |
| `wire_label_padding_x` | Wire | DU | `--op-graph-wire-label-padding-x` | pad_tight |
| `wire_handle_min` | Wire | DU | `--op-graph-wire-handle-min` | 10; обчислення маршруту в Rust використовують проп |
| `frame_header_height` | GraphFrame | DU | `--op-graph-frame-header-height` | row_compact (`8 DU`) |
| `frame_padding_x` | GraphFrame | DU | `--op-graph-frame-padding-x` | pad_card |
| `frame_gap` | GraphFrame | DU | `--op-graph-frame-gap` | pad_tight |
| `minimap_width` | Minimap | DU | `--op-graph-minimap-width` | size.0 (`50 DU`) |
| `minimap_height` | Minimap | DU | `--op-graph-minimap-height` | size.1 (`35 DU`) |
| `minimap_offset_x` | Minimap | DU | `--op-graph-minimap-offset-x` | margin_screen |
| `minimap_offset_y` | Minimap | DU | `--op-graph-minimap-offset-y` | margin_screen |
| `minimap_item_min_size` | Minimap | DU | `--op-graph-minimap-item-min-size` | 0.25 |

Компонування Socket використовує ту саму змінну `row_height` для розміру рядка та розташування сокетів.
NodeCard спостерігає за відрендереними карткою, заголовком, тілом, рядками та сокетами й вимірює центри сокетів після зміни слотів або атрибутів; результат не залежить від масштабу камери.
Застосункам слід використовувати `on_sockets` для кінцевих точок з'єднань, а не відтворювати обчислення заголовка та рядків.
Коли геометрія компонування також використовується в Rust, пропси передають значення і змінним, і обчисленням.
Minimap додатково вимірює відрендерений розмір і контрольний елемент мінімального розміру, тому успадковані змінні та явні стилі також узгоджують область перегляду й перетворення координат вказівника.
Розміри підпису Wire застосовуються до SVG-прямокутника та його центрованої геометрії CSS.
`wire_handle_min` керує обчисленням кривої через проп; змінюйте це значення пропом, оскільки сама змінна не змінює маршрут у Rust.
Обмеження стовпця тіла типово є пропорційним `fit-content(40%)`; `label_width`, якщо встановлено, задає обмеження в DU.
Повнорозмірні шари залишаються відносними до свого контейнера.

| Змінна кольору | Типове значення |
| :--- | :--- |
| `--op-graph-background` | background |
| `--op-graph-divider` | divider |
| `--op-graph-surface` | surface |
| `--op-graph-header-background` | surface_alt |
| `--op-graph-text` | text_primary |
| `--op-graph-secondary-text` | text_secondary |
| `--op-graph-border` | border |
| `--op-graph-strong-border` | border_strong |
| `--op-graph-accent` | accent |
| `--op-graph-error` | error |
| `--op-graph-node-color` | прозорий акцент заголовка; secondary_text для категорії |
| `--op-graph-socket-color` | secondary_text |
| `--op-graph-wire-color` | secondary_text для штриха; strong_border для підпису |
| `--op-graph-frame-color` | accent; заповнення та заголовок використовують тонування 8% і 25% |
| `--op-graph-minimap-item` | text_disabled |

| Слот | Компонент | Розташування |
| :--- | :--- | :--- |
| `empty` | GraphCanvas | Вміст в екранних координатах, поки `is_empty` має значення true. |
| `header` | NodeCard | Замінює заголовок і категорію; довідка та дії залишаються доступними. |
| `body` | NodeCard | Замінює дочірній вміст власними рядками тіла у спільній сітці. |
| `footer` | NodeCard | Під тілом. |
| `input, output` | NodeRow | Вміст сокетів на кожному краї. |
| `title_slot` | GraphFrame | Замінює заголовок рамки. |

### Рядки

| Ключ | Проп | Типове значення |
| :--- | :--- | :--- |
| `graph.node-help` | `help_label` | About this node |
| `graph.minimap` | `label` | Minimap |

### Піктограми

| Ключ | Проп | Типове значення |
| :--- | :--- | :--- |
| `graph.node-help` | `help_icon` | `info` |
