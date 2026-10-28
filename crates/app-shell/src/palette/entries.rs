use dioxus::prelude::*;

use super::act::Act;
use super::entry::Entry;
use super::rail_entries::rail_entries;
use crate::icons;
use crate::model::Placement;
use crate::state::{cmd, Shell};

pub(super) fn entries(shell: &Shell) -> Vec<Entry> {
    let spec = shell.spec.read().clone();
    let mut out = vec![];
    if let Some(ty) = shell.active_type() {
        for g in &ty.toolbar {
            for t in &g.tools {
                out.push(Entry {
                    label: t.label.to_string(),
                    group: g.label,
                    hint: t.shortcut,
                    icon: t.icon,
                    act: Act::Cmd(t.id),
                });
            }
        }
        rail_entries(&mut out, &ty.left_rail, Placement::Left, "Panel");
        rail_entries(&mut out, &ty.right_rail, Placement::Right, "Panel");
        rail_entries(&mut out, &ty.bottom_rail, Placement::Bottom, "Panel");
    }
    rail_entries(&mut out, &spec.nav_rail, Placement::Nav, "Navigate");
    for d in shell.documents.read().iter() {
        out.push(Entry {
            label: d.name.clone(),
            group: "Document",
            hint: None,
            icon: spec.doc_type(d.kind).map(|t| t.icon).unwrap_or(icons::FILES),
            act: Act::Open(d.id.clone()),
        });
    }
    for t in &spec.doc_types {
        out.push(Entry {
            label: format!("New {}", t.label),
            group: "Create",
            hint: None,
            icon: icons::PLUS,
            act: Act::New(t.id),
        });
    }
    for t in &spec.title_actions {
        out.push(Entry { label: t.label.to_string(), group: "App", hint: None, icon: t.icon, act: Act::Cmd(t.id) });
    }
    out.push(Entry {
        label: "Toggle theme".into(),
        group: "App",
        hint: None,
        icon: icons::SUN,
        act: Act::Cmd(cmd::TOGGLE_THEME),
    });
    out.push(Entry {
        label: "Toggle navigation pane".into(),
        group: "App",
        hint: Some("Ctrl B"),
        icon: icons::MENU,
        act: Act::Cmd(cmd::TOGGLE_NAV),
    });
    out
}
