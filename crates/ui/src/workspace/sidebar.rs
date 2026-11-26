//! Task sidebar: search and new-task commands, task groups, and footer tools.

use gpui_kit::assets::IconName;
use gpui_kit::component::button::{Button, ButtonVariants as _};
use gpui_kit::component::kbd::Kbd;
use gpui_kit::component::scroll::ScrollableElement as _;
use gpui_kit::component::spinner::Spinner;
use gpui_kit::component::{
    ActiveTheme as _, Icon, Selectable as _, Sizable as _, StyledExt as _, ThemeMode, h_flex,
    v_flex,
};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    AnyElement, App, InteractiveElement as _, IntoElement, ParentElement as _, Rems, SharedString,
    Styled as _, div, rems,
};
use onecode_core::{Task, TaskGroup, TaskGroupKind, TaskStatus};

use super::{icon_button, keycap, project_badge, shortcut, sidebar_toggle};
use crate::theme::{self, Palette, TEXT_CONTROL, TEXT_KBD};

/// Height of the search and new-task commands (38 px).
const COMMAND_HEIGHT: Rems = rems(2.375);

/// The sidebar for `groups`, highlighting `selected_task`. The workspace owns
/// the tasks and the sidebar's width and visibility.
pub(super) fn sidebar(groups: &[TaskGroup], selected_task: &str, cx: &App) -> impl IntoElement {
    let palette = Palette::global(cx);
    let mono = cx.theme().mono_font_family.clone();

    // Each block carries the horizontal inset itself so the task list's
    // scrollbar can sit on the sidebar's trailing edge.
    v_flex()
        .size_full()
        .pt_3p5()
        .pb_3()
        .gap_3p5()
        .child(
            v_flex()
                .pl_3()
                .pr_2p5()
                .gap_3p5()
                .child(window_controls_row())
                .child(search_command(palette, mono.clone()))
                .child(new_task_command(palette, mono.clone())),
        )
        .child(
            v_flex()
                .id("task-groups")
                .flex_1()
                .min_h_0()
                .pl_3()
                .pr_2p5()
                .gap_3p5()
                .overflow_y_scrollbar()
                .children(
                    groups
                        .iter()
                        .map(|group| task_group(group, selected_task, palette, &mono)),
                ),
        )
        .child(footer(palette, cx))
}

/// The row the macOS window controls sit in, with the sidebar toggle at its end.
fn window_controls_row() -> impl IntoElement {
    h_flex().h_6().px_1().justify_end().child(sidebar_toggle())
}

fn search_command(palette: &Palette, mono: SharedString) -> Button {
    Button::new("search")
        .outline()
        .w_full()
        .h(COMMAND_HEIGHT)
        .child(
            h_flex()
                .flex_1()
                .gap_2p5()
                .text_size(TEXT_CONTROL)
                .text_color(palette.ink_2)
                .child(Icon::new(IconName::Search).size_4())
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .text_ellipsis()
                        .child("Search or jump to…"),
                )
                .child(keycap(shortcut("cmd-k"), palette, mono)),
        )
}

fn new_task_command(palette: &Palette, mono: SharedString) -> Button {
    Button::new("new-task")
        .outline()
        .w_full()
        .h(COMMAND_HEIGHT)
        .child(
            h_flex()
                .flex_1()
                .gap_2p5()
                .text_size(TEXT_CONTROL)
                .text_color(palette.ink)
                .child(Icon::new(IconName::SquarePen).size_4())
                .child(div().flex_1().font_semibold().child("New task"))
                .child(
                    div()
                        .font_family(mono)
                        .text_size(TEXT_KBD)
                        .font_medium()
                        .text_color(palette.ink_3)
                        .child(Kbd::new(shortcut("cmd-n")).appearance(false)),
                ),
        )
}

fn task_group(
    group: &TaskGroup,
    selected_task: &str,
    palette: &Palette,
    mono: &SharedString,
) -> impl IntoElement {
    let header: AnyElement = match &group.kind {
        TaskGroupKind::NeedsYou => {
            status_header("Needs you", group.tasks.len(), palette, mono).into_any_element()
        }
        TaskGroupKind::Running => {
            status_header("Running", group.tasks.len(), palette, mono).into_any_element()
        }
        TaskGroupKind::Project(name) => project_header(name, palette).into_any_element(),
    };

    v_flex().gap_0p5().child(header).children(
        group
            .tasks
            .iter()
            .map(|task| task_row(task, task.id == selected_task, palette)),
    )
}

