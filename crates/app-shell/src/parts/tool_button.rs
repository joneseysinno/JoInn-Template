use dioxus::prelude::*;

use crate::icons::Ico;
use crate::model::Tool;
use crate::state::use_shell;

#[component]
pub fn ToolButton(tool: Tool) -> Element {
    let mut shell = use_shell();
    let id = tool.id;
    let tip = match tool.shortcut {
        Some(k) => format!("{} ({k})", tool.label),
        None => tool.label.to_string(),
    };
    rsx! {
        button {
            class: if tool.primary { "as-tbtn as-primary" } else { "as-tbtn" },
            title: "{tip}",
            "aria-label": tool.label,
            onclick: move |_| shell.run(id),
            Ico { icon: tool.icon }
            if tool.primary {
                span { "{tool.label}" }
            }
        }
    }
}
