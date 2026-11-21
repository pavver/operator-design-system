# Токени та генерація коду

[tokens/tokens.toml](../tokens/tokens.toml) є єдиним джерелом правди для одиниць, шрифту, просторових токенів, стилів шрифту, руху та палітр кольорів.
[tokens/icons](../tokens/icons) містить піктограми у вигляді звичайних файлів SVG.

Компоненти пишуться вручну один раз для кожної платформи.
Генеруються лише токени, оскільки поведінка компонентів залежить від моделі подій та анімацій кожного набору інструментів.

## Запуск генератора

З кореня репозиторію:

```bash
cargo run -p operator-tokens            # write outputs
cargo run -p operator-tokens -- --check # fail when outputs are stale
```

| Вивід | Споживач |
| :--- | :--- |
| [web/operator-ui/style/tokens.css](../web/operator-ui/style/tokens.css) | Будь-який вебстек через користувацькі властивості CSS. |
| [crates/operator-ui-tokens/src/lib.rs](../crates/operator-ui-tokens/src/lib.rs) | Набори інструментів Rust (Leptos, egui); `no_std`. |
| [generated/kotlin/OperatorTokens.kt](../generated/kotlin/OperatorTokens.kt) | Jetpack Compose. |

Згенеровані файли ніколи не повинні редагуватися вручну.
CI запускає режим `--check`.

## Формат файлу

| Таблиця | Одиниця | Вміст |
| :--- | :--- | :--- |
| `[units]` | points | `du_points` та `tu_points`. |
| `[font]` | — | Гарнітура та список запасних варіантів. |
| `[dimensions]` | DU | Просторові токени. |
| `[typography.<style>]` | TU | `size`, `line`, `weight`. |
| `[motion]` | ms | Тривалості. |
| `[themes.<theme>.<mode>]` | `#RRGGBB` або `#RRGGBBAA` | Одна палітра на тему й режим. |

Кожна палітра повинна визначати однакові ключі.
Генератор відхиляє палітру з відсутнім або зайвим ключем.

## Іменування у виводах

| Тип | CSS | Rust | Kotlin |
| :--- | :--- | :--- | :--- |
| Розмірність | `--op-dim-row-standard` | `dim::ROW_STANDARD` | `OperatorDu.RowStandard` |
| Стиль шрифту | `--op-type-body-size` | `typography::BODY` | `OperatorType.Body` |
| Рух | `--op-motion-state-color` | `motion::STATE_COLOR_MS` | `OperatorMotion.StateColor` |
| Колір | `--op-accent` | `Palette::accent` | `OperatorPalette.accent` |

## Піктограми

Піктограми — це Material Symbols Sharp, оголошені в [tokens/icons.toml](../tokens/icons.toml) та імпортовані в [tokens/icons](../tokens/icons).
Див. [foundations/iconography.md](foundations/iconography.md) щодо правил і кроків імпорту.
Генератор вбудовує кожну піктограму у вивід Rust як `icons::<NAME>`.
