//! Engineering worksheet document type: same shell, completely different
//! toolbar, rails and viewport.

use std::collections::HashMap;

use app_shell::prelude::*;
use dioxus::prelude::*;

pub const ID: &str = "calc-sheet";

/// Example inputs for a uniform bearing-pressure check (user editable).
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Calc {
    pub p_kip: f64,
    pub b_ft: f64,
    pub l_ft: f64,
    pub qa_ksf: f64,
}

impl Default for Calc {
    fn default() -> Self {
        Self { p_kip: 120.0, b_ft: 8.0, l_ft: 16.0, qa_ksf: 2.0 }
    }
}

impl Calc {
    pub fn area(&self) -> f64 {
        self.b_ft * self.l_ft
    }
    pub fn q(&self) -> f64 {
        if self.area() > 0.0 { self.p_kip / self.area() } else { f64::NAN }
    }
    pub fn ratio(&self) -> f64 {
        self.q() / self.qa_ksf
    }
}

pub static CALCS: GlobalSignal<HashMap<String, Calc>> = Signal::global(HashMap::new);

fn calc(doc: &str) -> Calc {
    CALCS.read().get(doc).copied().unwrap_or_default()
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Field {
    P,
    B,
    L,
    Qa,
}

pub fn doc_type() -> DocType {
    let mut t = DocType::new(ID, "Calc Sheet", icons::FX, Viewport);
    t.toolbar = vec![
        ToolGroup::new("Insert", vec![
            Tool::new("cs.eq", "Insert equation", icons::FX).key("E"),
            Tool::new("cs.table", "Insert table", icons::TABLE),
            Tool::new("cs.var", "Insert variable", icons::VARIABLE),
        ]),
        ToolGroup::new("Solve", vec![Tool::new("cs.run", "Recalculate", icons::PLAY).primary()]),
        ToolGroup::new("Output", vec![Tool::new("cs.export", "Export PDF", icons::DOWNLOAD)]),
    ];
    t.left_rail = vec![RailItem::panel("cs.outline", "Outline", icons::TREE, Outline)];
    t.right_rail = vec![RailItem::panel("cs.vars", "Variables", icons::VARIABLE, Variables)];
    t.bottom_rail = vec![RailItem::panel("cs.console", "Console", icons::TERMINAL, super::ConsolePane)];
    t.status = Some(View(Status));
    t.default_left = Some("cs.outline");
    t.default_right = Some("cs.vars");
    t
}

#[component]
fn NumField(doc: String, field: Field, label: &'static str, sym: &'static str, unit: &'static str) -> Element {
    let c = calc(&doc);
    let v = match field {
        Field::P => c.p_kip,
        Field::B => c.b_ft,
        Field::L => c.l_ft,
        Field::Qa => c.qa_ksf,
    };
    let input_id = format!("{doc}-{sym}");
    rsx! {
        div { class: "demo-calc-row",
            label { r#for: "{input_id}", span { class: "demo-sym", "{sym}" } " {label}" }
            input {
                id: "{input_id}",
                r#type: "number",
                step: "any",
                value: "{v}",
                oninput: move |e| {
                    if let Ok(x) = e.value().parse::<f64>() {
                        let mut all = CALCS.write();
                        let c = all.entry(doc.clone()).or_default();
                        match field {
                            Field::P => c.p_kip = x,
                            Field::B => c.b_ft = x,
                            Field::L => c.l_ft = x,
                            Field::Qa => c.qa_ksf = x,
                        }
                    }
                },
            }
            span { class: "demo-muted", "{unit}" }
        }
    }
}

#[allow(non_snake_case)]
fn Viewport() -> Element {
    let doc = use_document();
    let id = doc.as_ref().map(|d| d.id.clone()).unwrap_or_default();
    let name = doc.map(|d| d.name).unwrap_or_default();
    let c = calc(&id);
    let ok = c.ratio() <= 1.0;
    rsx! {
        div { class: "demo-sheet-wrap",
            article { class: "demo-sheet",
                h2 { "{name}" }
                p { class: "demo-muted", "Uniform bearing pressure under a mat. Example inputs; edit to recalculate." }
                h3 { id: "{id}-inputs", "1. Inputs" }
                NumField { doc: id.clone(), field: Field::P, label: "Total load", sym: "P", unit: "kip" }
                NumField { doc: id.clone(), field: Field::B, label: "Mat width", sym: "B", unit: "ft" }
                NumField { doc: id.clone(), field: Field::L, label: "Mat length", sym: "L", unit: "ft" }
                NumField { doc: id.clone(), field: Field::Qa, label: "Allowable bearing", sym: "qₐ", unit: "ksf" }
                h3 { id: "{id}-calc", "2. Bearing pressure" }
                div { class: "demo-eq", "A = B · L = {c.b_ft} · {c.l_ft} = " strong { "{c.area():.2} ft²" } }
                div { class: "demo-eq", "q = P / A = {c.p_kip} / {c.area():.2} = " strong { "{c.q():.3} ksf" } }
                h3 { id: "{id}-check", "3. Check" }
                div { class: "demo-eq",
                    "q / qₐ = {c.ratio():.3} "
                    span { class: if ok { "demo-ok" } else { "demo-ng" }, if ok { "≤ 1.0  OK" } else { "> 1.0  NG" } }
                }
            }
        }
    }
}

#[allow(non_snake_case)]
fn Outline() -> Element {
    rsx! {
        ul { class: "demo-tree",
            li { "1. Inputs" }
            li { "2. Bearing pressure" }
            li { "3. Check" }
        }
    }
}

#[allow(non_snake_case)]
fn Variables() -> Element {
    let id = use_document().map(|d| d.id).unwrap_or_default();
    let c = calc(&id);
    rsx! {
        dl { class: "demo-props",
            dt { "P" } dd { "{c.p_kip} kip" }
            dt { "B" } dd { "{c.b_ft} ft" }
            dt { "L" } dd { "{c.l_ft} ft" }
            dt { "qₐ" } dd { "{c.qa_ksf} ksf" }
            dt { "A" } dd { "{c.area():.2} ft²" }
            dt { "q" } dd { "{c.q():.3} ksf" }
            dt { "q/qₐ" } dd { "{c.ratio():.3}" }
        }
    }
}

#[allow(non_snake_case)]
fn Status() -> Element {
    let id = use_document().map(|d| d.id).unwrap_or_default();
    let c = calc(&id);
    let ok = c.ratio() <= 1.0;
    rsx! {
        span { "q = {c.q():.3} ksf" }
        span { "{c.ratio() * 100.0:.0}% of qₐ" }
        span { class: if ok { "demo-ok" } else { "demo-ng" }, if ok { "OK" } else { "NG" } }
    }
}
