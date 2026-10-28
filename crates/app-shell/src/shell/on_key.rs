use dioxus::prelude::*;

use crate::state::{cmd, Shell};

pub(super) fn on_key(mut shell: Shell, e: KeyboardEvent) {
    let m = e.modifiers();
    let ctrl = m.contains(Modifiers::CONTROL) || m.contains(Modifiers::META);
    let alt = m.contains(Modifiers::ALT);
    let shift = m.contains(Modifiers::SHIFT);
    match e.key() {
        Key::Character(c) if ctrl && c.eq_ignore_ascii_case("k") => {
            e.prevent_default();
            shell.run(cmd::PALETTE);
        }
        Key::Character(c) if ctrl && c.eq_ignore_ascii_case("b") => {
            e.prevent_default();
            shell.run(cmd::TOGGLE_NAV);
        }
        // Alt+]/[ cycles documents: the replacement for bottom tabs.
        Key::Character(c) if alt && c == "]" => shell.run(cmd::NEXT_DOC),
        Key::Character(c) if alt && c == "[" => shell.run(cmd::PREV_DOC),
        Key::Escape => shell.palette_open.set(false),
        Key::Character(c) if !ctrl && !alt => {
            // Single-key tool shortcuts ("S" = sketch, "Shift+E" = ...).
            let pressed = if shift { format!("Shift+{}", c.to_uppercase()) } else { c.to_uppercase() };
            let hit = shell.active_type().and_then(|t| {
                t.toolbar
                    .iter()
                    .flat_map(|g| g.tools.iter())
                    .find(|tool| tool.shortcut == Some(pressed.as_str()))
                    .map(|tool| tool.id)
            });
            if let Some(id) = hit {
                e.prevent_default();
                shell.run(id);
            }
        }
        _ => {}
    }
}
