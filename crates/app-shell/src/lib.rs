//! # app-shell
//!
//! A reusable, Onshape-inspired application frame for Dioxus.
//!
//! * **Title bar**: full width; app brand, active document, app actions.
//! * **Nav rail** (far left) opening **navigation panes** (documents,
//!   versions, comments...). The Documents pane replaces bottom tabs.
//! * **Main window**, entirely driven by the active document's [`DocType`]:
//!   toolbar on top, icon rails on the left, right and bottom, each owning
//!   a resizable panel slot, and a viewport in the middle.
//! * A generated **command palette** (Ctrl+K).
//!
//! You describe an app as data ([`ShellSpec`]) and supply views as plain
//! `fn() -> Element`. Views talk back through [`use_shell`].

pub mod icons;
mod model;
mod palette;
mod parts;
mod shell;
mod state;

pub use model::*;
pub use parts::{use_document, DocId, DocumentsPane, SettingsPane, ToolButton};
pub use shell::{AppShell, AppShellProps};
pub use state::{cmd, use_active_document, use_shell, Shell, SlotState};

pub mod prelude {
    pub use crate::icons::{self, Icon, Ico};
    pub use crate::{
        cmd, use_active_document, use_document, use_shell, Align, AppShell, CommandEvent, DocType,
        Document, DocumentsPane, Placement, RailItem, SettingsPane, Shell, ShellSpec, Theme, Tool,
        ToolGroup, View,
    };
}
