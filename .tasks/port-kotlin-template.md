Task: port components from the Kotlin/Compose app to the Operator UI web library (Leptos), as Draft.

Source (read-only): [Kotlin components](../../notifiApp/NotifiApp/Android/app/src/main/kotlin/com/pavver/notifiapp/ui/components/).
The app is close to the maintainer's ideal in BEHAVIOUR and STRUCTURE (message types, interactions, gestures, states, keyboard/pointer handling, layout ideas).
It uses an OUTDATED version of the design system (old neumorphic colors, shadows, rounded corners, its DESIGN_SYSTEM.md is obsolete): do NOT copy its colors, shadows, radii or dimensions.
Use the CURRENT Operator tokens and rules instead: square corners (0 DU unless the spec lists a round exception), DU/TU only, tokens from tokens/tokens.toml via the existing CSS variables, Operator states and motion, Material Symbols from operator_ui_tokens::icons (do not add icons; reuse the closest one and list missing ones in the summary).
Components must not get their own background or rectangular frame unless the control needs it; pending/result feedback is drawn on the element itself.
Telegram-specific code becomes generic: no Telegram names, network fetches or API assumptions in the library; data (emoji sets, stickers, GIFs, media URLs) is passed by the application.
The demo never touches the network: use data: URLs, inline SVG and generated placeholder media.

Then follow [the new component template](new-components-template.md) for every component (implementation, English and Ukrainian specs, change files, demo page, verification).
Each commit that changes a component adds one `docs/changes/<component>/<YYYY-MM-DD>-<HHMM>-<short-kebab-slug>.toml` file, starting with `status = "draft"`.
Include an RFC 3339 `date` with offset and a `kind` (`new`, `feature`, `fix`, `breaking`, `docs`, or `refactor`).
Both `[en]` and `[uk]` tables require Markdown `summary` texts and should include `check` instructions identifying the examples, states, or props to inspect.
Every `breaking` change requires `migration` instructions in both languages explaining what callers must change.
The optional `commit` accepts 7–40 hexadecimal characters; otherwise the documentation build may derive the adding commit from complete Git history.
The maintainer approves, replaces, or deletes each file during review.
Desktop and phone: every component works with mouse+keyboard on desktop and touch on phones (pointer events, no hover-only actions); where the Kotlin version is phone-only (for example a keyboard panel replacing the IME), provide a desktop variant too (for example an anchored popover).
Do not commit.
End with a short summary per component and a list of Kotlin features you left out.

Media resources (maintainer, 2026-11-21): do NOT port the emoji/sticker/GIF assets themselves (the Kotlin app uses Telegram resources).
Port the MECHANISM: a pluggable resource model where the application installs any packs (Telegram ones included) through providers/data passed in.
Support raster (PNG/WebP) and vector (SVG; Lottie-style animated vector through an app-supplied renderer slot) stickers and icons, and custom emoji that can replace Unicode emoji.
Animated "GIFs" are short muted looping MP4/WebM videos (like Telegram), rendered with <video autoplay muted loop playsinline>, paused when off-screen; real .gif decoding is not the primary path (it is slow).
The library ships no packs; the demo uses a tiny generated pack (inline SVG/data URLs) to show the mechanism.
