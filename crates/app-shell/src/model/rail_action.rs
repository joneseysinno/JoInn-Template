use super::view::View;

/// What clicking a rail icon does.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RailAction {
    /// Toggle the rail's panel slot to show this view.
    Panel(View),
    /// Fire a command (see [`crate::Shell::run`]) without opening anything.
    Command,
}
