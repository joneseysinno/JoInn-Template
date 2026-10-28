# dioxus-app-shell

A reusable, Onshape-inspired application frame for **Dioxus 0.7**. Describe
your app as data, hand it to `AppShell`, and you get the full layout:

```text
┌──────────────────────────── Title bar ───────────────────────────────┐
├────┬───────────┬──────────────── Main window ────────────────────────┤
│Nav │ Nav pane  │ Toolbar  (from the active DocType)       [Search ⌘K]│
│rail│           ├─────────┬────────────────────────────────┬──────────┤
│    │ Documents │ Left    │                                │ Right    │
│ ▣  │ Activity  │ panel   │▌      Viewport (per doc)      ▐│ panel    │
│ ◷  │ Settings  │         │▌  tabs float over the edges   ▐│          │
│    │ ...       │         ├────────────────────────────────┤          │
│    │           │         │  Bottom panel                  │          │
│ ⚙  │           ├─────────┴────────────────────────────────┴──────────┤
│    │           │ Bottom rail: [Console] status readout  [Iso|Top|Front]
└────┴───────────┴───────────────────────────────────────────────────────┘
```

No bottom document tabs: the **Documents** nav pane, `Alt+[` / `Alt+]` and
the command palette replace them.

## Run the demo

```bash
cargo run -p demo                 # desktop (default)
dx serve -p demo --platform web --no-default-features --features web
```

The demo is two unrelated document types in one shell: a **Part Studio**
(modeling toolbar, feature tree, isometric/top/front viewport, appearance
pane) and a **Calc Sheet** (live bearing-pressure worksheet, outline,
variables). Neither the shell nor the other type knows anything about them.

## Start a new app

`template/` is a [cargo-generate](https://cargo-generate.github.io/cargo-generate/)
template. It creates a standalone app that depends on this repo's `app-shell`
crate at a git tag, so every app gets the same layout:

```bash
cargo install cargo-generate          # once
cargo generate --git https://github.com/joneseysinno/JoInn-Template template --name my-app
# or, with the Dioxus CLI:
dx new my-app --template https://github.com/joneseysinno/JoInn-Template --subtemplate template
```

You are asked for the app title, the `app-shell` git URL and the tag to pin.
The generated app has one example document type to rename or copy; see its
README for where things go.

To upgrade an app's shell, change `tag` on its `app-shell` dependency, then
run `cargo update -p app-shell`. Breaking shell changes get a new minor
version tag (`v0.2.0`) while the crate is below 1.0.

To work on the shell and an app at the same time, point the app at your local
checkout without touching its dependency line:

```toml
# in the app's Cargo.toml
[patch."https://github.com/joneseysinno/JoInn-Template"]
app-shell = { path = "D:/Rust/Dioxus/JoInn Template/crates/app-shell" }
```

## Mental model

| Concept | What it is |
|---|---|
| `ShellSpec` | The whole app as data: brand, title actions, nav rail, doc types, initial documents, `on_command` hook. |
| `DocType` | A *kind* of document. Owns the toolbar, left/right/bottom rails, viewport and optional status readout. |
| `Document` | An open instance (`id`, `name`, `kind`). |
| `RailItem` | An icon. `panel(...)` toggles that rail's panel slot; `command(...)` fires a command. `.at_end()` pushes it to the far end. |
| `Tool` / `ToolGroup` | Toolbar buttons, grouped with separators. `.key("Shift+E")` adds a shortcut, `.primary()` the accent style. |
| `View` | Any `fn() -> Element`. The shell mounts each one in its own keyed scope, so views can use hooks freely. |
| `Shell` | A `Copy` handle of signals in context: `use_shell()` from any view to open panels, run commands, switch documents, read history. |
| `use_document()` | The document *this* view belongs to (works in hidden keep-alive viewports too). |

## Add a document type

```rust
use app_shell::prelude::*;
use dioxus::prelude::*;

pub fn drawing() -> DocType {
    let mut t = DocType::new("drawing", "Drawing", icons::RULER, Sheet);
    t.toolbar = vec![ToolGroup::new("Annotate", vec![
        Tool::new("dw.dim", "Dimension", icons::RULER).key("D"),
    ])];
    t.left_rail = vec![RailItem::panel("dw.sheets", "Sheets", icons::LAYERS, Sheets)];
    t.default_left = Some("dw.sheets");
    t
}

#[allow(non_snake_case)]
fn Sheet() -> Element {
    let doc = use_document();
    rsx! { "Drawing for {doc.map(|d| d.name).unwrap_or_default()}" }
}
// fn Sheets() -> Element { ... }  // the left-panel view
```

Then `spec.doc_types.push(drawing())`. It shows up in the Documents pane's
"+" menu, the empty state and the palette ("New Drawing") automatically.

## What the shell gives you for free

- **Command palette** (`Ctrl/⌘+K`), *generated* from every tool, rail item,
  title action, open document and doc type. Subsequence matching.
- **One command bus**: toolbar, rails, shortcuts and palette all call
  `shell.run(id)`. Every run is recorded in `shell.history` (an undo stack or
  macro recorder is a short step from here) and passed to
  `ShellSpec::on_command`.
- **Panel memory per document type**: the Part Studio remembers its feature
  tree open; the Calc Sheet remembers outline + variables.
- **Keep-alive viewports**: every open document stays mounted (hidden), so
  switching documents keeps scroll position, camera, local state.
- **Resizable panels** (drag the inner edge), **dark/light theme** from CSS
  tokens, narrow-window overlay panels, keyboard focus rings, ARIA labels.
- **Renderer-agnostic**: the `app-shell` crate depends only on Dioxus core
  features; the app picks desktop, web or mobile.

## Layout

```
crates/app-shell/src/
  lib.rs       public API + prelude
  model.rs     ShellSpec, DocType, RailItem, Tool, View, Placement...
  state.rs     Shell handle (signals), commands, panel slots
  shell.rs     AppShell grid, keyboard, splitter drag, main window
  parts.rs     TitleBar, Rail, PanelSlot, Toolbar, DocumentsPane, SettingsPane
  palette.rs   generated command palette
  icons.rs     tiny stroke icon set (Icon = SVG path data)
  shell.css    all styling, driven by --as-* tokens
demo/src/
  main.rs      the app spec
  docs/        part_studio.rs, calc_sheet.rs, shared Console pane
template/      cargo-generate starter app (excluded from the workspace)
```

## Theming

Everything is a CSS custom property on `.as-root`. Override from your app:

```css
.as-root[data-theme="dark"] { --as-accent: #f59e0b; --as-rail-w: 44px; }
```

The left/right side tabs float over the viewport and are tuned with
`--as-tab-w`, `--as-tab-h`, `--as-tab-blur`, `--as-tab-bg` and
`--as-tab-bg-hover` (lower the alpha in `--as-tab-bg` for more see-through tabs):

```css
.as-root[data-theme="dark"] { --as-tab-bg: color-mix(in srgb, var(--as-surface) 35%, transparent); }
```

## Ideas for next steps

- Persist layout (`nav_width`, open panels per type, theme) to disk / localStorage.
- Floating / detachable panels (Dioxus desktop supports multiple windows).
- Context-sensitive toolbar: `DocType::toolbar` as `fn(&Shell) -> Vec<ToolGroup>` so
  tools change with selection or mode (sketch mode vs. part mode).
- Split viewports (two documents side by side) by making the center a tree of slots.
- A plugin trait so crates can register doc types at startup.
