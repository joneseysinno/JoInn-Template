use dioxus::prelude::*;

use super::doc_id::DocId;
use crate::model::Document;
use crate::state::use_shell;

/// The document *this* view was mounted for (not necessarily the active
/// one: hidden keep-alive viewports still know who they are).
pub fn use_document() -> Option<Document> {
    let shell = use_shell();
    let id = try_use_context::<DocId>();
    match id {
        Some(DocId(id)) => shell.documents.read().iter().find(|d| d.id == id).cloned(),
        None => shell.active_document(),
    }
}
