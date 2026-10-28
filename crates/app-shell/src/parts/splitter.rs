use dioxus::prelude::*;

use crate::model::Placement;
use crate::state::{use_shell, Drag};

#[component]
pub(super) fn Splitter(placement: Placement) -> Element {
    let mut shell = use_shell();
    let class = match placement {
        Placement::Bottom => "as-split as-split-h",
        Placement::Right => "as-split as-split-v as-split-start",
        _ => "as-split as-split-v",
    };
    rsx! {
        div {
            class,
            "aria-hidden": "true",
            onpointerdown: move |e: PointerEvent| {
                e.prevent_default();
                let p = e.client_coordinates();
                let start = if placement == Placement::Bottom { p.y } else { p.x };
                let start_size = *shell.size_signal(placement).read();
                shell.drag.set(Some(Drag { target: placement, start, start_size }));
            },
        }
    }
}
