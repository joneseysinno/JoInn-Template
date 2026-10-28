use dioxus::prelude::*;

use super::shell::Shell;

/// Grab the shell handle from any view inside an `AppShell`.
pub fn use_shell() -> Shell {
    use_context::<Shell>()
}
