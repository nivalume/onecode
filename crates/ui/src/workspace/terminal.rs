//! The terminal region under the conversation: a shell on the session's
//! machine, started in the open task's project.

use gpui_kit::assets::IconName;
use gpui_kit::component::{ActiveTheme as _, Icon, StyledExt as _, h_flex, v_flex};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{App, IntoElement, ParentElement as _, Styled as _, div, rems};
use onecode_core::Machine;

use super::{ToggleTerminal, icon_button};
use crate::theme::{Palette, TEXT_CAPTION};

pub(super) fn terminal(machine: &Machine, project: Option<&str>, cx: &App) -> impl IntoElement {
    let palette = Palette::global(cx);
    let tab_label = match project {
        Some(project) => format!("{project} — {}", machine.shell),
        None => machine.shell.clone(),
    };
    v_flex()
        .size_full()
        .bg(palette.bg)
        .child(
            h_flex()
                .flex_none()
                .h_10()
                .pl_3()
                .pr_2()
                .gap_1()
                .child(
                    h_flex()
                        .h_7()
                        .px_2p5()
                        .gap_1p5()
                        .rounded(rems(0.4375))
                        .bg(palette.fill)
                        .text_size(TEXT_CAPTION)
                        .font_semibold()
                        .child(Icon::new(IconName::SquareTerminal).size_3p5())
                        .child(tab_label),
                )
                .child(icon_button("new-terminal", IconName::Plus, "New terminal").size_7())
                .child(div().flex_1())
                .child(icon_button("split-right", IconName::Columns2, "Split right").size_7())
                .child(icon_button("split-down", IconName::Rows2, "Split down").size_7())
                .child(
                    icon_button("close-terminal", IconName::Close, "Close terminal")
                        .size_7()
                        .on_click(|_, window, cx| {
                            window.dispatch_action(Box::new(ToggleTerminal), cx)
                        }),
                ),
        )
        .child(
            h_flex()
                .flex_1()
                .min_h_0()
                .items_start()
                .px_4()
                .pt_1p5()
                .pb_3()
                .font_family(cx.theme().mono_font_family.clone())
                .text_size(TEXT_CAPTION)
                .line_height(rems(1.25))
                .text_color(palette.ink_2)
                .child(
                    div()
                        .text_color(palette.accent)
                        .child(format!("{}@{}", machine.user, machine.host_name)),
                )
                .when_some(project, |prompt, project| {
                    prompt.child(
                        div()
                            .ml_2()
                            .text_color(palette.ink)
                            .child(project.to_string()),
                    )
                })
                .child(div().ml_2().child("%"))
                .child(
                    // Block cursor, 7 × 15 px.
                    div()
                        .ml_2()
                        .mt(rems(0.125))
                        .w(rems(0.4375))
                        .h(rems(0.9375))
                        .bg(palette.ink),
                ),
        )
}
