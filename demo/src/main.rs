//! Demo: one shell, two very different document types.
//!
//! * Part Studio: CAD-like (toolbar of modeling tools, feature tree, viewport)
//! * Calc Sheet: engineering worksheet (inputs, live results, variables)
//!
//! Everything app-specific lives in `spec()` and the `docs` modules; the
//! shell crate knows nothing about CAD or calcs.

use app_shell::prelude::*;
use dioxus::prelude::*;

mod docs;
mod panes;

fn main() {
    #[cfg(feature = "desktop")]
    {
        use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
        let window = WindowBuilder::new()
            .with_title("Forge")
            .with_inner_size(LogicalSize::new(1440.0, 900.0));
        dioxus::LaunchBuilder::desktop()
            .with_cfg(Config::new().with_window(window).with_menu(None))
            .launch(App);
    }
    #[cfg(not(feature = "desktop"))]
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        style { {docs::DEMO_CSS} }
        AppShell { spec }
    }
}

/// The whole app, described as data.
fn spec() -> ShellSpec {
    let mut s = ShellSpec::new("Forge");
    s.context_label = Some("Main · 8x16 Mats");
    s.title_actions = vec![
        Tool::new("app.notifications", "Notifications", icons::BELL),
        Tool::new("app.help", "Help", icons::HELP),
        Tool::new("app.share", "Share", icons::SHARE).primary(),
        Tool::new("app.account", "Account", icons::USER),
    ];
    s.nav_rail = vec![
        RailItem::panel("nav.docs", "Documents", icons::FILES, DocumentsPane),
        RailItem::panel("nav.activity", "Activity", icons::HISTORY, panes::ActivityPane),
        RailItem::panel("nav.settings", "Settings", icons::SLIDERS, SettingsPane).at_end(),
    ];
    s.default_nav = Some("nav.docs");
    s.doc_types = vec![docs::part_studio::doc_type(), docs::calc_sheet::doc_type()];
    s.documents = vec![
        Document::new("d1", "6x10 Mat", "part-studio"),
        Document::new("d2", "Tapered Column Mat", "part-studio"),
        Document::new("d3", "Mat Bearing Check", "calc-sheet"),
    ];
    s.on_command = Some(docs::on_command);
    s
}
