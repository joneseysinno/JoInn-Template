use super::tool::Tool;

#[derive(Clone, PartialEq, Debug)]
#[non_exhaustive]
pub struct ToolGroup {
    pub label: &'static str,
    pub tools: Vec<Tool>,
}

impl ToolGroup {
    pub fn new(label: &'static str, tools: Vec<Tool>) -> Self {
        Self { label, tools }
    }
}
