//! Command palette (Ctrl+K). It is generated, not authored: every toolbar
//! tool, rail item, title action, open document and "New <type>" becomes
//! an entry automatically. Any app built on the shell gets full keyboard
//! reach for free.

mod act;
mod entries;
mod entry;
mod exec;
mod fuzzy;
mod rail_entries;

use dioxus::prelude::*;

use crate::icons::Ico;
use entries::entries;
use entry::Entry;
use exec::exec;
use fuzzy::fuzzy;

#[component]
pub(crate) fn Palette() -> Element {
    let mut shell = crate::use_shell();
    let mut query = use_signal(String::new);
    let mut sel = use_signal(|| 0usize);

    let q = query.read().to_lowercase();
    let hits: Vec<Entry> = entries(&shell)
        .into_iter()
        .filter(|e| fuzzy(&q, &e.label.to_lowercase()))
        .take(60)
        .collect();
    let n = hits.len();
    let s = sel().min(n.saturating_sub(1));
    let keyed = hits.clone();

    rsx! {
        div {
            class: "as-scrim",
            onclick: move |_| {
                shell.palette_open.set(false);
                shell.focus_root();
            },
            div {
                class: "as-palette",
                role: "dialog",
                "aria-label": "Command palette",
                onclick: move |e| e.stop_propagation(),
                input {
                    class: "as-pal-input",
                    placeholder: "Type a tool, panel, or document…",
                    "aria-label": "Search commands",
                    value: "{query}",
                    onmounted: move |e| async move {
                        let _ = e.set_focus(true).await;
                    },
                    oninput: move |e| {
                        query.set(e.value());
                        sel.set(0);
                    },
                    onkeydown: move |e: KeyboardEvent| {
                        e.stop_propagation();
                        match e.key() {
                            Key::ArrowDown => {
                                e.prevent_default();
                                if n > 0 { sel.set((s + 1) % n) }
                            }
                            Key::ArrowUp => {
                                e.prevent_default();
                                if n > 0 { sel.set((s + n - 1) % n) }
                            }
                            Key::Enter => {
                                if let Some(h) = keyed.get(s) {
                                    exec(shell, h.act.clone());
                                }
                            }
                            Key::Escape => {
                                shell.palette_open.set(false);
                                shell.focus_root();
                            }
                            _ => {}
                        }
                    },
                }
                div { class: "as-pal-list", role: "listbox", "aria-label": "Results",
                    for (i, h) in hits.iter().enumerate() {
                        button {
                            key: "{i}-{h.label}",
                            role: "option",
                            "aria-selected": if i == s { "true" } else { "false" },
                            class: if i == s { "as-pal-row as-sel" } else { "as-pal-row" },
                            onmouseenter: move |_| sel.set(i),
                            onclick: {
                                let a = h.act.clone();
                                move |_| exec(shell, a.clone())
                            },
                            Ico { icon: h.icon, size: 16 }
                            span { class: "as-pal-label", "{h.label}" }
                            span { class: "as-pal-group", "{h.group}" }
                            if let Some(k) = h.hint {
                                kbd { "{k}" }
                            }
                        }
                    }
                    if n == 0 {
                        p { class: "as-pal-empty", "No matches" }
                    }
                }
            }
        }
    }
}
