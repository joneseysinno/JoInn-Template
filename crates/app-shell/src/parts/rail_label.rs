use crate::model::Placement;

pub(super) fn rail_label(p: Placement) -> &'static str {
    match p {
        Placement::Nav => "Navigation",
        Placement::Left => "Left panels",
        Placement::Right => "Right panels",
        Placement::Bottom => "Bottom tools",
    }
}
