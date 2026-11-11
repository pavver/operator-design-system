//! xterm text parser: turns log text with ANSI escape sequences into styled lines.
//!
//! Only SGR (`ESC [ … m`) changes the style; every other CSI, OSC, and two-byte escape is dropped.
//! The style carries over from line to line, like in a terminal.

use crate::core::styled_text::{SpanStyle, StyledLine, push_text};

const ESC: char = '\u{1b}';
const BEL: char = '\u{7}';

/// Names of the 16 base colors in the `term_*` tokens.
const ANSI_NAMES: [&str; 8] = [
    "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
];

/// A terminal color before it is resolved to CSS.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Color {
    /// Palette index 0..=255; 0..=15 map to the `term_*` tokens.
    Indexed(u8),
    Rgb(u8, u8, u8),
}

impl Color {
    fn css(self) -> String {
        match self {
            Color::Indexed(i) if i < 16 => {
                let bright = if i >= 8 { "bright-" } else { "" };
                format!("var(--op-term-{bright}{})", ANSI_NAMES[usize::from(i % 8)])
            }
            Color::Indexed(i) => {
                let (r, g, b) = xterm_rgb(i);
                format!("rgb({r} {g} {b})")
            }
            Color::Rgb(r, g, b) => format!("rgb({r} {g} {b})"),
        }
    }
}

/// RGB of an xterm 256-color index from 16 upward: a 6×6×6 cube, then 24 grays.
fn xterm_rgb(i: u8) -> (u8, u8, u8) {
    const LEVELS: [u8; 6] = [0, 95, 135, 175, 215, 255];
    if i >= 232 {
        let v = 8 + (i - 232) * 10;
        return (v, v, v);
    }
    let n = i.saturating_sub(16);
    (
        LEVELS[usize::from(n / 36)],
        LEVELS[usize::from(n / 6 % 6)],
        LEVELS[usize::from(n % 6)],
    )
}

/// SGR state of the terminal.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Sgr {
    fg: Option<Color>,
    bg: Option<Color>,
    bold: bool,
    dim: bool,
    italic: bool,
    underline: bool,
    strike: bool,
    inverse: bool,
}

impl Sgr {
    /// Span style with inverse video resolved against the terminal colors.
    fn style(&self) -> SpanStyle {
        let fg = self.fg.map(Color::css);
        let bg = self.bg.map(Color::css);
        let (fg, bg) = if self.inverse {
            (
                Some(bg.unwrap_or_else(|| "var(--op-term-background)".into())),
                Some(fg.unwrap_or_else(|| "var(--op-term-foreground)".into())),
            )
        } else {
            (fg, bg)
        };
        SpanStyle {
            fg,
            bg,
            bold: self.bold,
            dim: self.dim,
            italic: self.italic,
            underline: self.underline,
            strike: self.strike,
            class: None,
        }
    }

    /// Applies the parameter string of one `ESC [ … m` sequence.
    fn apply(&mut self, params: &str) {
        let groups: Vec<&str> = params.split(';').collect();
        let mut i = 0;
        while i < groups.len() {
            let group = groups[i];
            i += 1;
            if group.contains(':') {
                self.apply_colon_group(group);
                continue;
            }
            let code = group.parse::<u16>().unwrap_or(0);
            match code {
                38 | 48 => {
                    let rest: Vec<u16> = groups[i..]
                        .iter()
                        .map(|g| g.parse::<u16>().unwrap_or(0))
                        .collect();
                    let (color, used) = extended_color(&rest);
                    i += used;
                    self.set_color(code == 38, color);
                }
                _ => self.apply_code(code),
            }
        }
    }

    /// Handles a colon sub-parameter group such as `38:2::255:0:0` or `4:0`.
    fn apply_colon_group(&mut self, group: &str) {
        let parts: Vec<u16> = group
            .split(':')
            .map(|p| p.parse::<u16>().unwrap_or(0))
            .collect();
        match parts.first().copied() {
            Some(code @ (38 | 48)) => {
                let rest = &parts[1..];
                // `38:2:<colorspace>:r:g:b` carries an extra colorspace id.
                let rest = if rest.first() == Some(&2) && rest.len() >= 5 {
                    let mut v = vec![2];
                    v.extend_from_slice(&rest[rest.len() - 3..]);
                    v
                } else {
                    rest.to_vec()
                };
                let (color, _) = extended_color(&rest);
                self.set_color(code == 38, color);
            }
            Some(4) => self.underline = parts.get(1).copied().unwrap_or(1) != 0,
            Some(code) => self.apply_code(code),
            None => {}
        }
    }

