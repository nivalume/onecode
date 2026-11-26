//! The workspace panel beside the chat: a launcher for the task's tools, and
//! the Context tool showing what the task can see and use.

use gpui_kit::assets::IconName;
use gpui_kit::component::button::{Button, ButtonVariants as _};
use gpui_kit::component::progress::Progress;
use gpui_kit::component::scroll::ScrollableElement as _;
use gpui_kit::component::switch::Switch;
use gpui_kit::component::{
    ActiveTheme as _, Disableable as _, Icon, Sizable as _, StyledExt as _, h_flex, v_flex,
};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    AnyElement, Context, InteractiveElement as _, IntoElement, ParentElement as _, Rems,
    SharedString, Styled as _, Window, div, rems,
};
use onecode_core::TaskContext;

use super::{HEADER_HEIGHT, Workspace, icon_button, keycap, shortcut};
use crate::theme::{Palette, TEXT_CAPTION, TEXT_CONTROL, TEXT_MESSAGE};

/// Header buttons in the panel (30 px).
const HEADER_BUTTON: Rems = rems(1.875);
/// Widest the launcher's list gets, centered in the panel (384 px).
const LAUNCHER_MAX_WIDTH: Rems = rems(24.);

/// What the panel shows.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum ToolSurface {
    Launcher,
    Context,
}

/// A tool the launcher lists.
struct Tool {
    label: &'static str,
    hint: &'static str,
    /// The key that opens it from the launcher.
    key: &'static str,
    icon: IconName,
    available: bool,
    /// The surface it opens in this build, if any.
    opens: Option<ToolSurface>,
}

