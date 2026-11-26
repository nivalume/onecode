//! The main area's header: a breadcrumb to the open task, and the panel
//! toggles with the session control popover.

use gpui_kit::assets::IconName;
use gpui_kit::component::button::{Button, ButtonVariants as _};
use gpui_kit::component::menu::{DropdownMenu as _, PopupMenuItem};
use gpui_kit::component::popover::Popover;
use gpui_kit::component::{Icon, Selectable as _, StyledExt as _, h_flex, v_flex};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    Action, Anchor, Context, FocusHandle, IntoElement, ParentElement as _, Pixels, SharedString,
    Styled as _, Window, div, px, rems,
};
use onecode_core::Task;

use super::{
    CONTEXT, ChooseEditor, HEADER_HEIGHT, ToggleRightPanel, ToggleSessionControl, ToggleTerminal,
    Workspace, icon_button, project_badge, sidebar_toggle,
};
use crate::theme::{Palette, TEXT_CONTROL};

/// Leading space the macOS window controls take in the header while the
/// sidebar is hidden: their 16 px inset, 52 px span and a 10 px gap.
const WINDOW_CONTROLS_INSET: Pixels = px(78.);

/// Where the session control's "Open in" command opens the project.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Editor {
    VsCode,
    Finder,
}

impl Editor {
    const ALL: [Editor; 2] = [Editor::VsCode, Editor::Finder];

    fn label(self) -> &'static str {
        match self {
            Editor::VsCode => "VS Code",
            Editor::Finder => "Finder",
        }
    }

    fn icon(self) -> IconName {
        match self {
            Editor::VsCode => IconName::Code,
            Editor::Finder => IconName::Folder,
        }
    }
}

impl Workspace {
    pub(super) fn render_header(
        &self,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let session_control = self.render_session_control(cx);
        let palette = Palette::global(cx);
        h_flex()
            .flex_none()
            .h(HEADER_HEIGHT)
            .gap_3()
            .pr_3p5()
            .map(|row| match (self.sidebar_open, window.is_fullscreen()) {
                (true, _) => row.pl_5(),
                // Full screen hides the window controls, so the toggle takes their place.
                (false, true) => row.pl_3p5().child(sidebar_toggle()),
                (false, false) => row.pl(WINDOW_CONTROLS_INSET).child(sidebar_toggle()),
            })
            .when_some(self.selected_task(), |row, task| {
                row.child(breadcrumb(task, palette))
            })
            .child(div().flex_1())
            .child(
                h_flex()
                    .gap_0p5()
                    .child(session_control)
                    .child(
                        panel_toggle(
                            "toggle-terminal",
                            IconName::PanelBottom,
                            "Toggle terminal",
                            &ToggleTerminal,
                            self.terminal_open,
                            palette,
                        )
                        .on_click(|_, window, cx| {
                            window.dispatch_action(Box::new(ToggleTerminal), cx)
                        }),
                    )
                    .child(
                        panel_toggle(
                            "toggle-right-panel",
                            IconName::PanelRight,
                            "Toggle right panel",
                            &ToggleRightPanel,
                            self.panel_open,
                            palette,
                        )
                        .on_click(|_, window, cx| {
                            window.dispatch_action(Box::new(ToggleRightPanel), cx)
                        }),
                    ),
            )
    }

    /// The session toggle and the popover it opens. The workspace owns whether
    /// it is open, so ⌘I and the button stay in step.
    fn render_session_control(&self, cx: &mut Context<Self>) -> Popover {
        let palette = Palette::global(cx);
        let editor = self.editor;
        let machine_name = SharedString::from(self.machine.name.clone());
        let focus_handle = self.focus_handle.clone();
        Popover::new("session-control")
            .anchor(Anchor::TopRight)
            .trigger(panel_toggle(
                "toggle-session",
                IconName::SquareMenu,
                "Toggle session control",
                &ToggleSessionControl,
                self.session_open,
                palette,
            ))
            .open(self.session_open)
            .on_open_change(cx.listener(|this, open: &bool, _, cx| {
                this.session_open = *open;
                cx.notify();
            }))
            .w(rems(18.))
            .p_1p5()
            .rounded(rems(0.875))
            .border_1()
            .border_color(palette.border)
            .bg(palette.surface)
            .shadow(palette.shadow_composer.clone())
            .content(move |_, _, cx| {
                session_menu(
                    editor,
                    machine_name.clone(),
                    focus_handle.clone(),
                    Palette::global(cx),
                )
            })
    }
}

