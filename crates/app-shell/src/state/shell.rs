use std::collections::HashMap;
use std::rc::Rc;

use dioxus::prelude::*;

use super::cmd;
use super::drag::Drag;
use super::slot_state::SlotState;
use crate::model::{CommandEvent, DocType, Document, Placement, ShellSpec, Theme};

#[derive(Clone, Copy, PartialEq)]
pub struct Shell {
    pub spec: Signal<Rc<ShellSpec>>,
    pub nav_pane: Signal<Option<&'static str>>,
    pub slots: Signal<HashMap<&'static str, SlotState>>,
    pub nav_width: Signal<f64>,
    pub left_width: Signal<f64>,
    pub right_width: Signal<f64>,
    pub bottom_height: Signal<f64>,
    pub documents: Signal<Vec<Document>>,
    pub active: Signal<Option<String>>,
    pub theme: Signal<Theme>,
    pub palette_open: Signal<bool>,
    pub history: Signal<Vec<CommandEvent>>,
    pub drag: Signal<Option<Drag>>,
    seq: Signal<u64>,
    pub(crate) root_el: Signal<Option<Rc<MountedData>>>,
}

impl Shell {
    pub(crate) fn new(spec: ShellSpec) -> Self {
        let active = spec
            .active_document
            .clone()
            .or_else(|| spec.documents.first().map(|d| d.id.clone()));
        Self {
            nav_pane: Signal::new(spec.default_nav),
            slots: Signal::new(HashMap::new()),
            nav_width: Signal::new(240.0),
            left_width: Signal::new(260.0),
            right_width: Signal::new(280.0),
            bottom_height: Signal::new(180.0),
            documents: Signal::new(spec.documents.clone()),
            active: Signal::new(active),
            theme: Signal::new(spec.theme),
            palette_open: Signal::new(false),
            history: Signal::new(vec![]),
            drag: Signal::new(None),
            seq: Signal::new(0),
            root_el: Signal::new(None),
            spec: Signal::new(Rc::new(spec)),
        }
    }

    // ---------- documents ----------

    pub fn active_document(&self) -> Option<Document> {
        let id = self.active.read().clone()?;
        self.documents.read().iter().find(|d| d.id == id).cloned()
    }

    pub fn active_type(&self) -> Option<DocType> {
        let doc = self.active_document()?;
        self.spec.read().doc_type(doc.kind).cloned()
    }

    pub fn activate(&mut self, id: &str) {
        self.active.set(Some(id.to_string()));
    }

    pub fn new_document(&mut self, kind: &'static str) -> String {
        let label = self
            .spec
            .read()
            .doc_type(kind)
            .map(|t| t.label)
            .unwrap_or("Document");
        let n = self.documents.read().iter().filter(|d| d.kind == kind).count() + 1;
        let id = format!("{kind}-{}", *self.seq.peek() + 1000);
        *self.seq.write() += 1;
        self.documents
            .write()
            .push(Document::new(id.clone(), format!("{label} {n}"), kind));
        self.activate(&id);
        id
    }

    pub fn close_document(&mut self, id: &str) {
        let mut docs = self.documents.write();
        let Some(ix) = docs.iter().position(|d| d.id == id) else { return };
        docs.remove(ix);
        let next = docs.get(ix.min(docs.len().saturating_sub(1))).map(|d| d.id.clone());
        drop(docs);
        if self.active.read().as_deref() == Some(id) {
            self.active.set(next);
        }
    }

    pub fn cycle_document(&mut self, step: isize) {
        let docs = self.documents.read();
        if docs.is_empty() {
            return;
        }
        let cur = self.active.read().clone();
        let ix = cur
            .and_then(|c| docs.iter().position(|d| d.id == c))
            .unwrap_or(0) as isize;
        let n = docs.len() as isize;
        let next = docs[((ix + step) % n + n) as usize % n as usize].id.clone();
        drop(docs);
        self.active.set(Some(next));
    }

    // ---------- panels ----------

    /// Panel open in an inner slot for the current document type.
    pub fn slot(&self, placement: Placement) -> Option<&'static str> {
        if placement == Placement::Nav {
            return *self.nav_pane.read();
        }
        let ty = self.active_type()?;
        let slots = self.slots.read();
        let s = slots.get(ty.id).cloned().unwrap_or(SlotState {
            left: ty.default_left,
            right: ty.default_right,
            bottom: None,
        });
        match placement {
            Placement::Left => s.left,
            Placement::Right => s.right,
            Placement::Bottom => s.bottom,
            Placement::Nav => None,
        }
    }

    /// Open `id` in `placement`, or close it if it is already open.
    pub fn toggle_panel(&mut self, placement: Placement, id: &'static str) {
        let next = if self.slot(placement) == Some(id) { None } else { Some(id) };
        self.set_panel(placement, next);
    }

    pub fn set_panel(&mut self, placement: Placement, id: Option<&'static str>) {
        if placement == Placement::Nav {
            self.nav_pane.set(id);
            return;
        }
        let Some(ty) = self.active_type() else { return };
        let mut slots = self.slots.write();
        let s = slots.entry(ty.id).or_insert(SlotState {
            left: ty.default_left,
            right: ty.default_right,
            bottom: None,
        });
        match placement {
            Placement::Left => s.left = id,
            Placement::Right => s.right = id,
            Placement::Bottom => s.bottom = id,
            Placement::Nav => {}
        }
    }

    // ---------- commands ----------

    /// Run a command by id. Shell built-ins are handled here; everything is
    /// recorded in `history` and forwarded to `ShellSpec::on_command`.
    pub fn run(&mut self, id: &'static str) {
        match id {
            cmd::PALETTE => self.palette_open.set(true),
            cmd::TOGGLE_NAV => {
                let cur = *self.nav_pane.read();
                let fallback = self.spec.read().nav_rail.first().map(|r| r.id);
                self.nav_pane.set(if cur.is_some() { None } else { fallback });
            }
            cmd::TOGGLE_THEME => {
                let t = *self.theme.read();
                self.theme.set(if t == Theme::Dark { Theme::Light } else { Theme::Dark });
            }
            cmd::NEXT_DOC => self.cycle_document(1),
            cmd::PREV_DOC => self.cycle_document(-1),
            cmd::CLOSE_DOC => {
                let cur = self.active.read().clone();
                if let Some(d) = cur {
                    self.close_document(&d);
                }
            }
            _ => {}
        }
        let seq = *self.seq.peek() + 1;
        self.seq.set(seq);
        let ev = CommandEvent { seq, id, doc: self.active.read().clone() };
        {
            let mut h = self.history.write();
            h.push(ev.clone());
            let len = h.len();
            if len > 500 {
                h.drain(..len - 500);
            }
        }
        let hook = self.spec.read().on_command;
        if let Some(f) = hook {
            f(*self, &ev);
        }
    }

    pub fn last_command(&self) -> Option<CommandEvent> {
        self.history.read().last().cloned()
    }

    /// Return keyboard focus to the shell (after a dialog closes).
    pub fn focus_root(&self) {
        if let Some(el) = self.root_el.read().clone() {
            spawn(async move {
                let _ = el.set_focus(true).await;
            });
        }
    }

    // ---------- splitters ----------

    pub(crate) fn size_signal(&self, p: Placement) -> Signal<f64> {
        match p {
            Placement::Nav => self.nav_width,
            Placement::Left => self.left_width,
            Placement::Right => self.right_width,
            Placement::Bottom => self.bottom_height,
        }
    }
}
