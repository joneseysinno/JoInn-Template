use dioxus::prelude::*;

use crate::icons::Ico;
use crate::model::{Placement, RailAction, RailItem};
use crate::state::use_shell;

#[component]
pub(super) fn RailButton(item: RailItem, placement: Placement, active: bool) -> Element {
    let mut shell = use_shell();
    let id = item.id;
    let is_panel = matches!(item.action, RailAction::Panel(_));
    // Side tabs show a CSS label on hover; a native tooltip would double it.
    let is_tab = matches!(placement, Placement::Left | Placement::Right);
    rsx! {
        button {
            class: if active { "as-rbtn as-active" } else { "as-rbtn" },
            title: if is_tab { None } else { Some(item.label) },
            "aria-label": item.label,
            "aria-pressed": if !is_panel { None } else if active { Some("true") } else { Some("false") },
            onclick: move |_| {
                if is_panel {
                    shell.toggle_panel(placement, id)
                } else {
                    shell.run(id)
                }
            },
            Ico { icon: item.icon }
            span { class: "as-rlabel", "{item.label}" }
        }
    }
}
