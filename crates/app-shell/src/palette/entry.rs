use super::act::Act;
use crate::icons::Icon;

#[derive(Clone, PartialEq, Debug)]
pub(super) struct Entry {
    pub(super) label: String,
    pub(super) group: &'static str,
    pub(super) hint: Option<&'static str>,
    pub(super) icon: Icon,
    pub(super) act: Act,
}
