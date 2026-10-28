use dioxus::prelude::*;

use crate::model::Placement;
use crate::parts::{DocHost, EmptyState, PanelSlot, Rail, Toolbar};

/// Everything right of the nav pane, driven by the active document's type.
#[component]
pub(super) fn MainWindow() -> Element {
    let shell = crate::use_shell();
    let Some(ty) = shell.active_type() else {
        return rsx! {
            div { class: "as-main", EmptyState {} }
        };
    };
    let active = shell.active.read().clone().unwrap_or_default();
    let docs = shell.documents.read().clone();
    let spec = shell.spec.read().clone();

    rsx! {
        main { class: "as-main",
            Toolbar { ty: ty.clone() }
            div { class: "as-work",
                PanelSlot { placement: Placement::Left }
                div { class: "as-center",
                    // Keep-alive: every open document stays mounted, only the
                    // active one is visible, so viewport state survives switching.
                    div { class: "as-viewports",
                        for doc in docs.iter().filter(|d| spec.doc_type(d.kind).is_some()) {
                            div {
                                key: "{doc.id}",
                                class: if doc.id == active { "as-viewport" } else { "as-viewport as-hidden" },
                                DocHost {
                                    doc_id: doc.id.clone(),
                                    view: spec.doc_type(doc.kind).map(|t| t.viewport).unwrap(),
                                }
                            }
                        }
                        // Side tabs float over the viewport edges, on the
                        // viewport side of their panels, so the tab you click
                        // to collapse a panel stays put next to the model.
                        Rail { placement: Placement::Left }
                        Rail { placement: Placement::Right }
                    }
                    PanelSlot { placement: Placement::Bottom }
                }
                PanelSlot { placement: Placement::Right }
            }
            Rail { placement: Placement::Bottom }
        }
    }
}
