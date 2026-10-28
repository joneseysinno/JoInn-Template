use dioxus::prelude::*;

/// A piece of UI supplied by the app (a panel body, a viewport, a status
/// readout). It is a plain `fn() -> Element`, so it may use hooks freely:
/// the shell always mounts it inside its own keyed component scope.
#[derive(Clone, Copy)]
pub struct View(pub fn() -> Element);

impl PartialEq for View {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::fn_addr_eq(self.0, other.0)
    }
}

impl std::fmt::Debug for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("View(fn)")
    }
}
