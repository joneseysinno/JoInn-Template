//! The *data* half of the shell.
//!
//! An app is described entirely by a [`ShellSpec`]: plain structs and
//! function pointers, no components. The shell turns that description into
//! UI. Swapping the spec is how the same shell becomes a CAD tool, a calc
//! package, a log viewer, etc.

mod align;
mod command_event;
mod doc_type;
mod document;
mod placement;
mod rail_action;
mod rail_item;
mod shell_spec;
mod theme;
mod tool;
mod tool_group;
mod view;

pub use align::Align;
pub use command_event::CommandEvent;
pub use doc_type::DocType;
pub use document::Document;
pub use placement::Placement;
pub use rail_action::RailAction;
pub use rail_item::RailItem;
pub use shell_spec::ShellSpec;
pub use theme::Theme;
pub use tool::Tool;
pub use tool_group::ToolGroup;
pub use view::View;
