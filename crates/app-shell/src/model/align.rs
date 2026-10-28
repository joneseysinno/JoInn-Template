/// Which end of a rail an item sits on. `End` items are pushed to the
/// bottom of a vertical rail / right of the bottom rail.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Align {
    #[default]
    Start,
    End,
}
