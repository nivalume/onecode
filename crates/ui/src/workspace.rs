//! The main window: sidebar on the leading edge, conversation in the middle,
//! terminal below it and the workspace panel on the trailing edge.

mod conversation;
mod header;
mod model_picker;
mod sidebar;
mod terminal;
mod tool_panel;

use gpui_kit::assets::IconName;
use gpui_kit::component::button::{Button, ButtonVariants as _};
use gpui_kit::component::input::TextareaState;
use gpui_kit::component::kbd::Kbd;
use gpui_kit::component::resizable::{h_resizable, resizable_panel, v_resizable};
use gpui_kit::component::{StyledExt as _, v_flex};
use gpui_kit::{
    App, AppContext as _, Bounds, Context, Div, Entity, FocusHandle, InteractiveElement as _,
    IntoElement, Keystroke, ParentElement as _, Rems, Render, SharedString, Styled as _,
    Subscription, TitlebarOptions, Window, WindowBounds, WindowOptions, div, point, px, rems, size,
};
use onecode_core::{Machine, Message, Task, TaskContext, TaskGroup, sample};

use crate::theme::{Palette, TEXT_KBD};
use conversation::{composer, composer_state, conversation};
use header::Editor;
use model_picker::ModelPicker;
use sidebar::sidebar;
use terminal::terminal;
use tool_panel::ToolSurface;

gpui_kit::actions!(
    workspace,
    [
        ToggleSidebar,
        ToggleSessionControl,
        ToggleTerminal,
        ToggleRightPanel
    ]
);

/// Chooses where the session control's "Open in" command opens the project.
#[derive(Clone, Copy, Debug, PartialEq, gpui_kit::Action)]
#[action(namespace = workspace, no_json)]
pub(crate) struct ChooseEditor(Editor);

/// Key context for bindings that act on the whole workspace.
pub(crate) const CONTEXT: &str = "Workspace";

/// `spacing.layout.header-height` (56 px), shared by the main and panel headers.
const HEADER_HEIGHT: Rems = rems(3.5);

/// Options for the main window: the 1440 × 900 artboard size, centered, with a
/// transparent title bar so the macOS window controls sit inside the sidebar.
pub fn window_options(cx: &App) -> WindowOptions {
    // Window geometry is platform-owned, so these stay in pixels.
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
            None,
            size(px(1440.), px(900.)),
            cx,
        ))),
        window_min_size: Some(size(px(960.), px(600.))),
        titlebar: Some(TitlebarOptions {
            title: Some("OneCode".into()),
            appears_transparent: true,
            // Lines the controls up with the sidebar's 16 px inset and its
            // first 24 px row.
            traffic_light_position: Some(point(px(16.), px(20.))),
        }),
        ..Default::default()
    }
}

/// Root view of the main window. Owns the task list and which regions are open.
pub struct Workspace {
    focus_handle: FocusHandle,
    sidebar_open: bool,
    session_open: bool,
    terminal_open: bool,
    panel_open: bool,
    editor: Editor,
    machine: Machine,
    messages: Vec<Message>,
    composer: Entity<TextareaState>,
    model_picker: Entity<ModelPicker>,
    picker_open: bool,
    _model_choice_changed: Subscription,
    task_groups: Vec<TaskGroup>,
    selected_task: String,
    task_context: TaskContext,
    tool_surface: ToolSurface,
}

impl Workspace {
    /// Creates the workspace and focuses it so its key bindings work at once.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        window.focus(&focus_handle, cx);

        let task_groups = sample::task_groups();
        let selected_task = String::from(sample::OPEN_TASK);
        let project = task_groups
            .iter()
            .flat_map(|group| &group.tasks)
            .find(|task| task.id == selected_task)
            .map(|task| SharedString::from(task.project.clone()));
        let model_picker = cx.new(|cx| ModelPicker::new(project, window, cx));
        // The composer's chip shows the picker's choice.
        let _model_choice_changed = cx.observe(&model_picker, |_, _, cx| cx.notify());

        Self {
            focus_handle,
            sidebar_open: true,
            session_open: false,
            terminal_open: false,
            panel_open: false,
            editor: Editor::VsCode,
            machine: sample::machine(),
            messages: sample::conversation(),
            composer: cx.new(|cx| composer_state(window, cx)),
            model_picker,
            picker_open: false,
            _model_choice_changed,
            task_groups,
            selected_task,
            task_context: sample::task_context(),
            tool_surface: ToolSurface::Launcher,
        }
    }

    /// The task open in the conversation, if it is still in the list.
    fn selected_task(&self) -> Option<&Task> {
        self.task_groups
            .iter()
            .flat_map(|group| &group.tasks)
            .find(|task| task.id == self.selected_task)
    }
}