    fn set_color(&mut self, foreground: bool, color: Option<Color>) {
        if let Some(color) = color {
            if foreground {
                self.fg = Some(color);
            } else {
                self.bg = Some(color);
            }
        }
    }

    fn apply_code(&mut self, code: u16) {
        match code {
            0 => *self = Sgr::default(),
            1 => self.bold = true,
            2 => self.dim = true,
            3 => self.italic = true,
            4 | 21 => self.underline = true,
            7 => self.inverse = true,
            9 => self.strike = true,
            22 => {
                self.bold = false;
                self.dim = false;
            }
            23 => self.italic = false,
            24 => self.underline = false,
            27 => self.inverse = false,
            29 => self.strike = false,
            30..=37 => self.fg = Some(Color::Indexed(index(code - 30))),
            39 => self.fg = None,
            40..=47 => self.bg = Some(Color::Indexed(index(code - 40))),
            49 => self.bg = None,
            90..=97 => self.fg = Some(Color::Indexed(index(code - 90 + 8))),
            100..=107 => self.bg = Some(Color::Indexed(index(code - 100 + 8))),
            _ => {}
        }
    }
}

/// Narrows a small palette offset to `u8`.
fn index(n: u16) -> u8 {
    u8::try_from(n).unwrap_or(0)
}

/// Reads `5;n` or `2;r;g;b` after 38/48; returns the color and how many parameters it used.
fn extended_color(rest: &[u16]) -> (Option<Color>, usize) {
    let byte = |n: u16| u8::try_from(n.min(255)).unwrap_or(u8::MAX);
    match rest {
        [5, n, ..] => (Some(Color::Indexed(byte(*n))), 2),
        [2, r, g, b, ..] => (Some(Color::Rgb(byte(*r), byte(*g), byte(*b))), 4),
        [5] | [2, ..] => (None, rest.len()),
        _ => (None, 0),
    }
}

