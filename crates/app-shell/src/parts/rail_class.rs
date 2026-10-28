use crate::model::Placement;

pub(super) fn rail_class(p: Placement) -> &'static str {
    match p {
        Placement::Nav => "as-rail as-rail-nav",
        Placement::Left => "as-tabs as-tabs-left",
        Placement::Right => "as-tabs as-tabs-right",
        Placement::Bottom => "as-rail as-rail-bottom",
    }
}
