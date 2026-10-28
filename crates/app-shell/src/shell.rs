//! `AppShell`: the layout grid, keyboard handling and splitter dragging.
//!
//! ```text
//! ┌───────────────────────── TitleBar ─────────────────────────┐
//! ├────┬──────────┬────────────── MainWindow ──────────────────┤
//! │Nav │ Nav pane │ Toolbar (per DocType)                      │
//! │rail│ (docs,   ├────────┬───┬──────────────┬────────┬───────┤
//! │    │ versions,│ Left   │ L │  Viewport    │ Right  │ R     │
//! │    │ ...)     │ panel  │ r │ (per Doc)    │ panel  │ r     │
//! │    │          │        │ a ├──────────────┤        │ a     │
//! │    │          │        │ i │ Bottom panel │        │ i     │
//! │    │          │        │ l │              │        │ l     │
//! │    │          ├────────┴───┴──────────────┴────────┴───────┤
//! │    │          │ Bottom rail  [items]  status  [end items]  │
//! └────┴──────────┴────────────────────────────────────────────┘
//! ```

mod app_shell_props;
mod main_window;
mod on_drag;
mod on_key;

use dioxus::prelude::*;

pub use app_shell_props::AppShellProps;
use main_window::MainWindow;
use on_drag::on_drag;
use on_key::on_key;

use crate::model::{Placement, Theme};
use crate::palette::Palette;
use crate::parts::{PanelSlot, Rail, TitleBar};
use crate::state::Shell;

const CSS: &str = include_str!("shell.css");

/// The whole application frame. Put it at the root of your app:
///
/// ```ignore
/// fn App() -> Element { rsx! { AppShell { spec: my_spec } } }
/// ```
#[allow(non_snake_case)]
pub fn AppShell(props: AppShellProps) -> Element {
    let build = props.spec;
    let mut shell = use_context_provider(|| Shell::new(build()));

    let theme = match *shell.theme.read() {
        Theme::Dark => "dark",
        Theme::Light => "light",
    };
    let dragging = shell.drag.read().is_some();
    let nav_open = shell.nav_pane.read().is_some();
    let palette_open = *shell.palette_open.read();
    let root_class = if dragging { "as-root as-dragging" } else { "as-root" };

    rsx! {
        style { {CSS} }
        div {
            class: root_class,
            "data-theme": theme,
            tabindex: "0",
            onmounted: move |e| {
                let el = e.data();
                shell.root_el.set(Some(el.clone()));
                async move {
                    let _ = el.set_focus(true).await;
                }
            },
            onkeydown: move |e: KeyboardEvent| on_key(shell, e),
            onpointermove: move |e: PointerEvent| on_drag(shell, e),
            onpointerup: move |_| shell.drag.set(None),
            onpointerleave: move |_| shell.drag.set(None),

            TitleBar {}
            div { class: "as-body",
                Rail { placement: Placement::Nav }
                if nav_open {
                    PanelSlot { placement: Placement::Nav }
                }
                MainWindow {}
            }
            if palette_open {
                Palette {}
            }
        }
    }
}
