//! CAD-style document type: modeling toolbar, feature tree, viewport.

use std::collections::HashMap;

use app_shell::prelude::*;
use dioxus::prelude::*;

pub const ID: &str = "part-studio";

#[derive(Clone, PartialEq, Debug)]
pub struct PartState {
    pub features: Vec<String>,
    pub view: &'static str,
    pub color: &'static str,
}

impl Default for PartState {
    fn default() -> Self {
        Self { features: vec![], view: "iso", color: "#d9771e" }
    }
}

pub static PARTS: GlobalSignal<HashMap<String, PartState>> = Signal::global(HashMap::new);

fn state(doc: &str) -> PartState {
    PARTS.read().get(doc).cloned().unwrap_or_default()
}

pub fn doc_type() -> DocType {
    let mut t = DocType::new(ID, "Part Studio", icons::CUBE, Viewport);
    t.toolbar = vec![
        ToolGroup::new("Edit", vec![
            Tool::new("ps.undo", "Undo", icons::UNDO),
            Tool::new("ps.redo", "Redo", icons::REDO),
        ]),
        ToolGroup::new("Sketch", vec![Tool::new("ps.sketch", "Sketch", icons::PENCIL).key("Shift+S")]),
        ToolGroup::new("Features", vec![
            Tool::new("ps.extrude", "Extrude", icons::EXTRUDE).key("Shift+E"),
            Tool::new("ps.revolve", "Revolve", icons::REVOLVE).key("Shift+W"),
            Tool::new("ps.fillet", "Fillet", icons::FILLET).key("Shift+F"),
            Tool::new("ps.chamfer", "Chamfer", icons::CHAMFER),
            Tool::new("ps.shell", "Shell", icons::SHELL),
        ]),
        ToolGroup::new("Patterns", vec![
            Tool::new("ps.pattern", "Linear pattern", icons::PATTERN),
            Tool::new("ps.mirror", "Mirror", icons::MIRROR),
        ]),
        ToolGroup::new("Variables", vec![Tool::new("ps.variable", "Variable", icons::VARIABLE)]),
    ];
    t.left_rail = vec![
        RailItem::panel("ps.tree", "Feature tree", icons::TREE, FeatureTree),
        RailItem::panel("ps.parts", "Parts", icons::LAYERS, PartsList),
    ];
    t.right_rail = vec![
        RailItem::panel("ps.appearance", "Appearance", icons::DROPLET, AppearancePane),
        RailItem::panel("ps.props", "Properties", icons::INFO, PropertiesPane),
        RailItem::command("ps.measure", "Measure", icons::RULER).at_end(),
    ];
    t.bottom_rail = vec![
        RailItem::panel("ps.console", "Console", icons::TERMINAL, super::ConsolePane),
        RailItem::command("ps.view.iso", "Iso", icons::CUBE).at_end(),
        RailItem::command("ps.view.top", "Top", icons::GRID).at_end(),
        RailItem::command("ps.view.front", "Front", icons::EYE).at_end(),
    ];
    t.status = Some(View(Status));
    t.default_left = Some("ps.tree");
    t
}

/// Command handler for this document type.
pub fn handle(doc: &str, id: &'static str) {
    let mut parts = PARTS.write();
    let st = parts.entry(doc.to_string()).or_default();
    let name = match id {
        "ps.sketch" => "Sketch",
        "ps.extrude" => "Extrude",
        "ps.revolve" => "Revolve",
        "ps.fillet" => "Fillet",
        "ps.chamfer" => "Chamfer",
        "ps.shell" => "Shell",
        "ps.pattern" => "Linear pattern",
        "ps.mirror" => "Mirror",
        "ps.variable" => "Variable",
        "ps.undo" => {
            st.features.pop();
            return;
        }
        "ps.view.iso" => return st.view = "iso",
        "ps.view.top" => return st.view = "top",
        "ps.view.front" => return st.view = "front",
        _ => return,
    };
    let n = st.features.iter().filter(|f| f.starts_with(name)).count() + 1;
    st.features.push(format!("{name} {n}"));
}

// ---------------------------------------------------------------------------
// Viewport: a 6 ft x 10 ft x 8 in slab, projected three ways.
// ---------------------------------------------------------------------------

type P3 = (f64, f64, f64);

fn project(view: &str, (x, y, z): P3) -> (f64, f64) {
    match view {
        "top" => (x, y),
        "front" => (x, -z),
        _ => ((x - y) * 0.866, (x + y) * 0.5 - z),
    }
}

fn faces(view: &str) -> Vec<(Vec<P3>, &'static str)> {
    let (l, w, h) = (120.0, 72.0, 8.0);
    let top = vec![(0.0, 0.0, h), (l, 0.0, h), (l, w, h), (0.0, w, h)];
    match view {
        "top" => vec![(top, "top")],
        "front" => vec![(vec![(0.0, 0.0, 0.0), (l, 0.0, 0.0), (l, 0.0, h), (0.0, 0.0, h)], "side")],
        _ => vec![
            (vec![(l, 0.0, 0.0), (l, w, 0.0), (l, w, h), (l, 0.0, h)], "end"),
            (vec![(0.0, w, 0.0), (l, w, 0.0), (l, w, h), (0.0, w, h)], "side"),
            (top, "top"),
        ],
    }
}

