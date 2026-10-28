/// The document a view belongs to (provided by the shell around every
/// viewport, inner panel and status readout).
#[derive(Clone, PartialEq, Debug)]
pub struct DocId(pub String);