fn status_header(
    label: &'static str,
    count: usize,
    palette: &Palette,
    mono: &SharedString,
) -> impl IntoElement {
    h_flex()
        .justify_between()
        .px_2()
        .py_1()
        .text_xs()
        .font_semibold()
        .text_color(palette.ink_3)
        .child(label)
        .child(
            div()
                .font_family(mono.clone())
                .font_medium()
                .child(count.to_string()),
        )
}

fn project_header(name: &str, palette: &Palette) -> impl IntoElement {
    let new_task_label = SharedString::from(format!("New task in {name}"));
    h_flex()
        .gap_2()
        .py_1()
        .pl_2()
        .pr_1()
        .text_xs()
        .font_semibold()
        .text_color(palette.ink_3)
        .child(project_badge(name, palette).size_4().rounded_sm())
        .child(div().flex_1().child(name.to_string()))
        .child(
            Button::new(SharedString::from(format!("new-task-in-{name}")))
                .ghost()
                .small()
                .icon(IconName::Plus)
                .accessibility_label(new_task_label.clone())
                .tooltip(new_task_label),
        )
}

fn task_row(task: &Task, selected: bool, palette: &Palette) -> Button {
    let status_icon: AnyElement = match task.status {
        TaskStatus::Waiting => Icon::new(IconName::CirclePause)
            .size_4()
            .text_color(palette.waiting)
            .into_any_element(),
        TaskStatus::Running => Spinner::new().color(palette.accent).into_any_element(),
        TaskStatus::Idle => div()
            .flex()
            .items_center()
            .justify_center()
            .size_4()
            .child(div().size_2().rounded_full().bg(palette.ring_track))
            .into_any_element(),
    };

    Button::new(SharedString::from(task.id.clone()))
        .ghost()
        .w_full()
        .h_auto()
        .p_2()
        .when(selected, |row| {
            row.selected(true)
                .bg(palette.surface)
                .shadow(palette.shadow_raised.clone())
        })
        .child(
            h_flex()
                .flex_1()
                .min_w_0()
                .items_start()
                .gap_2p5()
                .text_color(palette.ink)
                .child(div().mt_0p5().child(status_icon))
                .child(
                    v_flex()
                        .flex_1()
                        .min_w_0()
                        .child(
                            div()
                                .text_ellipsis()
                                .map(|title| {
                                    if selected {
                                        title.font_semibold()
                                    } else {
                                        title.font_medium()
                                    }
                                })
                                .child(task.title.clone()),
                        )
                        .when_some(task.detail.as_ref(), |column, detail| {
                            column.child(
                                div()
                                    .text_ellipsis()
                                    .text_xs()
                                    .text_color(palette.ink_3)
                                    .child(format!("{} · {detail}", task.project)),
                            )
                        }),
                )
                .when_some(task.age.clone(), |row, age| {
                    row.child(div().text_xs().text_color(palette.ink_3).child(age))
                }),
        )
}

fn footer(palette: &Palette, cx: &App) -> impl IntoElement {
    let (theme_label, theme_icon, next_mode) = if cx.theme().mode.is_dark() {
        ("Switch to light theme", IconName::Sun, ThemeMode::Light)
    } else {
        ("Switch to dark theme", IconName::Moon, ThemeMode::Dark)
    };

    h_flex()
        .gap_0p5()
        .pl_3p5()
        .pr_3()
        .children(
            [
                ("settings", IconName::Settings, "Settings"),
                ("reviews", IconName::GitPullRequest, "Reviews"),
                ("issues", IconName::CircleDot, "Issues"),
                ("automations", IconName::Clock, "Automations"),
                ("usage", IconName::ChartColumn, "Usage"),
            ]
            .map(|(id, icon, label)| icon_button(id, icon, label)),
        )
        .child(
            icon_button("theme", theme_icon, theme_label)
                .on_click(move |_, _, cx| theme::set_mode(next_mode, cx)),
        )
        .child(div().flex_1())
        .child(
            div()
                .relative()
                .child(icon_button(
                    "update",
                    IconName::Download,
                    "Update available",
                ))
                .child(
                    // A 7 px accent dot inside a 2 px ring of the sidebar color.
                    div()
                        .absolute()
                        .top_1()
                        .right_1()
                        .size(rems(0.6875))
                        .rounded_full()
                        .border_2()
                        .border_color(palette.bg)
                        .bg(palette.accent),
                ),
        )
}