/// The launcher's tools by group. Review tools wait for something to review.
fn tool_groups(context: &TaskContext) -> [(&'static str, [Tool; 3]); 3] {
    let tool = |label, hint, key, icon| Tool {
        label,
        hint,
        key,
        icon,
        available: true,
        opens: None,
    };
    let has_changes = context.changed_files > 0;
    [
        (
            "Build",
            [
                tool("Terminal", "Run commands", "t", IconName::SquareTerminal),
                tool("Files", "Browse the project", "f", IconName::File),
                tool("Browser", "Pages and local servers", "b", IconName::Globe),
            ],
        ),
        (
            "Review",
            [
                Tool {
                    available: has_changes,
                    ..tool(
                        "Changes",
                        if has_changes {
                            "Review what changed"
                        } else {
                            "Nothing changed yet"
                        },
                        "d",
                        IconName::FileDiff,
                    )
                },
                Tool {
                    available: context.git_initialized,
                    ..tool(
                        "Pull request",
                        if context.git_initialized {
                            "Open one for review"
                        } else {
                            "Set up Git first"
                        },
                        "p",
                        IconName::GitPullRequest,
                    )
                },
                tool("Issues", "Link an issue", "i", IconName::CircleDot),
            ],
        ),
        (
            "This task",
            [
                Tool {
                    opens: Some(ToolSurface::Context),
                    ..tool("Context", "Rules, memory, tools", "c", IconName::Layers)
                },
                tool("Side chat", "Ask on the side", "s", IconName::MessageSquare),
                tool("Device", "Phone or simulator", "m", IconName::Smartphone),
            ],
        ),
    ]
}

impl Workspace {
    /// Switches the panel surface and restores workspace focus so shortcuts keep working
    /// after controls on the previous surface unmount.
    fn set_tool_surface(
        &mut self,
        surface: ToolSurface,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.tool_surface = surface;
        window.focus(&self.focus_handle, cx);
        cx.notify();
    }

    pub(super) fn render_tool_panel(&self, cx: &Context<Self>) -> impl IntoElement {
        let palette = Palette::global(cx);
        let body: AnyElement = match self.tool_surface {
            ToolSurface::Launcher => self.render_launcher(palette, cx).into_any_element(),
            ToolSurface::Context => self.render_context(palette, cx).into_any_element(),
        };
        v_flex()
            .size_full()
            .bg(palette.bg)
            .child(
                h_flex()
                    .flex_none()
                    .h(HEADER_HEIGHT)
                    .px_2p5()
                    .gap_1p5()
                    .child(
                        icon_button("open-a-tool", IconName::Plus, "Open a tool")
                            .size(HEADER_BUTTON)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.set_tool_surface(ToolSurface::Launcher, window, cx);
                            })),
                    )
                    .when(self.tool_surface == ToolSurface::Context, |row| {
                        row.child(
                            h_flex()
                                .h(HEADER_BUTTON)
                                .pl_2p5()
                                .pr_1()
                                .gap_1p5()
                                .rounded_lg()
                                .bg(palette.fill)
                                .text_size(TEXT_CONTROL)
                                .font_semibold()
                                .child(Icon::new(IconName::Layers).size_3p5())
                                .child("Context")
                                .child(
                                    icon_button("close-context", IconName::Close, "Close Context")
                                        .size(rems(1.375))
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.set_tool_surface(
                                                ToolSurface::Launcher,
                                                window,
                                                cx,
                                            );
                                        })),
                                ),
                        )
                    })
                    .child(div().flex_1())
                    .child(
                        icon_button("expand-panel", IconName::Maximize2, "Expand panel")
                            .size(HEADER_BUTTON),
                    )
                    .child(
                        icon_button("close-panel", IconName::Close, "Close panel")
                            .size(HEADER_BUTTON)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.panel_open = false;
                                window.focus(&this.focus_handle, cx);
                                cx.notify();
                            })),
                    ),
            )
            .child(body)
    }

    fn render_launcher(&self, palette: &Palette, cx: &Context<Self>) -> impl IntoElement {
        let mono = cx.theme().mono_font_family.clone();
        v_flex()
            .id("tool-launcher")
            .flex_1()
            .min_h_0()
            .items_center()
            .px_7()
            .py_8()
            .overflow_y_scrollbar()
            .child(
                v_flex()
                    .my_auto()
                    .w_full()
                    .max_w(LAUNCHER_MAX_WIDTH)
                    .gap_4()
                    .child(
                        v_flex()
                            .px_1()
                            .gap_0p5()
                            .child(
                                div()
                                    .text_size(TEXT_MESSAGE)
                                    .font_semibold()
                                    .child("Open a tool"),
                            )
                            .child(
                                div()
                                    .text_size(TEXT_CAPTION)
                                    .text_color(palette.ink_3)
                                    .child(
                                        "It opens here, beside the chat. Press its key to jump in.",
                                    ),
                            ),
                    )
                    .child(v_flex().children(tool_groups(&self.task_context).map(
                        |(group, tools)| {
                            v_flex()
                                .child(
                                    div()
                                        .pt_2()
                                        .px_2p5()
                                        .pb_1()
                                        .text_xs()
                                        .font_semibold()
                                        .text_color(palette.ink_3)
                                        .child(group),
                                )
                                .children(
                                    tools.map(|tool| self.render_tool(tool, palette, &mono, cx)),
                                )
                        },
                    ))),
            )
    }

    fn render_tool(
        &self,
        tool: Tool,
        palette: &Palette,
        mono: &SharedString,
        cx: &Context<Self>,
    ) -> Button {
        let ink = if tool.available {
            palette.ink
        } else {
            palette.ink_disabled
        };
        let key = keycap(shortcut(tool.key), palette, mono.clone())
            .w_5()
            .when(!tool.available, |key| {
                key.bg(cx.theme().transparent)
                    .border_color(palette.line)
                    .text_color(palette.ink_disabled)
            });

        Button::new(SharedString::from(format!("tool-{}", tool.label)))
            .ghost()
            .w_full()
            .h_10()
            .px_2()
            .disabled(!tool.available)
            .child(
                h_flex()
                    .flex_1()
                    .min_w_0()
                    .gap_3()
                    .text_color(ink)
                    .child(
                        div()
                            .flex()
                            .flex_none()
                            .items_center()
                            .justify_center()
                            .size(rems(1.625))
                            .child(Icon::new(tool.icon).size_3p5()),
                    )
                    .child(div().flex_none().font_medium().child(tool.label))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .text_ellipsis()
                            .text_size(TEXT_CAPTION)
                            .text_color(palette.ink_3)
                            .child(tool.hint),
                    )
                    .child(key),
            )
            .when_some(tool.opens, |row, surface| {
                row.on_click(cx.listener(move |this, _, window, cx| {
                    this.set_tool_surface(surface, window, cx);
                }))
            })
    }

    fn render_context(&self, palette: &Palette, cx: &Context<Self>) -> impl IntoElement {
        let context = &self.task_context;
        let mono = cx.theme().mono_font_family.clone();
        let used_percent = (context.context_window_used.clamp(0., 1.) * 100.).round() as u8;

        v_flex()
            .id("task-context")
            .flex_1()
            .min_h_0()
            .pt_5()
            .px(rems(1.125))
            .pb(rems(1.125))
            .gap_6()
            .overflow_y_scrollbar()
            .child(
                v_flex()
                    .gap_0p5()
                    .child(div().font_semibold().child("Context"))
                    .child(
                        div()
                            .text_size(TEXT_CAPTION)
                            .text_color(palette.ink_3)
                            .child("What this task can see and use"),
                    ),
            )
            .child(context_section(
                "Project",
                palette,
                [
                    context_row(
                        IconName::GitBranch,
                        "Version control",
                        Some(if context.git_initialized {
                            "Git is set up"
                        } else {
                            "Git isn’t set up yet"
                        }),
                        (!context.git_initialized)
                            .then(|| small_button("set-up-git-context", "Set up", palette)),
                        palette,
                    ),
                    context_row(
                        IconName::SquarePlay,
                        "Project actions",
                        Some(if context.saved_commands == 0 {
                            "No saved commands yet"
                        } else {
                            "Saved commands run in one click"
                        }),
                        Some(small_button("add-project-action", "Add", palette)),
                        palette,
                    ),
                ],
            ))
            .child(context_section(
                "Knowledge",
                palette,
                [
                    context_row(
                        IconName::FileText,
                        "Project rules",
                        Some(if context.has_rules_file {
                            "AGENTS.md"
                        } else {
                            "No AGENTS.md found"
                        }),
                        (!context.has_rules_file)
                            .then(|| small_button("generate-rules", "Generate", palette)),
                        palette,
                    ),
                    context_row(
                        IconName::Bookmark,
                        "Memory",
                        Some("Corrections you make are kept here"),
                        Some(count_label(context.memories, mono.clone(), palette)),
                        palette,
                    ),
                ],
            ))
            .child(context_section(
                "Tools",
                palette,
                [
                    context_row(
                        IconName::SquareTerminal,
                        "Terminal",
                        None,
                        Some(
                            Switch::new("terminal-tool")
                                .accessibility_label("Terminal")
                                .checked(context.terminal_enabled)
                                .color(palette.accent)
                                .on_change(cx.listener(|this, on: &bool, _, cx| {
                                    this.task_context.terminal_enabled = *on;
                                    cx.notify();
                                }))
                                .into_any_element(),
                        ),
                        palette,
                    ),
                    context_row(
                        IconName::Globe,
                        "Browser",
                        None,
                        Some(
                            Switch::new("browser-tool")
                                .accessibility_label("Browser")
                                .checked(context.browser_enabled)
                                .color(palette.accent)
                                .on_change(cx.listener(|this, on: &bool, _, cx| {
                                    this.task_context.browser_enabled = *on;
                                    cx.notify();
                                }))
                                .into_any_element(),
                        ),
                        palette,
                    ),
                    context_row(
                        IconName::Plug,
                        "MCP servers",
                        None,
                        Some(if context.mcp_servers == 0 {
                            small_button("connect-mcp", "Connect", palette)
                        } else {
                            count_label(context.mcp_servers, mono.clone(), palette)
                        }),
                        palette,
                    ),
                ],
            ))
            .child(
                v_flex()
                    .mt_auto()
                    .pt_4()
                    .gap_2()
                    .border_t_1()
                    .border_color(palette.line)
                    .child(
                        h_flex()
                            .justify_between()
                            .text_size(TEXT_CAPTION)
                            .child(div().text_color(palette.ink_3).child("Context window"))
                            .child(
                                div()
                                    .font_family(mono)
                                    .child(format!("{used_percent}% used")),
                            ),
                    )
                    .child(
                        Progress::new("context-window")
                            .small()
                            .accessibility_label("Context window used")
                            .value(f32::from(used_percent))
                            .color(palette.accent),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(palette.ink_3)
                            .child("Older messages are compacted automatically as it fills up."),
                    ),
            )
    }
}