#[allow(non_snake_case)]
fn Viewport() -> Element {
    let mut shell = use_shell();
    let doc = use_document();
    let id = doc.as_ref().map(|d| d.id.clone()).unwrap_or_default();
    let st = state(&id);

    // Fit projected geometry into an 800 x 500 box.
    let polys = faces(st.view);
    let pts: Vec<(f64, f64)> = polys.iter().flat_map(|(f, _)| f.iter().map(|p| project(st.view, *p))).collect();
    let (minx, maxx) = pts.iter().fold((f64::MAX, f64::MIN), |a, p| (a.0.min(p.0), a.1.max(p.0)));
    let (miny, maxy) = pts.iter().fold((f64::MAX, f64::MIN), |a, p| (a.0.min(p.1), a.1.max(p.1)));
    let k = (640.0 / (maxx - minx)).min(340.0 / (maxy - miny));
    let ox = 400.0 - (minx + maxx) / 2.0 * k;
    let oy = 250.0 - (miny + maxy) / 2.0 * k;
    let shapes: Vec<(String, &'static str)> = polys
        .iter()
        .map(|(f, kind)| {
            let s = f
                .iter()
                .map(|p| {
                    let (x, y) = project(st.view, *p);
                    format!("{:.1},{:.1}", ox + x * k, oy + y * k)
                })
                .collect::<Vec<_>>()
                .join(" ");
            (s, *kind)
        })
        .collect();
    let last = shell
        .history
        .read()
        .iter()
        .rev()
        .find(|e| e.doc.as_deref() == Some(id.as_str()) && !e.id.starts_with("shell."))
        .cloned();

    rsx! {
        div { class: "demo-vp",
            svg {
                class: "demo-vp-svg",
                view_box: "0 0 800 500",
                preserve_aspect_ratio: "xMidYMid meet",
                role: "img",
                "aria-label": "{st.view} view of the part",
                for (i, (pts, kind)) in shapes.iter().enumerate() {
                    polygon {
                        key: "{i}",
                        points: "{pts}",
                        class: "demo-face demo-face-{kind}",
                        style: if *kind == "top" { format!("fill:{}", st.color) } else { String::new() },
                    }
                }
            }
            div { class: "demo-viewcube", role: "group", "aria-label": "Views",
                for (v , label) in [("iso", "Iso"), ("top", "Top"), ("front", "Front")] {
                    button {
                        key: "{v}",
                        class: if st.view == v { "as-active" } else { "" },
                        onclick: move |_| shell.run(match v { "top" => "ps.view.top", "front" => "ps.view.front", _ => "ps.view.iso" }),
                        "{label}"
                    }
                }
            }
            if let Some(ev) = last {
                div { class: "demo-chip", key: "{ev.seq}", "Ran ", code { "{ev.id}" } }
            }
        }
    }
}

#[allow(non_snake_case)]
fn FeatureTree() -> Element {
    let id = use_document().map(|d| d.id).unwrap_or_default();
    let st = state(&id);
    rsx! {
        ul { class: "demo-tree",
            li { class: "demo-muted", "Origin" }
            li { class: "demo-muted", "Top plane" }
            li { class: "demo-muted", "Front plane" }
            li { class: "demo-muted", "Right plane" }
            for (i, f) in st.features.iter().enumerate() {
                li { key: "{i}", "{f}" }
            }
            if st.features.is_empty() {
                li { class: "demo-hint", "Use the toolbar (or Shift+E) to add features." }
            }
        }
    }
}

#[allow(non_snake_case)]
fn PartsList() -> Element {
    let name = use_document().map(|d| d.name).unwrap_or_default();
    rsx! {
        ul { class: "demo-tree",
            li { "{name} body" }
        }
    }
}

#[allow(non_snake_case)]
fn AppearancePane() -> Element {
    let id = use_document().map(|d| d.id).unwrap_or_default();
    let current = state(&id).color;
    let swatches = [("#d9771e", "Orange"), ("#2f6fdb", "Blue"), ("#8a8f98", "Steel"), ("#3f9d5b", "Green")];
    rsx! {
        div { class: "demo-pad",
            p { class: "demo-muted", "Top face color" }
            div { class: "demo-swatches",
                for (hex , name) in swatches {
                    button {
                        key: "{hex}",
                        class: if current == hex { "demo-swatch demo-on" } else { "demo-swatch" },
                        style: "background:{hex}",
                        "aria-label": "{name}",
                        title: "{name}",
                        onclick: {
                            let id = id.clone();
                            move |_| PARTS.write().entry(id.clone()).or_default().color = hex
                        },
                    }
                }
            }
        }
    }
}

#[allow(non_snake_case)]
fn PropertiesPane() -> Element {
    let doc = use_document();
    let id = doc.as_ref().map(|d| d.id.clone()).unwrap_or_default();
    let st = state(&id);
    let name = doc.map(|d| d.name).unwrap_or_default();
    rsx! {
        dl { class: "demo-props",
            dt { "Name" } dd { "{name}" }
            dt { "Size" } dd { "120 × 72 × 8 in" }
            dt { "Features" } dd { "{st.features.len()}" }
            dt { "View" } dd { "{st.view}" }
        }
    }
}

#[allow(non_snake_case)]
fn Status() -> Element {
    let id = use_document().map(|d| d.id).unwrap_or_default();
    let st = state(&id);
    rsx! {
        span { "in · lbf" }
        span { "{st.features.len()} features" }
        span { "view: {st.view}" }
    }
}
