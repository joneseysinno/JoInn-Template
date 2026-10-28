//! The individual regions of the shell.

mod doc_host;
mod doc_id;
mod documents_pane;
mod empty_state;
mod panel_slot;
mod rail;
mod rail_button;
mod rail_class;
mod rail_label;
mod settings_pane;
mod splitter;
mod title_bar;
mod tool_button;
mod toolbar;
mod use_document;
mod view_host;

pub(crate) use doc_host::DocHost;
pub use doc_id::DocId;
pub use documents_pane::DocumentsPane;
pub(crate) use empty_state::EmptyState;
pub(crate) use panel_slot::PanelSlot;
pub(crate) use rail::Rail;
pub use settings_pane::SettingsPane;
pub(crate) use title_bar::TitleBar;
pub use tool_button::ToolButton;
pub(crate) use toolbar::Toolbar;
pub use use_document::use_document;
