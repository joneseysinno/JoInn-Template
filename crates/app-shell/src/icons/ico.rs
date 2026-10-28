use dioxus::prelude::*;

use super::icon::Icon;

/// Renders an [`Icon`].
#[component]
pub fn Ico(icon: Icon, #[props(default = 18)] size: u32) -> Element {
    rsx! {
        svg {
            class: "as-ico",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.75",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            "aria-hidden": "true",
            path { d: icon.0 }
        }
    }
}
