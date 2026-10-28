use dioxus::prelude::*;

use super::doc_host::DocHost;
use super::rail_button::RailButton;
use super::rail_class::rail_class;
use super::rail_label::rail_label;
use crate::model::{Align, Placement, RailItem};
use crate::state::use_shell;

#[component]
pub(crate) fn Rail(placement: Placement) -> Element {
    let shell = use_shell();
    let ty = shell.active_type();
    let items: Vec<RailItem> = match placement {
        Placement::Nav => shell.spec.read().nav_rail.clone(),
        p => ty.as_ref().map(|t| t.rail(p).to_vec()).unwrap_or_default(),
    };
    let status = if placement == Placement::Bottom { ty.as_ref().and_then(|t| t.status) } else { None };
    // A document type with no items for a rail simply doesn't get that rail.
    if items.is_empty() && status.is_none() && placement != Placement::Nav {
        return rsx! {};
    }
    let open = shell.slot(placement);
    let doc_id = shell.active.read().clone().unwrap_or_default();
    let start: Vec<RailItem> = items.iter().filter(|i| i.align == Align::Start).cloned().collect();
    let end: Vec<RailItem> = items.iter().filter(|i| i.align == Align::End).cloned().collect();

    rsx! {
        nav { class: rail_class(placement), "aria-label": rail_label(placement),
            for it in start {
                RailButton { key: "{it.id}", active: open == Some(it.id), item: it, placement }
            }
            if let Some(v) = status {
                div { class: "as-status",
                    DocHost { key: "status-{doc_id}", doc_id: doc_id.clone(), view: v }
                }
            }
            div { class: "as-grow" }
            for it in end {
                RailButton { key: "{it.id}", active: open == Some(it.id), item: it, placement }
            }
        }
    }
}
