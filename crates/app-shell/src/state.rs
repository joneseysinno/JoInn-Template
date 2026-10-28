//! The *state* half of the shell: one `Copy` handle full of signals,
//! provided as context. Any view the app supplies can grab it with
//! [`use_shell`] and drive the shell (open panels, run commands, switch
//! documents) without prop drilling.

pub mod cmd;
mod drag;
mod shell;
mod slot_state;
mod use_active_document;
mod use_shell;

pub use drag::Drag;
pub use shell::Shell;
pub use slot_state::SlotState;
pub use use_active_document::use_active_document;
pub use use_shell::use_shell;
