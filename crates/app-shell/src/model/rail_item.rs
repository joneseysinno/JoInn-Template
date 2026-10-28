use dioxus::prelude::*;

use super::align::Align;
use super::rail_action::RailAction;
use super::view::View;
use crate::icons::Icon;

#[derive(Clone, PartialEq, Debug)]
#[non_exhaustive]
pub struct RailItem {
    pub id: &'static str,
    pub label: &'static str,
    pub icon: Icon,
    pub action: RailAction,
    pub align: Align,
}

impl RailItem {
    pub fn panel(id: &'static str, label: &'static str, icon: Icon, view: fn() -> Element) -> Self {
        Self { id, label, icon, action: RailAction::Panel(View(view)), align: Align::Start }
    }
    pub fn command(id: &'static str, label: &'static str, icon: Icon) -> Self {
        Self { id, label, icon, action: RailAction::Command, align: Align::Start }
    }
    /// Push this item to the far end of its rail.
    pub fn at_end(mut self) -> Self {
        self.align = Align::End;
        self
    }
}
