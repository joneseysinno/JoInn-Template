use dioxus::prelude::*;

use crate::icons::Ico;
use crate::state::use_shell;

#[component]
pub(crate) fn EmptyState() -> Element {
    let mut shell = use_shell();
    let types = shell.spec.read().doc_types.clone();
    rsx! {
        div { class: "as-empty",
            h2 { "No document open" }
            p { "Create one to get started, or open one from the navigation pane." }
            div { class: "as-empty-actions",
                for t in types {
                    button {
                        key: "{t.id}",
                        class: "as-btn",
                        onclick: move |_| {
                            shell.new_document(t.id);
                        },
                        Ico { icon: t.icon, size: 16 }
                        span { "New {t.label}" }
                    }
                }
            }
        }
    }
}
