/// A command that ran. Doc views can watch [`crate::Shell::last_command`]
/// or the history to react; the app can also hook `ShellSpec::on_command`.
#[derive(Clone, PartialEq, Debug)]
pub struct CommandEvent {
    pub seq: u64,
    pub id: &'static str,
    pub doc: Option<String>,
}
