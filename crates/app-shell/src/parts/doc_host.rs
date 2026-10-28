use dioxus::prelude::*;

use super::doc_id::DocId;
use crate::model::View;

#[component]
pub(crate) fn DocHost(doc_id: String, view: View) -> Element {
    use_context_provider(|| DocId(doc_id.clone()));
    (view.0)()
}
