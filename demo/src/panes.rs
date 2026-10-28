use app_shell::prelude::*;
use dioxus::prelude::*;

/// Nav pane: every command run in this session, newest first.
#[allow(non_snake_case)]
pub fn ActivityPane() -> Element {
    let shell = use_shell();
    let docs = shell.documents.read().clone();
    let history: Vec<CommandEvent> = shell.history.read().iter().rev().take(100).cloned().collect();
    rsx! {
        div { class: "demo-pad",
            if history.is_empty() {
                p { class: "demo-muted", "Nothing yet. Run a tool from the toolbar or press Ctrl+K." }
            }
            ol { class: "demo-log",
                for ev in history {
                    li { key: "{ev.seq}",
                        code { "{ev.id}" }
                        span { class: "demo-muted",
                            {ev.doc.as_ref().and_then(|id| docs.iter().find(|d| &d.id == id)).map(|d| d.name.clone()).unwrap_or_default()}
                        }
                    }
                }
            }
        }
    }
}
