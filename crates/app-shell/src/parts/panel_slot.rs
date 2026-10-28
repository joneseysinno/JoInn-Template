use dioxus::prelude::*;

use super::doc_host::DocHost;
use super::splitter::Splitter;
use super::view_host::ViewHost;
use crate::icons::{self, Ico};
use crate::model::{Placement, RailAction, RailItem};
use crate::state::use_shell;

#[component]
pub(crate) fn PanelSlot(placement: Placement) -> Element {
    let mut shell = use_shell();
    let Some(open) = shell.slot(placement) else { return rsx! {} };
    let items: Vec<RailItem> = match placement {
        Placement::Nav => shell.spec.read().nav_rail.clone(),
        p => shell.active_type().map(|t| t.rail(p).to_vec()).unwrap_or_default(),
    };
    let Some((label, view)) = items.iter().find(|i| i.id == open).and_then(|i| match i.action {
        RailAction::Panel(v) => Some((i.label, v)),
        RailAction::Command => None,
    }) else {
        return rsx! {};
    };
    let size = *shell.size_signal(placement).read();
    let (class, style) = match placement {
        Placement::Nav => ("as-panel as-panel-nav", format!("width:{size}px")),
        Placement::Left => ("as-panel as-panel-left", format!("width:{size}px")),
        Placement::Right => ("as-panel as-panel-right", format!("width:{size}px")),
        Placement::Bottom => ("as-panel as-panel-bottom", format!("height:{size}px")),
    };
    let doc_id = shell.active.read().clone().unwrap_or_default();

    rsx! {
        section { class, style, "aria-label": label,
            div { class: "as-panel-head",
                span { class: "as-panel-title", "{label}" }
                button {
                    class: "as-icon-btn as-sm",
                    "aria-label": "Close {label}",
                    title: "Close",
                    onclick: move |_| shell.set_panel(placement, None),
                    Ico { icon: icons::CLOSE, size: 14 }
                }
            }
            div { class: "as-panel-body",
                if placement == Placement::Nav {
                    ViewHost { key: "nav-{open}", view }
                } else {
                    DocHost { key: "{doc_id}-{open}", doc_id: doc_id.clone(), view }
                }
            }
            Splitter { placement }
        }
    }
}
