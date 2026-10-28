use dioxus::prelude::*;

use crate::model::Placement;
use crate::state::Shell;

pub(super) fn on_drag(shell: Shell, e: PointerEvent) {
    let Some(d) = *shell.drag.read() else { return };
    let p = e.client_coordinates();
    let (delta, min, max) = match d.target {
        Placement::Bottom => (d.start - p.y, 80.0, 640.0),
        Placement::Right => (d.start - p.x, 180.0, 640.0),
        Placement::Left | Placement::Nav => (p.x - d.start, 160.0, 560.0),
    };
    let mut sig = shell.size_signal(d.target);
    sig.set((d.start_size + delta).clamp(min, max));
}