/// Parses log entries into styled lines; an entry with `\n` becomes several lines.
pub(crate) fn parse_lines<S: AsRef<str>>(entries: &[S]) -> Vec<StyledLine> {
    let mut sgr = Sgr::default();
    let mut lines = Vec::new();
    for entry in entries {
        let mut line = StyledLine::new();
        let mut text = String::new();
        let mut chars = entry.as_ref().chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                ESC => {
                    push_text(&mut line, &text, &sgr.style());
                    text.clear();
                    match chars.next() {
                        Some('[') => {
                            let mut params = String::new();
                            for p in chars.by_ref() {
                                if ('\u{40}'..='\u{7e}').contains(&p) {
                                    if p == 'm' {
                                        sgr.apply(&params);
                                    }
                                    break;
                                }
                                params.push(p);
                            }
                        }
                        Some(']') => {
                            // OSC: ends with BEL or ESC \.
                            while let Some(p) = chars.next() {
                                if p == BEL {
                                    break;
                                }
                                if p == ESC {
                                    chars.next_if_eq(&'\\');
                                    break;
                                }
                            }
                        }
                        // nF escapes such as `ESC ( B`: intermediates, then one final byte.
                        Some(i) if ('\u{20}'..='\u{2f}').contains(&i) => {
                            for p in chars.by_ref() {
                                if !('\u{20}'..='\u{2f}').contains(&p) {
                                    break;
                                }
                            }
                        }
                        _ => {}
                    }
                }
                '\n' => {
                    push_text(&mut line, &text, &sgr.style());
                    text.clear();
                    lines.push(std::mem::take(&mut line));
                }
                '\r' => {
                    // A lone carriage return rewrites the line, as progress bars do.
                    if !matches!(chars.peek(), Some('\n') | None) {
                        line.clear();
                        text.clear();
                    }
                }
                '\t' => text.push(c),
                c if c.is_control() => {}
                c => text.push(c),
            }
        }
        push_text(&mut line, &text, &sgr.style());
        lines.push(line);
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    fn texts(lines: &[StyledLine]) -> Vec<String> {
        lines
            .iter()
            .map(|l| l.iter().map(|s| s.text.as_str()).collect())
            .collect()
    }

    #[test]
    fn plain_text_is_one_unstyled_span() {
        let lines = parse_lines(&["hello"]);
        assert_eq!(texts(&lines), ["hello"]);
        assert_eq!(lines[0][0].style, SpanStyle::default());
    }

    #[test]
    fn basic_and_bright_colors_map_to_term_tokens() {
        let lines = parse_lines(&["\x1b[31mred\x1b[92mgreen\x1b[0m plain"]);
        let l = &lines[0];
        assert_eq!(l[0].style.fg.as_deref(), Some("var(--op-term-red)"));
        assert_eq!(
            l[1].style.fg.as_deref(),
            Some("var(--op-term-bright-green)")
        );
        assert_eq!(l[2].style, SpanStyle::default());
        assert_eq!(texts(&lines), ["redgreen plain"]);
    }

    #[test]
    fn style_carries_over_to_the_next_entry() {
        let lines = parse_lines(&["\x1b[1;34mstart", "still blue"]);
        assert!(lines[1][0].style.bold);
        assert_eq!(lines[1][0].style.fg.as_deref(), Some("var(--op-term-blue)"));
    }

    #[test]
    fn extended_colors_semicolon_and_colon_forms() {
        let lines =
            parse_lines(&["\x1b[38;5;196mA\x1b[48;2;1;2;3mB\x1b[38:2::10:20:30mC\x1b[38;5;244mD"]);
        let l = &lines[0];
        assert_eq!(l[0].style.fg.as_deref(), Some("rgb(255 0 0)"));
        assert_eq!(l[1].style.bg.as_deref(), Some("rgb(1 2 3)"));
        assert_eq!(l[2].style.fg.as_deref(), Some("rgb(10 20 30)"));
        assert_eq!(l[3].style.fg.as_deref(), Some("rgb(128 128 128)"));
    }

    #[test]
    fn low_256_indexes_use_the_palette() {
        let lines = parse_lines(&["\x1b[38;5;9mA"]);
        assert_eq!(
            lines[0][0].style.fg.as_deref(),
            Some("var(--op-term-bright-red)")
        );
    }

    #[test]
    fn attributes_turn_on_and_off() {
        let lines = parse_lines(&["\x1b[1;2;3;4;9ma\x1b[22;23;24;29mb"]);
        let a = &lines[0][0].style;
        assert!(a.bold && a.dim && a.italic && a.underline && a.strike);
        assert_eq!(lines[0][1].style, SpanStyle::default());
    }

    #[test]
    fn inverse_swaps_colors_with_terminal_defaults() {
        let lines = parse_lines(&["\x1b[7mx\x1b[27;31;7my"]);
        let x = &lines[0][0].style;
        assert_eq!(x.fg.as_deref(), Some("var(--op-term-background)"));
        assert_eq!(x.bg.as_deref(), Some("var(--op-term-foreground)"));
        let y = &lines[0][1].style;
        assert_eq!(y.bg.as_deref(), Some("var(--op-term-red)"));
    }

    #[test]
    fn empty_sgr_resets() {
        let lines = parse_lines(&["\x1b[31ma\x1b[mb"]);
        assert_eq!(lines[0][1].style, SpanStyle::default());
    }

    #[test]
    fn other_sequences_are_dropped() {
        let lines = parse_lines(&["\x1b[2K\x1b]0;title\x07a\x1b]8;;http://x\x1b\\b\x1b(Bc\x07d"]);
        assert_eq!(texts(&lines), ["abcd"]);
    }

    #[test]
    fn newline_splits_and_carriage_return_rewrites() {
        let lines = parse_lines(&["one\ntwo", "10%\r50%\r100%", "crlf\r\n"]);
        assert_eq!(texts(&lines), ["one", "two", "100%", "crlf", ""]);
    }

    #[test]
    fn unterminated_sequence_does_not_panic() {
        let lines = parse_lines(&["a\x1b[38;5", "\x1b[38;2;1m", "\x1b"]);
        assert_eq!(texts(&lines), ["a", "", ""]);
    }

    #[test]
    fn cube_and_gray_ramp() {
        assert_eq!(xterm_rgb(16), (0, 0, 0));
        assert_eq!(xterm_rgb(231), (255, 255, 255));
        assert_eq!(xterm_rgb(232), (8, 8, 8));
        assert_eq!(xterm_rgb(255), (238, 238, 238));
    }
}