/// Project badge, project name and the open task's title.
fn breadcrumb(task: &Task, palette: &Palette) -> impl IntoElement {
    h_flex()
        .min_w_0()
        .gap_2()
        .child(
            project_badge(&task.project, palette)
                .size(rems(1.125))
                .rounded(rems(0.3125)),
        )
        .child(div().text_color(palette.ink_2).child(task.project.clone()))
        .child(div().text_color(palette.ink_disabled).child("/"))
        .child(
            Button::new("task-title")
                .ghost()
                .h_7()
                .px_1p5()
                .label(task.title.clone())
                .dropdown_caret(true)
                .text_color(palette.ink)
                .font_semibold(),
        )
}

/// A 32 px header button that shows whether its region is open.
fn panel_toggle(
    id: &'static str,
    icon: IconName,
    label: &'static str,
    action: &dyn Action,
    open: bool,
    palette: &Palette,
) -> Button {
    icon_button(id, icon, label)
        .selected(open)
        .toggled(open)
        .when(open, |button| button.text_color(palette.ink))
        .tooltip_with_action(label, action, Some(CONTEXT))
}

/// The session control's rows: where the task runs, where to open the
/// project, and project setup commands.
// `use<>`: the rows copy what they show, so they outlive the palette borrow.
fn session_menu(
    editor: Editor,
    machine_name: SharedString,
    workspace_focus: FocusHandle,
    palette: &Palette,
) -> impl IntoElement + use<> {
    v_flex()
        .gap_0p5()
        .text_size(TEXT_CONTROL)
        .child(
            h_flex()
                .h_9()
                .pl_2p5()
                .pr_1()
                .gap_2p5()
                .child(
                    Icon::new(IconName::Laptop)
                        .size_4()
                        .text_color(palette.ink_2),
                )
                .child(div().flex_1().min_w_0().text_ellipsis().child(machine_name))
                .child(
                    icon_button(
                        "show-project-folder",
                        IconName::Folder,
                        "Show project folder",
                    )
                    .size_7(),
                ),
        )
        .child(
            h_flex()
                .h_9()
                .child(
                    menu_row(
                        "open-in-editor",
                        Icon::new(editor.icon()).text_color(palette.accent),
                        format!("Open in {}", editor.label()),
                        palette,
                    )
                    .flex_1(),
                )
                .child(
                    icon_button(
                        "choose-editor",
                        IconName::ChevronDown,
                        "Choose where to open",
                    )
                    .dropdown_menu_with_anchor(
                        Anchor::TopRight,
                        move |menu, _, _| {
                            // Route the choice to the workspace, which owns it.
                            let mut menu = menu
                                .action_context(workspace_focus.clone())
                                .label("Open in");
                            for choice in Editor::ALL {
                                menu = menu.item(
                                    PopupMenuItem::new(choice.label())
                                        .icon(choice.icon())
                                        .checked(choice == editor)
                                        .action(Box::new(ChooseEditor(choice))),
                                );
                            }
                            menu
                        },
                    ),
                ),
        )
        .child(menu_row(
            "add-project-script",
            Icon::new(IconName::Plus).text_color(palette.ink_2),
            "Add project script",
            palette,
        ))
        .child(div().h_px().mx_1p5().my_1().bg(palette.line))
        .child(menu_row(
            "set-up-git",
            Icon::new(IconName::GitBranch).text_color(palette.ink_2),
            "Set up Git",
            palette,
        ))
}

/// A full-width 36 px command row with a leading icon, as in a menu.
fn menu_row(
    id: &'static str,
    icon: Icon,
    label: impl Into<SharedString>,
    palette: &Palette,
) -> Button {
    Button::new(id).ghost().w_full().h_9().px_2p5().child(
        h_flex()
            .flex_1()
            .gap_2p5()
            .text_color(palette.ink)
            .child(icon.size_4())
            .child(label.into()),
    )
}
