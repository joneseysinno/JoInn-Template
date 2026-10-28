use dioxus::prelude::*;

use super::act::Act;
use crate::state::Shell;

pub(super) fn exec(mut shell: Shell, act: Act) {
    shell.palette_open.set(false);
    match act {
        Act::Cmd(id) => shell.run(id),
        Act::Panel(p, id) => shell.set_panel(p, Some(id)),
        Act::Open(id) => shell.activate(&id),
        Act::New(kind) => {
            shell.new_document(kind);
        }
    }
    shell.focus_root();
}
