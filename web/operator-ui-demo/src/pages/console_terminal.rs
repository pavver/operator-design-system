//! Live examples for console terminal.

use crate::kit::{Example, backend};
use leptos::prelude::*;
use operator_ui::prelude::*;

/// xterm output that exercises the SGR parser.
const ANSI_SAMPLE: &[&str] = &[
    "\x1b[1mcargo build\x1b[0m --release",
    "   \x1b[1;32mCompiling\x1b[0m operator-ui v0.6.0",
    "\x1b[1;33mwarning\x1b[0m\x1b[1m: unused variable: `frame`\x1b[0m",
    "\x1b[1;31merror[E0308]\x1b[0m\x1b[1m: mismatched types\x1b[0m",
    "16 colors: \x1b[30;47m 0 \x1b[0m\x1b[31m 1 \x1b[32m 2 \x1b[33m 3 \x1b[34m 4 \x1b[35m 5 \x1b[36m 6 \x1b[37m 7 \x1b[0m",
    "bright:    \x1b[90m 8 \x1b[91m 9 \x1b[92m 10 \x1b[93m 11 \x1b[94m 12 \x1b[95m 13 \x1b[96m 14 \x1b[97m 15 \x1b[0m",
    "256 colors: \x1b[38;5;208m208\x1b[0m \x1b[38;5;45m45\x1b[0m \x1b[48;5;236m 236 \x1b[0m  truecolor: \x1b[38;2;240;165;58mamber\x1b[0m",
    "\x1b[1mbold\x1b[0m \x1b[2mdim\x1b[0m \x1b[3mitalic\x1b[0m \x1b[4munderline\x1b[0m \x1b[9mstrike\x1b[0m \x1b[7minverse\x1b[0m",
    "download  10%\rdownload  55%\rdownload 100% \x1b[32mdone\x1b[0m",
];

pub fn console() -> AnyView {
    let b = backend();
    let sample = ANSI_SAMPLE
        .iter()
        .map(|l| (*l).to_owned())
        .collect::<Vec<_>>();
    view! {
        <Example title="Event log of this site" note="Every simulated request on these pages is logged here. It auto-scrolls to the newest line." block=true>
            <ConsoleTerminal logs=b.logs/>
        </Example>
        <Example title="xterm escape sequences" note="Colors and attributes come from SGR sequences; a carriage return rewrites the line." block=true>
            <ConsoleTerminal logs=sample height=50.0/>
        </Example>

        <Example title="Customization" note="Log metrics, a header slot and an application accessible name." block=true>
            <ThemeRoot strings=Callback::new(|key| (key == "console-terminal.label").then(|| "Build output".to_string()))>
                <ConsoleTerminal logs=vec!["Build complete".to_string()] height=25.0 padding=4.0 gap=2.0
                    header=|| view! { <strong>"Build"</strong> }/>
            </ThemeRoot>
        </Example>
    }
    .into_any()
}
