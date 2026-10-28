/// Which panel each inner slot shows, remembered per document type
/// (so a Part Studio keeps its feature tree open while a Calc Sheet
/// keeps its outline open).
#[derive(Clone, Default, PartialEq, Debug)]
pub struct SlotState {
    pub left: Option<&'static str>,
    pub right: Option<&'static str>,
    pub bottom: Option<&'static str>,
}
