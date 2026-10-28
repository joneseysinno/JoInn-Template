use dioxus::prelude::*;

use super::tool_button::ToolButton;
use crate::icons::{self, Ico};
use crate::state::{cmd, use_shell};

#[component]
pub(crate) fn TitleBar() -> Element {
    let mut shell = use_shell();
    let spec = shell.spec.read().clone();
    let doc = shell.active_document();
    let ty = shell.active_type();

    rsx! {
        header { class: "as-titlebar",
            div { class: "as-brand",
                Ico { icon: spec.logo, size: 20 }
                span { "{spec.app_name}" }
            }
            button {
                class: "as-icon-btn",
                title: "Commands (Ctrl+K)",
                "aria-label": "Open command palette",
                onclick: move |_| shell.run(cmd::PALETTE),
                Ico { icon: icons::MENU }
            }
            if let Some(d) = doc {
                div { class: "as-doc-title",
                    if let Some(t) = ty {
                        span { class: "as-doc-kind", title: "{t.label}", Ico { icon: t.icon, size: 15 } }
                    }
                    h1 { class: "as-doc-name", "{d.name}" }
                    if let Some(c) = spec.context_label {
                        span { class: "as-context", "{c}" }
                    }
                }
            }
            div { class: "as-grow" }
            div { class: "as-title-actions",
                for t in spec.title_actions.iter() {
                    ToolButton { key: "{t.id}", tool: t.clone() }
                }
            }
        }
    }
}
