use crate::model::Placement;

/// Something being resized with a splitter.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Drag {
    pub target: Placement,
    pub start: f64,
    pub start_size: f64,
}
