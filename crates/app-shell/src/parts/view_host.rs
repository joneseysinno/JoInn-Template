use dioxus::prelude::*;

use crate::model::View;

/// Mounts a [`View`] in its own component scope so its hooks are isolated.
/// Callers key it, so a different view = a fresh mount.
#[component]
pub(crate) fn ViewHost(view: View) -> Element {
    (view.0)()
}