impl Render for Workspace {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let palette = Palette::global(cx);
        div()
            .key_context(CONTEXT)
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(|this, _: &ToggleSidebar, _, cx| {
                this.sidebar_open = !this.sidebar_open;
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ToggleSessionControl, _, cx| {
                this.session_open = !this.session_open;
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ToggleTerminal, _, cx| {
                this.terminal_open = !this.terminal_open;
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ToggleRightPanel, window, cx| {
                this.panel_open = !this.panel_open;
                if !this.panel_open {
                    window.focus(&this.focus_handle, cx);
                }
                cx.notify();
            }))
            .on_action(cx.listener(|this, choice: &ChooseEditor, _, cx| {
                this.editor = choice.0;
                cx.notify();
            }))
            .size_full()
            .bg(palette.bg)
            .text_color(palette.ink)
            .text_sm()
            // Split sizes are resolved geometry, so the groups take pixels;
            // the defaults and ranges come from `spacing.layout` in DESIGN.md.
            .child(
                h_resizable("workspace-split")
                    .child(
                        resizable_panel()
                            .visible(self.sidebar_open)
                            .fixed()
                            .size(px(272.))
                            .size_range(px(264.)..px(440.))
                            .child(sidebar(&self.task_groups, &self.selected_task, cx)),
                    )
                    .child(
                        resizable_panel().child(
                            v_flex()
                                .size_full()
                                .child(self.render_header(window, cx))
                                .child(
                                    v_resizable("conversation-split")
                                        .child(
                                            resizable_panel().child(
                                                v_flex()
                                                    .size_full()
                                                    .child(conversation(&self.messages, cx))
                                                    .child(composer(&self.composer, self.render_model_chip(cx), cx)),
                                            ),
                                        )
                                        .child(
                                            resizable_panel()
                                                .visible(self.terminal_open)
                                                .size(px(240.))
                                                .size_range(px(120.)..px(480.))
                                                .child(terminal(
                                                    &self.machine,
                                                    self.selected_task()
                                                        .map(|task| task.project.as_str()),
                                                    cx,
                                                )),
                                        ),
                                ),
                        ),
                    )
                    .child(
                        resizable_panel()
                            .visible(self.panel_open)
                            .size(px(440.))
                            .size_range(px(320.)..px(720.))
                            .child(self.render_tool_panel(cx)),
                    ),
            )
    }
}

/// The sidebar show/hide button, in the sidebar's top row or in the header.
fn sidebar_toggle() -> Button {
    icon_button("toggle-sidebar", IconName::PanelLeft, "Toggle sidebar")
        .size_7()
        .tooltip_with_action("Toggle sidebar", &ToggleSidebar, Some(CONTEXT))
        .on_click(|_, window, cx| window.dispatch_action(Box::new(ToggleSidebar), cx))
}

/// A quiet icon-only button, named for assistive technology and its tooltip.
fn icon_button(id: impl Into<SharedString>, icon: IconName, label: &'static str) -> Button {
    Button::new(id.into())
        .ghost()
        .icon(icon)
        .accessibility_label(label)
        .tooltip(label)
}

/// The uppercased first letter of a name, for avatars and badges.
fn initial(name: &str) -> String {
    name.chars().take(1).flat_map(char::to_uppercase).collect()
}

/// The accent monogram tile for a project, sized by the caller.
fn project_badge(project: &str, palette: &Palette) -> Div {
    div()
        .flex()
        .flex_none()
        .items_center()
        .justify_center()
        .bg(palette.accent_soft)
        .text_color(palette.accent)
        .text_size(TEXT_KBD)
        .font_bold()
        .child(initial(project))
}

/// A bordered keycap in the design's `kbd` style.
fn keycap(keystroke: Keystroke, palette: &Palette, mono: SharedString) -> Kbd {
    Kbd::new(keystroke)
        .outline()
        .font_family(mono)
        .text_size(TEXT_KBD)
        .bg(palette.surface)
        .text_color(palette.ink_2)
}

/// Parses a shortcut literal written in this crate, such as `cmd-k`.
fn shortcut(literal: &'static str) -> Keystroke {
    // Every caller passes a literal, so a parse failure is a typo.
    Keystroke::parse(literal).expect("shortcut literals are valid keystrokes")
}
