use dioxus::prelude::*;

use super::tool_button::ToolButton;
use crate::icons::{self, Ico};
use crate::model::DocType;
use crate::state::{cmd, use_shell};

#[component]
pub(crate) fn Toolbar(ty: DocType) -> Element {
    let mut shell = use_shell();
    rsx! {
        div { class: "as-toolbar", role: "toolbar", "aria-label": "{ty.label} tools",
            for (gi, g) in ty.toolbar.iter().enumerate() {
                if gi > 0 {
                    div { class: "as-tsep", "aria-hidden": "true" }
                }
                div { class: "as-tgroup", role: "group", "aria-label": g.label,
                    for t in g.tools.iter() {
                        ToolButton { key: "{t.id}", tool: t.clone() }
                    }
                }
            }
            div { class: "as-grow" }
            button {
                class: "as-search",
                onclick: move |_| shell.run(cmd::PALETTE),
                Ico { icon: icons::SEARCH, size: 14 }
                span { "Search tools…" }
                kbd { "Ctrl K" }
            }
        }
    }
}
