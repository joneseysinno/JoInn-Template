use super::command_event::CommandEvent;
use super::doc_type::DocType;
use super::document::Document;
use super::rail_item::RailItem;
use super::theme::Theme;
use super::tool::Tool;
use crate::icons::Icon;

/// Everything that makes one app different from another.
#[derive(Clone)]
#[non_exhaustive]
pub struct ShellSpec {
    pub app_name: &'static str,
    pub logo: Icon,
    /// Small grey text after the document name (workspace, branch, folder).
    pub context_label: Option<&'static str>,
    /// Right side of the title bar (notifications, share, account...).
    pub title_actions: Vec<Tool>,
    /// Far-left global rail.
    pub nav_rail: Vec<RailItem>,
    pub default_nav: Option<&'static str>,
    pub doc_types: Vec<DocType>,
    pub documents: Vec<Document>,
    pub active_document: Option<String>,
    pub theme: Theme,
    /// Called after every command, with the shell handle.
    pub on_command: Option<fn(crate::Shell, &CommandEvent)>,
}

impl ShellSpec {
    pub fn new(app_name: &'static str) -> Self {
        Self {
            app_name,
            logo: crate::icons::LOGO,
            context_label: None,
            title_actions: vec![],
            nav_rail: vec![],
            default_nav: None,
            doc_types: vec![],
            documents: vec![],
            active_document: None,
            theme: Theme::Dark,
            on_command: None,
        }
    }

    pub fn doc_type(&self, id: &str) -> Option<&DocType> {
        self.doc_types.iter().find(|d| d.id == id)
    }
}
