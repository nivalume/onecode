//! OneCode desktop app entry point.

use gpui_kit::{AppContext as _, KeyBinding};
use onecode_ui::Workspace;

gpui_kit::actions!(onecode, [Quit]);

fn main() {
    gpui_kit::application()
        .with_assets(onecode_ui::Assets)
        .run(|cx| {
            gpui_kit::init(cx);
            onecode_ui::init(cx);

            cx.on_action(|_: &Quit, cx| cx.quit());
            cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
            // OneCode has one main window; closing it ends the app.
            cx.on_window_closed(|cx, _| cx.quit()).detach();

            let options = onecode_ui::window_options(cx);
            match gpui_kit::open_window(options, cx, |window, cx| {
                cx.new(|cx| Workspace::new(window, cx))
            }) {
                Ok(_) => cx.activate(true),
                Err(err) => {
                    eprintln!("onecode: could not open the main window: {err:#}");
                    cx.quit();
                }
            }
        });
}
