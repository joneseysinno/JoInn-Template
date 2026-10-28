use super::act::Act;
use super::entry::Entry;
use crate::model::{Placement, RailAction, RailItem};

pub(super) fn rail_entries(out: &mut Vec<Entry>, items: &[RailItem], p: Placement, group: &'static str) {
    for i in items {
        out.push(Entry {
            label: i.label.to_string(),
            group,
            hint: None,
            icon: i.icon,
            act: match i.action {
                RailAction::Panel(_) => Act::Panel(p, i.id),
                RailAction::Command => Act::Cmd(i.id),
            },
        });
    }
}
