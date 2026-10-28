use crate::icons::Icon;

/// A toolbar button. Every tool is also a command-palette entry.
#[derive(Clone, PartialEq, Debug)]
#[non_exhaustive]
pub struct Tool {
    pub id: &'static str,
    pub label: &'static str,
    pub icon: Icon,
    pub shortcut: Option<&'static str>,
    /// Rendered with the accent style (e.g. "Share", "Run").
    pub primary: bool,
}

impl Tool {
    pub fn new(id: &'static str, label: &'static str, icon: Icon) -> Self {
        Self { id, label, icon, shortcut: None, primary: false }
    }
    pub fn key(mut self, shortcut: &'static str) -> Self {
        self.shortcut = Some(shortcut);
        self
    }
    pub fn primary(mut self) -> Self {
        self.primary = true;
        self
    }
}
