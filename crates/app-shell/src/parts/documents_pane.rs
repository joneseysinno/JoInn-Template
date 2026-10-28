use dioxus::prelude::*;

use crate::icons::{self, Ico};
use crate::model::Document;
use crate::state::use_shell;

/// Built-in navigation pane listing open documents: the replacement for
/// Onshape's bottom tab strip. Use it in your nav rail:
/// `RailItem::panel("docs", "Documents", icons::FILES, app_shell::DocumentsPane)`.
#[allow(non_snake_case)]
pub fn DocumentsPane() -> Element {
    let mut shell = use_shell();
    let mut filter = use_signal(String::new);
    let mut show_new = use_signal(|| false);
    let spec = shell.spec.read().clone();
    let active = shell.active.read().clone();
    let f = filter.read().to_lowercase();
    let docs: Vec<Document> = shell
        .documents
        .read()
        .iter()
        .filter(|d| f.is_empty() || d.name.to_lowercase().contains(&f))
        .cloned()
        .collect();

    rsx! {
        div { class: "as-docs",
            div { class: "as-docs-tools",
                label { class: "as-field",
                    Ico { icon: icons::SEARCH, size: 14 }
                    input {
                        r#type: "search",
                        placeholder: "Filter documents",
                        "aria-label": "Filter documents",
                        value: "{filter}",
                        oninput: move |e| filter.set(e.value()),
                    }
                }
                button {
                    class: if show_new() { "as-icon-btn as-active" } else { "as-icon-btn" },
                    "aria-label": "New document",
                    "aria-expanded": if show_new() { "true" } else { "false" },
                    title: "New document",
                    onclick: move |_| show_new.toggle(),
                    Ico { icon: icons::PLUS }
                }
            }
            if show_new() {
                div { class: "as-new-menu",
                    for t in spec.doc_types.iter().cloned() {
                        button {
                            key: "{t.id}",
                            class: "as-row",
                            onclick: move |_| {
                                shell.new_document(t.id);
                                show_new.set(false);
                            },
                            Ico { icon: t.icon, size: 15 }
                            span { "New {t.label}" }
                        }
                    }
                }
            }
            div { class: "as-list", role: "list",
                for d in docs {
                    div {
                        key: "{d.id}",
                        role: "listitem",
                        class: if active.as_deref() == Some(d.id.as_str()) { "as-doc-row as-active" } else { "as-doc-row" },
                        button {
                            class: "as-row",
                            onclick: {
                                let id = d.id.clone();
                                move |_| shell.activate(&id)
                            },
                            Ico { icon: spec.doc_type(d.kind).map(|t| t.icon).unwrap_or(icons::FILES), size: 15 }
                            span { class: "as-ellipsis", "{d.name}" }
                        }
                        button {
                            class: "as-icon-btn as-sm as-row-x",
                            "aria-label": "Close {d.name}",
                            title: "Close",
                            onclick: {
                                let id = d.id.clone();
                                move |_| shell.close_document(&id)
                            },
                            Ico { icon: icons::CLOSE, size: 13 }
                        }
                    }
                }
            }
            p { class: "as-hint", "Alt+[ / Alt+] to switch · Ctrl+K to jump" }
        }
    }
}
