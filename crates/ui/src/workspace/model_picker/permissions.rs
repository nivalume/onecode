//! The picker's permissions page: how much the agent may do in this task
//! before asking, shown as a capability matrix per level.

use gpui_kit::assets::IconName;
use gpui_kit::component::button::{Button, ButtonVariants as _};
use gpui_kit::component::checkbox::Checkbox;
use gpui_kit::component::scroll::ScrollableElement as _;
use gpui_kit::component::{Icon, Selectable as _, StyledExt as _, h_flex, v_flex};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    Context, Div, InteractiveElement as _, IntoElement, ParentElement as _, Rems, SharedString,
    Styled as _, div, linear_color_stop, linear_gradient, px, rems,
};
use onecode_core::{Autonomy, Capability, PermissionLevel};

use super::{HEADER_HEIGHT, ModelPicker, Page};
use crate::theme::{Palette, TEXT_CAPTION, TEXT_CONTROL};

/// Width of each capability column (52 px).
const CAPABILITY_COLUMN: Rems = rems(3.25);
/// Inset that lines the column headings up with the rows' contents (13 px):
/// the rows' 12 px padding plus their 1 px border.
const MATRIX_INSET: Rems = rems(0.8125);

impl ModelPicker {
    pub(super) fn render_permissions(&self, cx: &Context<Self>) -> impl IntoElement {
        let palette = Palette::global(cx);
        v_flex()
            .size_full()
            .child(
                h_flex()
                    .flex_none()
                    .h(HEADER_HEIGHT)
                    .pl_2()
                    .pr_4()
                    .gap_2p5()
                    .border_b_1()
                    .border_color(palette.line)
                    .child(
                        Button::new("back-to-models")
                            .ghost()
                            .h(rems(1.875))
                            .px_2()
                            .child(
                                h_flex()
                                    .gap_1()
                                    .text_size(TEXT_CONTROL)
                                    .child(Icon::new(IconName::ChevronLeft).size_3())
                                    .child("Models"),
                            )
                            .on_click(
                                cx.listener(|this, _, _, cx| this.show_page(Page::Models, cx)),
                            ),
                    )
                    .child(div().w_px().h_4().bg(palette.border))
                    .child(div().text_sm().font_semibold().child("Permissions"))
                    .child(div().flex_1())
                    .child(
                        div()
                            .text_size(TEXT_CAPTION)
                            .text_color(palette.ink_3)
                            .child("Applies to this task"),
                    ),
            )
            .child(
                v_flex()
                    .id("permission-levels")
                    .flex_1()
                    .min_h_0()
                    .px_3()
                    .pt_2p5()
                    .pb_2()
                    .gap_1p5()
                    .overflow_y_scrollbar()
                    .child(
                        h_flex()
                            .px(MATRIX_INSET)
                            .text_xs()
                            .font_semibold()
                            .text_color(palette.ink_3)
                            .child(div().flex_1())
                            .children(Capability::ALL.map(|capability| {
                                div()
                                    .w(CAPABILITY_COLUMN)
                                    .text_center()
                                    .child(capability_name(capability))
                            })),
                    )
                    .children(
                        PermissionLevel::ALL.map(|level| self.render_level(level, palette, cx)),
                    )
                    .child(
                        h_flex()
                            .flex_wrap()
                            .gap_4()
                            .pt_1p5()
                            .px(MATRIX_INSET)
                            .text_xs()
                            .text_color(palette.ink_3)
                            .children(
                                [
                                    (Autonomy::OnItsOwn, "Does it on its own"),
                                    (Autonomy::SafeOnly, "Safe commands only"),
                                    (Autonomy::AsksFirst, "Asks you first"),
                                ]
                                .map(|(autonomy, meaning)| {
                                    h_flex()
                                        .gap_1p5()
                                        .child(autonomy_dot(autonomy, rems(0.5625), palette))
                                        .child(meaning)
                                }),
                            ),
                    ),
            )
            .child(
                h_flex()
                    .flex_none()
                    .h_12()
                    .pl_4()
                    .pr_2p5()
                    .gap_2p5()
                    .border_t_1()
                    .border_color(palette.line)
                    .child(
                        Checkbox::new("confirm-destructive-actions")
                            .label("Always ask before deleting files or pushing to Git")
                            .checked(self.confirm_destructive_actions)
                            .text_size(TEXT_CONTROL)
                            .on_change(cx.listener(|this, checked: &bool, _, cx| {
                                this.confirm_destructive_actions = *checked;
                                cx.notify();
                            })),
                    )
                    .child(div().flex_1())
                    .child(
                        Button::new("permissions-done")
                            .primary()
                            .h_8()
                            .px_4()
                            .accessibility_label("Done")
                            .child(div().text_size(TEXT_CONTROL).font_semibold().child("Done"))
                            .on_click(
                                cx.listener(|this, _, _, cx| this.show_page(Page::Models, cx)),
                            ),
                    ),
            )
    }

