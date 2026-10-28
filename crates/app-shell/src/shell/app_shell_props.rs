use dioxus::prelude::*;

use crate::model::ShellSpec;

#[derive(Props, Clone)]
pub struct AppShellProps {
    /// Builds the app description. Called once, when the shell mounts.
    pub spec: fn() -> ShellSpec,
}

impl PartialEq for AppShellProps {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::fn_addr_eq(self.spec, other.spec)
    }
}