/// A titled group of rows in the Context view.
fn context_section(
    title: &'static str,
    palette: &Palette,
    rows: impl IntoIterator<Item = impl IntoElement>,
) -> impl IntoElement {
    v_flex()
        .gap_1p5()
        .child(
            div()
                .text_xs()
                .font_semibold()
                .text_color(palette.ink_3)
                .child(title),
        )
        .children(rows)
}

/// One thing the task can use: icon, name, its state, and a trailing control.
fn context_row(
    icon: IconName,
    name: &'static str,
    status: Option<&'static str>,
    trailing: Option<AnyElement>,
    palette: &Palette,
) -> impl IntoElement {
    h_flex()
        .min_h_10()
        .gap_2p5()
        .child(Icon::new(icon).size_4().text_color(palette.ink_2))
        .child(
            v_flex()
                .flex_1()
                .min_w_0()
                .child(div().text_size(TEXT_CONTROL).font_medium().child(name))
                .when_some(status, |column, status| {
                    column.child(div().text_xs().text_color(palette.ink_3).child(status))
                }),
        )
        .children(trailing)
}

/// A mono count shown at the trailing edge of a Context row.
fn count_label(count: usize, mono: SharedString, palette: &Palette) -> AnyElement {
    div()
        .font_family(mono)
        .text_xs()
        .text_color(palette.ink_3)
        .child(count.to_string())
        .into_any_element()
}

/// A 28 px outline button for a row's one setup command.
fn small_button(id: &'static str, label: &'static str, palette: &Palette) -> AnyElement {
    Button::new(id)
        .outline()
        .h_7()
        .px_2p5()
        .label(label)
        .text_size(TEXT_CAPTION)
        .font_medium()
        .bg(palette.surface)
        .into_any_element()
}
