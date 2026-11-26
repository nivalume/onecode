//! OneCode desktop UI built on gpui-kit. Visual tokens are defined in `DESIGN.md`.

mod assets;
mod theme;
mod workspace;

use gpui_kit::component::ThemeMode;
use gpui_kit::{App, KeyBinding};

pub use assets::Assets;
pub use workspace::{Workspace, window_options};

/// Sets up the OneCode UI. Call once, right after `gpui_kit::init`.
pub fn init(cx: &mut App) {
    theme::set_mode(ThemeMode::Light, cx);
    let context = Some(workspace::CONTEXT);
    cx.bind_keys([
        KeyBinding::new("cmd-b", workspace::ToggleSidebar, context),
        KeyBinding::new("cmd-i", workspace::ToggleSessionControl, context),
        KeyBinding::new("cmd-j", workspace::ToggleTerminal, context),
        KeyBinding::new("alt-cmd-b", workspace::ToggleRightPanel, context),
    ]);
}
