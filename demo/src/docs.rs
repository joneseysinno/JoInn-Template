//! App-side document types. Per-document data lives in global signals keyed
//! by document id, so the viewport, side panels and status readout of one
//! document all see the same state.

use app_shell::prelude::*;
use dioxus::prelude::*;

pub mod calc_sheet;
pub mod part_studio;

pub const DEMO_CSS: &str = include_str!("docs/demo.css");

/// Every command from every document type lands here (after the shell
/// has handled its own built-ins).
pub fn on_command(_shell: Shell, ev: &CommandEvent) {
    let Some(doc) = ev.doc.clone() else { return };
    if ev.id.starts_with("ps.") {
        part_studio::handle(&doc, ev.id);
    }
}

/// Bottom panel shared by both document types: this document's command log.
#[allow(non_snake_case)]
pub fn ConsolePane() -> Element {
    let shell = use_shell();
    let me = use_document().map(|d| d.id);
    let lines: Vec<CommandEvent> = shell
        .history
        .read()
        .iter()
        .filter(|e| e.doc == me && !e.id.starts_with("shell."))
        .cloned()
        .collect();
    rsx! {
        div { class: "demo-console",
            if lines.is_empty() {
                p { class: "demo-muted", "> waiting for commands" }
            }
            for ev in lines {
                div { key: "{ev.seq}", span { class: "demo-muted", "#{ev.seq} " } "{ev.id}" }
            }
        }
    }
}
