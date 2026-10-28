/// Where a rail lives. Each rail owns exactly one panel slot.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Placement {
    /// Far-left global rail; opens navigation panes (documents, versions...).
    Nav,
    /// Inner-left rail of the main window (feature tree, outline...).
    Left,
    /// Inner-right rail of the main window (properties, appearance...).
    Right,
    /// Bottom rail of the main window; opens a bottom drawer (console...).
    Bottom,
}
