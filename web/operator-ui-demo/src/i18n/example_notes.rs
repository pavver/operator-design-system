//! Ukrainian explanatory notes for live examples.

pub fn ukrainian(key: &str) -> Option<&'static str> {
    Some(match key {
        "Only one section is open at a time." => "Одночасно відкрито лише один розділ.",
        "Shared metrics, header content and application expansion strings." => {
            "Спільні розміри, вміст заголовка та власні написи для розгортання."
        }
        "Metric props and slots customize this instance; its translated string follows the language selector." => {
            "Параметри розмірів і слоти налаштовують цей екземпляр; переклад напису відповідає вибраній мові."
        }
        "Async action with a pending running border, a 500 ms result icon flash and a 300 ms tail. Clicks are blocked while pending." => {
            "Асинхронна дія з рухомою рамкою під час очікування, спалахом піктограми результату на 500 мс і завершальною анімацією на 300 мс. Під час очікування натискання заблоковано."
        }
        "With href the button renders a real link: crawlers follow it, and middle-click, Ctrl+click, and the context menu work. Only Idle, Pressed, and Disabled apply." => {
            "З href кнопка стає справжнім посиланням: пошукові роботи переходять за ним, працюють середня кнопка миші, Ctrl+клацання та контекстне меню. Доступні лише стани Idle, Pressed і Disabled."
        }
        "Maintains standard widescreen proportions." => {
            "Зберігає стандартні широкоформатні пропорції."
        }
        "Square aspect ratio." => "Квадратне співвідношення сторін.",
        "Color is derived from the id with the same hash on every platform." => {
            "Колір визначається за ідентифікатором з однаковим хешуванням на всіх платформах."
        }
        "Custom badge slot, metric props and aria-label." => {
            "Власний слот позначки, параметри розмірів і aria-label."
        }
        "Path with default separators." => "Шлях зі стандартними роздільниками.",
        "Collapses middle items into a menu when exceeding max_items." => {
            "Згортає середні елементи в меню, коли їхня кількість перевищує max_items."
        }
        "Uses a chevron icon separator." => "Використовує піктограму шеврона як роздільник.",
        "Adjacent buttons share borders. While one action runs, the others wait." => {
            "Сусідні кнопки мають спільні рамки. Поки виконується одна дія, решта чекають."
        }
        "Joined icon actions with their own pending and result feedback." => {
            "З’єднані кнопки дій із піктограмами з власним очікуванням і результатом."
        }
        "One enabled prop turns every button of the group on or off." => {
            "Один проп enabled вмикає або вимикає всі кнопки групи."
        }
        "With exclusive off, every button runs independently." => {
            "Коли exclusive вимкнено, кожна кнопка виконує дію незалежно."
        }
        "Stretches buttons to fill available width." => "Розтягує кнопки на всю доступну ширину.",
        "Vertical stack sharing horizontal borders." => {
            "Вертикальний стос зі спільними горизонтальними рамками."
        }
        "Consecutive message grouping, sender roles, timestamps, and delivery verification." => {
            "Групування послідовних повідомлень, ролі відправників, позначки часу та підтвердження доставки."
        }
        "Voice waveforms, code blocks, blockquotes, media grids with spoilers, polls, and cards." => {
            "Звукові хвилі, блоки коду, цитати, сітки медіа зі спойлерами, опитування та картки."
        }
        "Reply quote, forward attribution, self-destruct countdown, and bot keyboard." => {
            "Цитата відповіді, джерело пересланого повідомлення, відлік до самознищення та клавіатура бота."
        }
        "Pure 1-3 emojis render big without a bubble; circular video note and standalone stickers." => {
            "Повідомлення лише з 1–3 емодзі відображаються великими без бульбашки; кругле відеоповідомлення та окремі наліпки."
        }
        "Tactical delivery checkmarks, Stop sign failure, unread counts, and syncing indicators." => {
            "Чіткі позначки доставки, помилка зі знаком «Стоп», лічильники непрочитаного та індикатори синхронізації."
        }
        "Each tile runs its own request; several can be pending at once." => {
            "Кожна плитка виконує власний запит; кілька запитів можуть очікувати одночасно."
        }
        "User tags with their own colors; the text stays readable in both modes." => {
            "Користувацькі мітки з власними кольорами; текст залишається читабельним в обох режимах."
        }
        "Shows percentage completion or numeric value inside the ring." => {
            "Показує відсоток виконання або числове значення всередині кільця."
        }
        "Spinning mode with success and error indicators." => {
            "Режим обертання з індикаторами успіху та помилки."
        }
        "Custom dimensions in Design Units with scaling center label." => {
            "Власні розміри в одиницях дизайну з масштабуванням напису в центрі."
        }
        "An unknown language is shown without highlighting." => {
            "Невідома мова відображається без підсвічування синтаксису."
        }
        "Code metrics, a header slot and an application copy string." => {
            "Розміри коду, слот заголовка та власний напис для копіювання."
        }
        "Type {{ for variables or | for filters; Ctrl+Space asks again. Unknown filters are underlined; Ctrl+Enter or leaving commits." => {
            "Введіть {{ для змінних або | для фільтрів; Ctrl+Space повторно відкриває підказки. Невідомі фільтри підкреслено; Ctrl+Enter або вихід із поля зберігає зміни."
        }
        "Custom typography, gutter, completion rows and diagnostic help; type {{ to open completion." => {
            "Власна типографіка, поле номерів рядків, рядки доповнення та діагностичні підказки; введіть {{, щоб відкрити доповнення."
        }
        "Type to filter, use the arrow keys and Enter. Unknown text reverts on blur." => {
            "Вводьте текст для фільтрування, використовуйте стрілки та Enter. Невідомий текст скидається після втрати фокуса."
        }
        "20 000 options; the list renders only the visible window." => {
            "20 000 варіантів; список відображає лише видиму частину."
        }
        "Sections, icons and details; the field shows the label, the value is the entity id." => {
            "Розділи, піктограми та деталі; поле показує підпис, а значенням є ідентифікатор сутності."
        }
        "Right-click the area: ComboboxPopup opens at the pointer with the search focused (quick add, wire menus)." => {
            "Клацніть область правою кнопкою: ComboboxPopup відкриється біля вказівника з фокусом у пошуку — для швидкого додавання та меню з’єднань."
        }
        "Reload asks the simulated backend. Success shows the list, or EmptyState when Empty is selected; an error shows the compact error state with Retry. Answers under 300 ms show no skeleton." => {
            "Кнопка перезавантаження надсилає запит до симулятора сервера. Успіх показує список або EmptyState, якщо вибрано порожній результат; помилка показує компактний стан помилки з повторною спробою. Якщо відповідь надходить швидше ніж за 300 мс, каркас не з’являється."
        }
        "Confirm runs async: success closes after 500 ms, error pulses and unlocks the dialog." => {
            "Підтвердження виконується асинхронно: успіх закриває вікно через 500 мс, помилка викликає пульсацію та розблоковує вікно."
        }
        "Every simulated request on these pages is logged here. It auto-scrolls to the newest line." => {
            "Тут записується кожен змодельований запит на цих сторінках. Журнал автоматично прокручується до найновішого рядка."
        }
        "Colors and attributes come from SGR sequences; a carriage return rewrites the line." => {
            "Кольори й атрибути задаються послідовностями SGR; повернення каретки перезаписує рядок."
        }
        "Log metrics, a header slot and an application accessible name." => {
            "Розміри журналу, слот заголовка та власна назва для допоміжних технологій."
        }
        "Click the inbox to mark everything read." => {
            "Натисніть вхідні, щоб позначити все прочитаним."
        }
        "Metric props (min_width, height, font_size) and custom label." => {
            "Параметри розмірів (min_width, height, font_size) і власний підпис."
        }
        "Click a header to sort ascending, descending, then back to the original order." => {
            "Натискайте заголовок, щоб сортувати за зростанням, за спаданням, а потім повернути початковий порядок."
        }
        "Scoped header and body cells, custom metrics and application sort strings." => {
            "Слоти комірок заголовка й тіла з доступом до даних, власні розміри та написи для сортування."
        }
        "Save runs inside the dialog: success closes it, error keeps it open. Width follows clamp(75 DU, 90% of the viewport, 140 DU)." => {
            "Збереження виконується у вікні: успіх закриває його, помилка залишає відкритим. Ширина визначається як clamp(75 DU, 90% області перегляду, 140 DU)."
        }
        "Metric props, title and action slots." => "Параметри розмірів, слоти заголовка та дій.",
        "Field owns the shell; the caller owns the control and its state." => {
            "Field відповідає за оболонку; викликальний код — за елемент керування та його стан."
        }
        "Switch the mode in the top bar to see the other palette." => {
            "Перемкніть режим у верхній панелі, щоб побачити іншу палітру."
        }
        "1 DU = 4 logical pixels on the web." => "1 DU = 4 логічні пікселі у вебі.",
        "Every icon in tokens/icons, rendered at 6 DU." => {
            "Усі піктограми з tokens/icons, відображені в розмірі 6 DU."
        }
        "Press Play to move each marker with its duration and the standard curve." => {
            "Натисніть відтворення, щоб перемістити кожен маркер із відповідною тривалістю та стандартною кривою."
        }
        "The same visual language applies to every control." => {
            "Єдина візуальна мова застосовується до кожного елемента керування."
        }
        "Drag headers to move nodes, the background or the middle button to pan, the wheel to zoom. Hover the info icon for node help." => {
            "Перетягуйте заголовки, щоб рухати вузли, тло або середню кнопку миші — щоб рухати полотно; коліщатко змінює масштаб. Наведіть вказівник на піктограму інформації, щоб побачити довідку про вузол."
        }
        "Change the row height to check that the measured sockets and wires stay aligned." => {
            "Змініть висоту рядка, щоб перевірити узгоджене розташування виміряних роз’ємів і з’єднань."
        }
        "Custom metrics, colors via CSS variables, translated strings and replaceable icons." => {
            "Власні розміри, кольори через змінні CSS, перекладені написи та замінювані піктограми."
        }
        "Hover the icon, or tap it on touch. The card stays open while the pointer is over it." => {
            "Наведіть вказівник на піктограму або торкніться її на сенсорному екрані. Картка залишається відкритою, поки вказівник над нею."
        }
        "With href the button renders a real link styled as an icon button." => {
            "З href кнопка стає справжнім посиланням, оформленим як кнопка з піктограмою."
        }
        "Maintains 16:9 proportion and renders caption overlay." => {
            "Зберігає пропорції 16:9 і відображає підпис поверх зображення."
        }
        "Shows error icon and message when download fails without touching the network." => {
            "Показує піктограму й повідомлення про помилку завантаження без звернення до мережі."
        }
        "Modifier and editing keys are wider; arrow names print as arrows." => {
            "Клавіші-модифікатори та клавіші редагування ширші; назви стрілок відображаються як стрілки."
        }
        "Metric props and custom separator slot." => {
            "Параметри розмірів і власний слот роздільника."
        }
        "Full built-in language list with flags and standard BCP 47 codes. Type to filter by native name, English name, or language code." => {
            "Повний вбудований список мов із прапорами та стандартними кодами BCP 47. Вводьте текст для пошуку за рідною чи англійською назвою або кодом мови."
        }
        "Apps can supply a custom subset of languages or completely custom LanguageOption items." => {
            "Застосунки можуть надати власну підмножину мов або повністю власні елементи LanguageOption."
        }
        "Setting show_flags=false hides flags in the field and the suggestion list." => {
            "Параметр show_flags=false приховує прапори в полі та списку підказок."
        }
        "Custom flag dimensions (--op-language-picker-flag-width/height), field metrics, and custom label format." => {
            "Власні розміри прапорів (--op-language-picker-flag-width/height), розміри поля та формат підпису."
        }
        "From 5 DU the value moves inside the bar; the text keeps contrast over both parts." => {
            "За товщини від 5 DU значення переходить усередину смуги; текст зберігає контраст на обох частинах."
        }
        "Custom thickness, gap, and value slot." => "Власні товщина, проміжок і слот значення.",
        "Metric props, a content slot and application strings." => {
            "Параметри розмірів, слот вмісту та власні написи застосунку."
        }
        "Thin top indicator showing background page activity." => {
            "Тонкий індикатор угорі показує фонову активність сторінки."
        }
        "Blocks inputs while operation is pending." => "Блокує введення на час виконання операції.",
        "Overflowing text waits 2 s, scrolls at 7.5 DU/s and pauses 1.2 s between loops. Hover or long-press shows the full text." => {
            "Текст, що не вміщується, чекає 2 с, прокручується зі швидкістю 7,5 DU/с і робить паузу 1,2 с між циклами. Наведення або тривале натискання показує повний текст."
        }
        "Text and tooltip metrics, a tooltip slot and application text." => {
            "Розміри тексту й підказки, слот підказки та власний текст застосунку."
        }
        "Opens under the button, aligned to its end edge. Arrow keys move, Escape closes." => {
            "Відкривається під кнопкою з вирівнюванням за її кінцевим краєм. Стрілки переміщують фокус, Escape закриває меню."
        }
        "Right-click inside the area." => "Клацніть правою кнопкою всередині області.",
        "Metric and scoped content overrides apply to submenus too; the header and footer frame only the root menu." => {
            "Перевизначення розмірів і слотів вмісту з доступом до даних застосовуються також до підменю; заголовок і нижній колонтитул обрамлюють лише кореневе меню."
        }
        "2 000 tree rows, virtualized. Arrows navigate the tree; the chevron collapses the panel." => {
            "2 000 віртуалізованих рядків дерева. Стрілки переміщують фокус деревом; шеврон згортає панель."
        }
        "Metric props, header/footer slots and custom labels customize NavigationRail and SidePanel." => {
            "Параметри розмірів, слоти заголовка й підвала та власні підписи налаштовують NavigationRail і SidePanel."
        }
        "Tree row metrics, a scoped node label and an application accessible name." => {
            "Розміри рядків дерева, слот підпису з даними вузла та власна назва для допоміжних технологій."
        }
        "Click the current page, type a number, and press Enter; values past the end go to the last page. Escape restores the page." => {
            "Натисніть поточну сторінку, введіть номер і натисніть Enter; завеликі значення ведуть на останню сторінку. Escape відновлює попередню сторінку."
        }
        "Custom metrics, before/after slots, and custom labels." => {
            "Власні розміри, слоти before/after і власні підписи."
        }
        "The box clips and transforms its content; the layers still open fully on top." => {
            "Блок обрізає та перетворює свій вміст; накладні шари все одно повністю відкриваються поверх нього."
        }
        "The layer exposes offsets, slots, and a reactive accessible name." => {
            "Шар надає зміщення, слоти та реактивну назву для допоміжних технологій."
        }
        "Narrow containers stack the label above the value." => {
            "У вузьких контейнерах підпис розташовується над значенням."
        }
        "A scoped value slot retains the original value for copying." => {
            "Слот значення з доступом до даних зберігає початкове значення для копіювання."
        }
        "The mark moves only after the request succeeds; the target shows a spinner while pending." => {
            "Позначка переміщується лише після успішного запиту; під час очікування цільовий варіант показує індикатор завантаження."
        }
        "Inline control on page surface. Pointer hover previews the score; async save shows a spinner and result flash on the chosen star, with rollback on error." => {
            "Вбудований елемент на поверхні сторінки. Наведення показує попередню оцінку; асинхронне збереження показує індикатор очікування та спалах результату на вибраній зірці, а в разі помилки відновлює попереднє значення."
        }
        "Pending spinner and result flash appear on the chosen star; focus ring highlights the focused star only." => {
            "Індикатор очікування та спалах результату з’являються на вибраній зірці; рамка фокуса виділяє лише зірку у фокусі."
        }
        "Scroll to an edge and keep pulling with touch or the mouse wheel: the content stretches and springs back." => {
            "Прокрутіть до краю та продовжуйте тягнути дотиком або коліщатком миші: вміст розтягується й пружно повертається назад."
        }
        "Custom metrics, header/footer slots, and custom label." => {
            "Власні розміри, слоти заголовка й підвала та власний підпис."
        }
        "Hover the right edge to widen the bar; drag the thumb or press the track to page." => {
            "Наведіть вказівник на правий край, щоб розширити смугу; перетягуйте повзунок або натискайте доріжку для посторінкового прокручування."
        }
        "Each bar stops short of the shared corner." => {
            "Кожна смуга не доходить до спільного кута."
        }
        "Custom track size, thumb thickness, and header/footer slots." => {
            "Власні розмір доріжки, товщина повзунка та слоти заголовка й підвала."
        }
        "The query is reported 300 ms after the last key press; the spinner shows while results load. Escape clears." => {
            "Запит надсилається через 300 мс після останнього натискання клавіші; під час завантаження результатів відображається індикатор. Escape очищає поле."
        }
        "The async variant shows a spinner in the target segment and slides back on error." => {
            "Асинхронний варіант показує індикатор очікування в цільовому сегменті та повертається назад у разі помилки."
        }
        "Escape, the scrim, or the close button dismiss the sheet." => {
            "Escape, натискання на затемнення або кнопка закриття закривають панель."
        }
        "The sheet body scrolls; the page behind it stays still, also when the wheel is over the scrim." => {
            "Вміст панелі прокручується; сторінка позаду залишається нерухомою, навіть коли коліщатко миші працює над затемненням."
        }
        "Custom width, height, gap and label." => "Власні ширина, висота, проміжок і підпис.",
        "Drag or click the track; arrow keys work when focused. The commit after release runs async; on error the value rolls back." => {
            "Перетягуйте повзунок або натискайте доріжку; у фокусі працюють стрілки. Збереження після відпускання виконується асинхронно; у разі помилки значення відновлюється."
        }
        "Pushes buttons to opposite ends of a container." => {
            "Розсуває кнопки до протилежних країв контейнера."
        }
        "Reserves minimum space before expanding." => {
            "Резервує мінімальний простір перед розширенням."
        }
        "Drag the divider, use the arrow keys when it is focused, or double-click to reset." => {
            "Перетягуйте роздільник, використовуйте стрілки, коли він у фокусі, або двічі клацніть для скидання."
        }
        "Custom handle size, custom label, and custom grip slot." => {
            "Власні розмір ручки, підпис і слот захвату."
        }
        "Metric props and caption slot." => "Параметри розмірів і слот підпису.",
        "Stays interactive while pending for continuous input." => {
            "Залишається інтерактивним під час очікування для безперервного введення."
        }
        "The value commits only after success; the thumb parks in the center with a spinner while pending." => {
            "Значення зберігається лише після успіху; під час очікування повзунок зупиняється в центрі з індикатором завантаження."
        }
        "The variant used on the Android main screen: the active tab opens into the content through the accent baseline." => {
            "Варіант із головного екрана Android: активна вкладка з’єднується з вмістом через акцентну базову лінію."
        }
        "Custom metrics, before/after slots, and custom label." => {
            "Власні розміри, слоти before/after і власний підпис."
        }
        "Grows from 3 to 8 lines, then scrolls." => {
            "Зростає від 3 до 8 рядків, а потім прокручується."
        }
        "Edit the first value to reveal the async SAVE button. It hides after a successful save." => {
            "Змініть перше значення, щоб показати кнопку асинхронного збереження. Вона зникає після успішного збереження."
        }
        "The switch in the top bar of this site is the same component." => {
            "Перемикач у верхній панелі цього сайту — той самий компонент."
        }
        "Milestones with tones and icons." => "Етапи з тональностями та піктограмами.",
        "Reduced vertical spacing." => "Зменшені вертикальні проміжки.",
        "Set on ToastHost or at runtime with set_position. Top stacks put the newest toast nearest the edge." => {
            "Задається в ToastHost або під час роботи через set_position. У верхніх стосах найновіше сповіщення розміщується найближче до краю."
        }
        "Checks how wrapped text, the icon and the buttons behave in a tall toast." => {
            "Демонструє поведінку тексту з перенесенням, піктограми та кнопок у високому сповіщенні."
        }
        "The control shows its own result; the toast carries the error message the control cannot show." => {
            "Елемент керування показує власний результат; сповіщення містить повідомлення про помилку, яке елемент не може показати."
        }
        "Arrow keys move focus between the controls. Drag the bottom-right corner to narrow the bar: commands that do not fit move into the overflow menu." => {
            "Стрілки переміщують фокус між елементами керування. Потягніть нижній правий кут, щоб звузити панель: команди, що не вміщуються, переходять у меню додаткових дій."
        }
        "Custom metrics, start/end slots, and translated strings." => {
            "Власні розміри, слоти start/end і перекладені написи."
        }
        "Hover for 500 ms, focus with Tab, or long-press on touch." => {
            "Наведіть вказівник на 500 мс, перейдіть у фокус клавішею Tab або натисніть і утримуйте на сенсорному екрані."
        }
        "Arrow Up/Down steps; invalid or out-of-range text rolls back with the error pulse." => {
            "Стрілки вгору/вниз змінюють значення кроками; некоректний текст або значення поза діапазоном скидається з пульсацією помилки."
        }
        "Durations accept 1h 30m, 90s, 1:30 and a sign; times accept 07:30 or 0730." => {
            "Тривалість приймає 1h 30m, 90s, 1:30 і знак; час приймає 07:30 або 0730."
        }
        "Spectrum, RGB, HSL and temperature editors; the code updates live while dragging. Opacity adds an alpha byte." => {
            "Редактори спектра, RGB, HSL і температури; код оновлюється наживо під час перетягування. Прозорість додає байт альфа-каналу."
        }
        "The arrow opens a dial: drag the hand to the hour, then to the minute. The inner ring holds 00 and 13–23." => {
            "Стрілка відкриває циферблат: перетягніть стрілку на годину, потім на хвилину. Внутрішнє коло містить 00 і 13–23."
        }
        "Press the first day, move to preview, press the last day; or pick a quick range. Text such as 12 Jul – 5 Nov or 2026-07-12..2026-11-05 works too." => {
            "Натисніть перший день, ведіть для попереднього перегляду, натисніть останній; або виберіть швидкий діапазон. Текст на кшталт 12 лип – 5 лис чи 2026-07-12..2026-11-05 теж працює."
        }
        "The arrow opens one column per unit; the buttons carry minutes into hours, and a signed field adds a sign switch." => {
            "Стрілка відкриває колонку для кожної одиниці; кнопки переносять хвилини в години, а поле зі знаком додає перемикач знака."
        }
        "The − and + buttons step within min and max and turn off at the bounds." => {
            "Кнопки − і + змінюють значення в межах min і max та вимикаються на межах."
        }
        "The dial alone, for panels and dialogs." => "Лише циферблат — для панелей і діалогів.",
        "The editor alone, for panels and dialogs." => "Лише редактор — для панелей і діалогів.",
        "The arrow opens a calendar or the swatches; text input works too." => {
            "Стрілка відкриває календар або зразки кольорів; текстове введення також працює."
        }
        "Maintains 16:9 widescreen ratio with visible offline placeholder." => {
            "Зберігає широкоформатні пропорції 16:9 із видимою заглушкою недоступного відео."
        }
        "Only the visible rows and a few above and below exist in the DOM." => {
            "У DOM є лише видимі рядки та кілька рядків вище й нижче."
        }
        "Metric props and header/footer slots customize VirtualList." => {
            "Параметри розмірів і слоти заголовка й підвала налаштовують VirtualList."
        }
        _ => return None,
    })
}
