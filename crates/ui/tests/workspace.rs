use gpui_kit::test::TestWindowExt;
use gpui_kit::{
    AnyWindowHandle, AppContext as _, Bounds, TestAppContext, Window, WindowBounds, WindowOptions,
    px, size,
};
use onecode_ui::Workspace;

fn open_workspace(cx: &mut TestAppContext) -> AnyWindowHandle {
    cx.update(|cx| {
        gpui_kit::init(cx);
        onecode_ui::init(cx);
    });
    let (handle, _) = cx.update(|cx| {
        gpui_kit::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                    Default::default(),
                    size(px(1440.), px(900.)),
                ))),
                ..Default::default()
            },
            cx,
            |window, cx| cx.new(|cx| Workspace::new(window, cx)),
        )
        .expect("open workspace test window")
    });
    handle
}

fn is_visible(window: &Window, id: &'static str) -> bool {
    window.try_find(id).is_some_and(|el| el.visible())
}

/// Clicks an element and flushes deferred actions (e.g. `window.dispatch_action`).
fn click_and_flush(cx: &mut TestAppContext, handle: AnyWindowHandle, id: &'static str) {
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        window.click(id, cx);
    })
    .expect("click target in workspace window");
    cx.run_until_parked();
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
    })
    .expect("render workspace window after action");
}

#[gpui_kit::test]
fn tool_panel_launcher_and_context_view(cx: &mut TestAppContext) {
    let handle = open_workspace(cx);

    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        assert!(!is_visible(window, "tool-Context"));
        assert_eq!(window.find("toggle-right-panel").checked(), Some(false));
    })
    .unwrap();

    // Open the right panel from the header toggle (dispatches ToggleRightPanel).
    click_and_flush(cx, handle, "toggle-right-panel");

    cx.update_window(handle, |_, window, cx| {
        assert_eq!(window.find("toggle-right-panel").checked(), Some(true));
        assert!(is_visible(window, "tool-Context"));
        assert!(is_visible(window, "tool-Terminal"));

        // Review tools that need Git or changes stay visible but reject clicks.
        let changes = window.find("tool-Changes");
        let pull_request = window.find("tool-Pull request");
        assert_eq!(changes.disabled(), Some(true));
        assert_eq!(pull_request.disabled(), Some(true));
        window.click("tool-Pull request", cx);
        assert!(is_visible(window, "tool-Context"));
        assert!(!is_visible(window, "terminal-tool"));

        // Open Context from the launcher.
        window.click("tool-Context", cx);
        assert!(!is_visible(window, "tool-Context"));
        assert!(is_visible(window, "close-context"));

        // Switches reflect TaskContext and write back when toggled.
        let terminal = window.find("terminal-tool");
        assert_eq!(terminal.label(), Some("Terminal"));
        assert_eq!(terminal.checked(), Some(true));
        let browser = window.find("browser-tool");
        assert_eq!(browser.label(), Some("Browser"));
        assert_eq!(browser.checked(), Some(true));

        let context_window = window.find("context-window");
        assert!(context_window.visible());
        assert_eq!(context_window.label(), Some("Context window used"));

        window.click("terminal-tool", cx);
        assert_eq!(window.find("terminal-tool").checked(), Some(false));
        window.click("terminal-tool", cx);
        assert_eq!(window.find("terminal-tool").checked(), Some(true));

        window.click("browser-tool", cx);
        assert_eq!(window.find("browser-tool").checked(), Some(false));

        // Closing the panel via shortcut while a Context switch has focus restores workspace focus.
        window.press("alt-cmd-b", cx);
        assert!(!is_visible(window, "browser-tool"));
        window.press("alt-cmd-b", cx);
        assert_eq!(window.find("browser-tool").checked(), Some(false));

        // Closing the Context chip returns to the launcher.
        window.click("close-context", cx);
        assert!(is_visible(window, "tool-Context"));
        assert!(!is_visible(window, "terminal-tool"));

        // Re-opening Context keeps the toggled Browser switch, and "+" returns to the launcher.
        window.click("tool-Context", cx);
        assert_eq!(window.find("browser-tool").checked(), Some(false));
        window.click("open-a-tool", cx);
        assert!(is_visible(window, "tool-Context"));
        // Closing the panel hides it and clears the header toggle state.
        window.click("close-panel", cx);
        assert!(!is_visible(window, "tool-Context"));
        assert_eq!(window.find("toggle-right-panel").checked(), Some(false));

        // Focus returned to the workspace, so Opt+Cmd+B still re-opens the panel.
        window.press("alt-cmd-b", cx);
        assert!(is_visible(window, "tool-Context"));
    })
    .unwrap();
}

#[gpui_kit::test]
fn workspace_panel_shortcuts_and_toggles(cx: &mut TestAppContext) {
    let handle = open_workspace(cx);

    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);

        // Sidebar starts open; Cmd+B hides and restores it.
        assert!(is_visible(window, "new-task"));
        window.press("cmd-b", cx);
        assert!(!is_visible(window, "new-task"));
        window.press("cmd-b", cx);
        assert!(is_visible(window, "new-task"));

        // Terminal starts closed; Cmd+J opens it.
        assert!(!is_visible(window, "close-terminal"));
        window.press("cmd-j", cx);
        assert!(is_visible(window, "close-terminal"));
    })
    .unwrap();

    // Terminal close button dispatches ToggleTerminal.
    click_and_flush(cx, handle, "close-terminal");

    cx.update_window(handle, |_, window, cx| {
        assert!(!is_visible(window, "close-terminal"));

        // Right panel toggles with Opt+Cmd+B.
        assert!(!is_visible(window, "open-a-tool"));
        window.press("alt-cmd-b", cx);
        assert!(is_visible(window, "open-a-tool"));
        window.press("alt-cmd-b", cx);
        assert!(!is_visible(window, "open-a-tool"));
    })
    .unwrap();
}

#[gpui_kit::test]
fn model_picker_and_permissions_flow(cx: &mut TestAppContext) {
    let handle = open_workspace(cx);

    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        assert_eq!(window.find("model").label(), Some("Gemini 4 Neon"));

        // Open the model picker popover from the composer chip.
        window.click("model", cx);
        assert!(is_visible(window, "filter-Anthropic"));

        // Filter to Anthropic and pick a model and reasoning effort.
        window.click("filter-Anthropic", cx);
        assert!(!is_visible(window, "model-Gemini 4 Neon"));
        window.click("model-Claude Opus 5.5", cx);
        window.click("effort-Extra", cx);

        // All four reasoning segments fit within the 256 px details column (224 px inner width).
        let segments_width = window.find("effort-Extra").bounds().right()
            - window.find("effort-Low").bounds().left();
        assert!(segments_width <= px(224.));

        // Open Permissions, pick Auto, toggle destructive-action confirmation, and return.
        window.click("permissions", cx);
        assert!(is_visible(window, "level-Auto"));
        window.click("level-Auto", cx);

        assert_eq!(
            window.find("confirm-destructive-actions").checked(),
            Some(true)
        );
        window.click("confirm-destructive-actions", cx);
        assert_eq!(
            window.find("confirm-destructive-actions").checked(),
            Some(false)
        );

        window.click("permissions-done", cx);
        assert!(is_visible(window, "permissions"));
    })
    .unwrap();

    // Observer notifies Workspace to update the composer's model chip label.
    cx.run_until_parked();
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        assert_eq!(window.find("model").label(), Some("Claude Opus 5.5"));
    })
    .unwrap();
}
