use super::use_shell::use_shell;
use crate::model::Document;

/// Convenience: the active document (reactive).
pub fn use_active_document() -> Option<Document> {
    use_shell().active_document()
}