    fn render_level(
        &self,
        level: PermissionLevel,
        palette: &Palette,
        cx: &Context<Self>,
    ) -> Button {
        let selected = level == self.level;
        Button::new(SharedString::from(format!("level-{}", level_name(level))))
            .outline()
            .w_full()
            .h_auto()
            .px_3()
            .py_2p5()
            .rounded(px(10.))
            .selected(selected)
            .accessibility_label(format!(
                "{}. {}",
                level_name(level),
                level_description(level)
            ))
            .map(|row| {
                if selected {
                    row.bg(palette.accent_soft).border_color(palette.accent)
                } else {
                    row.bg(palette.surface)
                }
            })
            .child(
                h_flex()
                    .w_full()
                    .text_color(palette.ink)
                    .child(
                        // Radio indicator: a thick accent ring when chosen.
                        div()
                            .flex_none()
                            .size_4()
                            .rounded_full()
                            .bg(palette.surface)
                            .map(|radio| {
                                if selected {
                                    radio.border_4().border_color(palette.accent)
                                } else {
                                    radio.border_1().border_color(palette.ink_disabled)
                                }
                            }),
                    )
                    .child(
                        v_flex()
                            .flex_1()
                            .min_w_0()
                            .pl_2()
                            .whitespace_normal()
                            .child(
                                h_flex()
                                    .gap_2()
                                    .font_semibold()
                                    .child(level_name(level))
                                    .when(level == PermissionLevel::FullAccess, |name| {
                                        name.child(
                                            div()
                                                .px_2()
                                                .rounded_full()
                                                .border_1()
                                                .border_color(palette.warn_border)
                                                .bg(palette.warn_bg)
                                                .text_color(palette.warn)
                                                .text_xs()
                                                .child("Use with care"),
                                        )
                                    }),
                            )
                            .child(
                                div()
                                    .text_size(TEXT_CAPTION)
                                    .text_color(palette.ink_3)
                                    .child(level_description(level)),
                            ),
                    )
                    .children(Capability::ALL.map(|capability| {
                        div()
                            .flex()
                            .flex_none()
                            .justify_center()
                            .w(CAPABILITY_COLUMN)
                            .child(autonomy_dot(
                                level.autonomy(capability),
                                rems(0.625),
                                palette,
                            ))
                    })),
            )
            .on_click(cx.listener(move |this, _, _, cx| {
                this.level = level;
                cx.notify();
            }))
    }
}

/// How the picker names a permission level.
pub(super) fn level_name(level: PermissionLevel) -> &'static str {
    match level {
        PermissionLevel::AskFirst => "Ask first",
        PermissionLevel::AutoEdit => "Auto-edit",
        PermissionLevel::Auto => "Auto",
        PermissionLevel::FullAccess => "Full access",
    }
}

fn level_description(level: PermissionLevel) -> &'static str {
    match level {
        PermissionLevel::AskFirst => "Asks before every edit and command.",
        PermissionLevel::AutoEdit => "Edits files on its own. Asks before running commands.",
        PermissionLevel::Auto => "Runs commands it judges safe. Asks before anything risky.",
        PermissionLevel::FullAccess => "Never asks. Best in a sandbox you can throw away.",
    }
}

fn capability_name(capability: Capability) -> &'static str {
    match capability {
        Capability::Read => "Read",
        Capability::Edit => "Edit",
        Capability::Run => "Run",
        Capability::Web => "Web",
    }
}

/// A filled, half-filled or hollow dot for what the agent does on its own.
fn autonomy_dot(autonomy: Autonomy, size: Rems, palette: &Palette) -> Div {
    let dot = div().flex_none().size(size).rounded_full();
    match autonomy {
        Autonomy::OnItsOwn => dot.bg(palette.accent),
        Autonomy::SafeOnly => dot
            .border_1()
            .border_color(palette.accent)
            .bg(linear_gradient(
                90.,
                linear_color_stop(palette.accent, 0.5),
                linear_color_stop(palette.accent.opacity(0.), 0.5),
            )),
        Autonomy::AsksFirst => dot.border_1().border_color(palette.ink_disabled),
    }
}
