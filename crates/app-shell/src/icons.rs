//! Tiny built-in stroke icon set (24×24 grid, drawn with `currentColor`).
//!
//! An [`Icon`] is just SVG path data, so apps can add their own:
//! `const MY_ICON: Icon = Icon("M4 4h16v16H4z");`
//! or swap the whole set for Lucide/Tabler paths without touching the shell.

mod ico;
mod icon;

pub use ico::{Ico, IcoProps};
pub use icon::Icon;

// circle helper is written out as two arcs: "M cx-r cy a r r 0 1 0 2r 0 a r r 0 1 0 -2r 0"

pub const LOGO: Icon = Icon("M12 2.5l8.2 4.75v9.5L12 21.5l-8.2-4.75v-9.5zM12 7.5l4 2.3v4.4L12 16.5l-4-2.3V9.8z");
pub const MENU: Icon = Icon("M4 6h16M4 12h16M4 18h16");
pub const CLOSE: Icon = Icon("M6 6l12 12M18 6L6 18");
pub const PLUS: Icon = Icon("M12 5v14M5 12h14");
pub const SEARCH: Icon = Icon("M4 11a7 7 0 1 0 14 0a7 7 0 1 0-14 0M20 20l-4-4");
pub const FILES: Icon = Icon("M6 3h8l4 4v14H6zM14 3v4h4M9 12h6M9 16h6");
pub const FOLDER: Icon = Icon("M3 6h6l2 2h10v11H3z");
pub const HISTORY: Icon = Icon("M3 12a9 9 0 1 0 18 0a9 9 0 1 0-18 0M12 7v5l3 3");
pub const COMMENT: Icon = Icon("M4 5h16v11H9l-5 4z");
pub const SLIDERS: Icon = Icon("M4 7h10M18 7h2M4 17h4M12 17h8M16 5v4M10 15v4");
pub const BELL: Icon = Icon("M6 16v-5a6 6 0 0 1 12 0v5l2 2H4zM10 20a2 2 0 0 0 4 0");
pub const SHARE: Icon = Icon("M12 3v12M8 7l4-4 4 4M5 13v7h14v-7");
pub const USER: Icon = Icon("M8 8a4 4 0 1 0 8 0a4 4 0 1 0-8 0M4 21a8 8 0 0 1 16 0");
pub const HELP: Icon = Icon("M3 12a9 9 0 1 0 18 0a9 9 0 1 0-18 0M9.5 9.5a2.5 2.5 0 1 1 3.5 2.3c-.7.3-1 .8-1 1.7M12 17h.01");
pub const PENCIL: Icon = Icon("M4 20l4-1L19 8l-3-3L5 16zM14 7l3 3");
pub const EXTRUDE: Icon = Icon("M4 15l8 4 8-4-8-4zM12 11V3M9 6l3-3 3 3");
pub const REVOLVE: Icon = Icon("M20 12a8 8 0 1 1-2.6-5.9M20 4v5h-5");
pub const FILLET: Icon = Icon("M5 20v-8a7 7 0 0 1 7-7h7");
pub const CHAMFER: Icon = Icon("M5 20v-9l6-6h8");
pub const SHELL: Icon = Icon("M3 7l9-4 9 4v10l-9 4-9-4zM8 9.5l4 1.8 4-1.8v5l-4 1.8-4-1.8z");
pub const PATTERN: Icon = Icon("M4 4h6v6H4zM14 4h6v6h-6zM4 14h6v6H4zM14 14h6v6h-6z");
pub const MIRROR: Icon = Icon("M12 3v18M9 7L4 17h5zM15 7l5 10h-5z");
pub const TREE: Icon = Icon("M4 5h7M9 12h9M9 19h9M5 5v14M5 12h4M5 19h4");
pub const LAYERS: Icon = Icon("M12 3l9 5-9 5-9-5zM3 13l9 5 9-5");
pub const DROPLET: Icon = Icon("M12 3s6 7 6 11a6 6 0 0 1-12 0c0-4 6-11 6-11z");
pub const INFO: Icon = Icon("M3 12a9 9 0 1 0 18 0a9 9 0 1 0-18 0M12 11v6M12 7.5h.01");
pub const SCALE: Icon = Icon("M12 4v16M6 20h12M5 8h14M7 8l-3 6h6zM17 8l-3 6h6z");
pub const TERMINAL: Icon = Icon("M4 5h16v14H4zM7 9l3 3-3 3M12 15h5");
pub const RULER: Icon = Icon("M3 17L17 3l4 4L7 21zM7 13l2 2M10 10l2 2M13 7l2 2");
pub const CUBE: Icon = Icon("M12 3l8 4.5v9L12 21l-8-4.5v-9zM4 7.5l8 4.5 8-4.5M12 12v9");
pub const EYE: Icon = Icon("M2 12s4-7 10-7 10 7 10 7-4 7-10 7S2 12 2 12zM9 12a3 3 0 1 0 6 0a3 3 0 1 0-6 0");
pub const UNDO: Icon = Icon("M9 14L4 9l5-5M4 9h10a6 6 0 0 1 0 12h-3");
pub const REDO: Icon = Icon("M15 14l5-5-5-5M20 9H10a6 6 0 0 0 0 12h3");
pub const FX: Icon = Icon("M6 20c2 0 2-2 3-8s1-8 3-8M6 11h7M14 13l5 6M19 13l-5 6");
pub const TABLE: Icon = Icon("M4 5h16v14H4zM4 10h16M4 15h16M10 5v14");
pub const PLAY: Icon = Icon("M7 4l13 8-13 8z");
pub const DOWNLOAD: Icon = Icon("M12 4v11M7 10l5 5 5-5M5 20h14");
pub const VARIABLE: Icon = Icon("M8 4c-3 3-3 13 0 16M16 4c3 3 3 13 0 16M10 9l4 6M14 9l-4 6");
pub const CHEVRON_RIGHT: Icon = Icon("M9 6l6 6-6 6");
pub const CHEVRON_DOWN: Icon = Icon("M6 9l6 6 6-6");
pub const SUN: Icon = Icon("M8 12a4 4 0 1 0 8 0a4 4 0 1 0-8 0M12 2v2M12 20v2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M2 12h2M20 12h2M4.9 19.1l1.4-1.4M17.7 6.3l1.4-1.4");
pub const COMMAND: Icon = Icon("M9 6a3 3 0 1 0-3 3h12a3 3 0 1 0-3-3v12a3 3 0 1 0 3-3H6a3 3 0 1 0 3 3z");
pub const HOME: Icon = Icon("M4 11l8-7 8 7v9H4zM10 20v-5h4v5");
pub const GRID: Icon = Icon("M3 9h18M3 15h18M9 3v18M15 3v18");
pub const TARGET: Icon = Icon("M3 12a9 9 0 1 0 18 0a9 9 0 1 0-18 0M8 12a4 4 0 1 0 8 0a4 4 0 1 0-8 0M12 2v4M12 18v4M2 12h4M18 12h4");
pub const LINK: Icon = Icon("M10 14a4 4 0 0 0 6 0l3-3a4 4 0 0 0-6-6l-1 1M14 10a4 4 0 0 0-6 0l-3 3a4 4 0 0 0 6 6l1-1");
pub const DOT: Icon = Icon("M11 12a1 1 0 1 0 2 0a1 1 0 1 0-2 0");
