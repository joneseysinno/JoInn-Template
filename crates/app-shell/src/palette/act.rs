use crate::model::Placement;

#[derive(Clone, PartialEq, Debug)]
pub(super) enum Act {
    Cmd(&'static str),
    Panel(Placement, &'static str),
    Open(String),
    New(&'static str),
}
