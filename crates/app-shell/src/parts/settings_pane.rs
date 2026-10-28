use dioxus::prelude::*;

use crate::model::Theme;
use crate::state::use_shell;

/// Built-in settings pane: theme + keyboard reference.
#[allow(non_snake_case)]
pub fn SettingsPane() -> Element {
    let mut shell = use_shell();
    let dark = *shell.theme.read() == Theme::Dark;
    rsx! {
        div { class: "as-pad",
            h3 { class: "as-h3", "Appearance" }
            div { class: "as-seg", role: "group", "aria-label": "Theme",
                button {
                    class: if dark { "as-active" } else { "" },
                    "aria-pressed": if dark { "true" } else { "false" },
                    onclick: move |_| shell.theme.set(Theme::Dark),
                    "Dark"
                }
                button {
                    class: if !dark { "as-active" } else { "" },
                    "aria-pressed": if !dark { "true" } else { "false" },
                    onclick: move |_| shell.theme.set(Theme::Light),
                    "Light"
                }
            }
            h3 { class: "as-h3", "Keyboard" }
            dl { class: "as-keys",
                dt { kbd { "Ctrl K" } } dd { "Command palette" }
                dt { kbd { "Ctrl B" } } dd { "Toggle navigation pane" }
                dt { kbd { "Alt ]" } } dd { "Next document" }
                dt { kbd { "Alt [" } } dd { "Previous document" }
                dt { kbd { "Esc" } } dd { "Close palette" }
            }
        }
    }
}
