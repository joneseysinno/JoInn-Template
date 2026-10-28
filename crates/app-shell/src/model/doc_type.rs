use dioxus::prelude::*;

use super::placement::Placement;
use super::rail_item::RailItem;
use super::tool_group::ToolGroup;
use super::view::View;
use crate::icons::Icon;

/// A *kind* of document (Part Studio, Calc Sheet, Drawing...). It decides
/// everything inside the main window: toolbar, the three inner rails,
/// the viewport, and an optional status readout in the bottom rail.
#[derive(Clone, PartialEq, Debug)]
#[non_exhaustive]
pub struct DocType {
    pub id: &'static str,
    pub label: &'static str,
    pub icon: Icon,
    pub toolbar: Vec<ToolGroup>,
    pub left_rail: Vec<RailItem>,
    pub right_rail: Vec<RailItem>,
    pub bottom_rail: Vec<RailItem>,
    pub viewport: View,
    pub status: Option<View>,
    /// Panels open the first time a document of this type is shown.
    pub default_left: Option<&'static str>,
    pub default_right: Option<&'static str>,
}

impl DocType {
    pub fn new(id: &'static str, label: &'static str, icon: Icon, viewport: fn() -> Element) -> Self {
        Self {
            id,
            label,
            icon,
            toolbar: vec![],
            left_rail: vec![],
            right_rail: vec![],
            bottom_rail: vec![],
            viewport: View(viewport),
            status: None,
            default_left: None,
            default_right: None,
        }
    }

    pub fn rail(&self, placement: Placement) -> &[RailItem] {
        match placement {
            Placement::Left => &self.left_rail,
            Placement::Right => &self.right_rail,
            Placement::Bottom => &self.bottom_rail,
            Placement::Nav => &[],
        }
    }
}
