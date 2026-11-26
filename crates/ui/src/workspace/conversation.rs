//! The open task's conversation and the composer under it.

use gpui_kit::assets::IconName;
use gpui_kit::component::button::{Button, ButtonVariants as _};
use gpui_kit::component::input::{Textarea, TextareaState};
use gpui_kit::component::popover::Popover;
use gpui_kit::component::scroll::ScrollableElement as _;
use gpui_kit::component::{ActiveTheme as _, Icon, Sizable as _, StyledExt as _, h_flex, v_flex};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    Anchor, App, Context, Entity, InteractiveElement as _, IntoElement, Keystroke, Modifiers,
    ParentElement as _, Rems, SharedString, Styled as _, Window, div, relative, rems, svg,
};
use onecode_core::{Author, Message, Suggestion, SuggestionKind};

use super::model_picker::effort_label;
use super::{Workspace, icon_button, initial, keycap};
use crate::assets::ONECODE_MARK;
use crate::theme::{Palette, TEXT_CAPTION, TEXT_CONTROL, TEXT_MESSAGE};

/// `spacing.layout.thread-max` (720 px).
const THREAD_MAX_WIDTH: Rems = rems(45.);
/// `spacing.layout.chat-max` (760 px).
const COMPOSER_MAX_WIDTH: Rems = rems(47.5);
/// The model picker's size: wide enough for the list and its details (720 × 480 px).
const PICKER_WIDTH: Rems = rems(45.);
const PICKER_HEIGHT: Rems = rems(30.);
/// Narrowest a suggestion card gets before the row wraps (200 px).
const SUGGESTION_MIN_WIDTH: Rems = rems(12.5);

/// Placeholder that tells the person what the composer understands.
const COMPOSER_PLACEHOLDER: &str =
    "Describe the task. Type @ to mention files, $ for skills, / for commands";

/// The composer's text state, owned by the workspace.
pub(super) fn composer_state(
    window: &mut Window,
    cx: &mut Context<TextareaState>,
) -> TextareaState {
    TextareaState::new(window, cx)
        .placeholder(COMPOSER_PLACEHOLDER)
        .auto_grow(2, 8)
}

pub(super) fn conversation(messages: &[Message], cx: &App) -> impl IntoElement {
    let palette = Palette::global(cx);
    let mono = cx.theme().mono_font_family.clone();
    // The scroll region spans the panel so its scrollbar sits on the panel's
    // edge; the thread inside it is centered and capped.
    v_flex()
        .id("conversation")
        .flex_1()
        .min_h_0()
        .overflow_y_scrollbar()
        .child(
            v_flex()
                .w_full()
                .items_center()
                .pt_11()
                .px_8()
                .pb_6()
                .child(
                    v_flex()
                        .w_full()
                        .max_w(THREAD_MAX_WIDTH)
                        .gap(rems(1.875))
                        .children(
                            messages
                                .iter()
                                .map(|message| message_row(message, palette, &mono)),
                        ),
                ),
        )
}

pub(super) fn composer(
    input: &Entity<TextareaState>,
    model_chip: impl IntoElement,
    cx: &App,
) -> impl IntoElement {
    let palette = Palette::global(cx);
    v_flex()
        .flex_none()
        .items_center()
        .px_8()
        .pb(rems(1.125))
        .child(
            v_flex()
                .w_full()
                .max_w(COMPOSER_MAX_WIDTH)
                .gap_2p5()
                .pt_3p5()
                .pr_3()
                .pb_2p5()
                .pl_3p5()
                .rounded_2xl()
                .border_1()
                .border_color(palette.border)
                .bg(palette.surface)
                .shadow(palette.shadow_composer.clone())
                .child(
                    div().text_size(TEXT_MESSAGE).child(
                        Textarea::new(input)
                            .appearance(false)
                            .aria_label("Message OneCode"),
                    ),
                )
                .child(
                    h_flex()
                        .gap_1()
                        .child(icon_button(
                            "attach-files",
                            IconName::Paperclip,
                            "Attach files",
                        ))
                        .child(icon_button(
                            "share-screen",
                            IconName::Monitor,
                            "Share your screen with the agent",
                        ))
                        .child(icon_button("voice-input", IconName::Mic, "Voice input"))
                        .child(div().flex_1())
                        .child(model_chip)
                        .child(
                            Button::new("send")
                                .primary()
                                .icon(IconName::ArrowUp)
                                .size(rems(2.125))
                                .rounded_full()
                                .ml_1()
                                .accessibility_label("Send"),
                        ),
                ),
        )
}

impl Workspace {
    /// The composer's model chip, which opens the model picker above it.
    pub(super) fn render_model_chip(&self, cx: &mut Context<Self>) -> Popover {
        let palette = Palette::global(cx);
        let picker = self.model_picker.read(cx);
        let open = self.picker_open;
        let model_name = picker.model().to_string();
        let chip = Button::new("model")
            .ghost()
            .small()
            .h_7()
            .px_2p5()
            .accessibility_label(model_name.clone())
            .dropdown_caret(true)
            .text_color(palette.ink_2)
            // While the picker is open the chip keeps its pressed fill.
            .when(!open, |chip| chip.bg(palette.fill))
            .child(
                h_flex()
                    .gap_1p5()
                    .text_size(TEXT_CONTROL)
                    .child(div().font_medium().text_color(palette.ink).child(model_name))
                    .child(
                        div()
                            .text_size(TEXT_CAPTION)
                            .text_color(palette.ink_3)
                            .child(effort_label(picker.effort())),
                    ),
            );
        let search = picker.search_focus_handle(cx);
        let model_picker = self.model_picker.clone();

        Popover::new("model-picker")
            // Opens above the chip and extends toward the composer's start.
            .anchor(Anchor::BottomRight)
            .trigger(chip)
            .open(open)
            .on_open_change(cx.listener(|this, open: &bool, _, cx| {
                this.picker_open = *open;
                cx.notify();
            }))
            .track_focus(&search)
            .w(PICKER_WIDTH)
            .h(PICKER_HEIGHT)
            .p_0()
            .overflow_hidden()
            .rounded(rems(0.875))
            .border_1()
            .border_color(palette.border)
            .bg(palette.surface)
            .shadow(palette.shadow_popover.clone())
            .content(move |_, _, _| model_picker.clone())
    }
}

fn message_row(message: &Message, palette: &Palette, mono: &SharedString) -> impl IntoElement {
    h_flex()
        .items_start()
        .gap_3()
        .child(avatar(&message.author, palette))
        .child(
            v_flex()
                .flex_1()
                .min_w_0()
                .gap_3p5()
                .pt_0p5()
                .child(
                    v_flex()
                        .gap_0p5()
                        .child(byline(&message.author, palette))
                        .child(
                            div()
                                .text_size(TEXT_MESSAGE)
                                .line_height(relative(1.7))
                                .child(message.text.clone()),
                        ),
                )
                .when(!message.suggestions.is_empty(), |column| {
                    column.child(
                        h_flex().flex_wrap().items_stretch().gap_2p5().children(
                            message
                                .suggestions
                                .iter()
                                .zip(1..)
                                .map(|(suggestion, number)| {
                                    suggestion_card(suggestion, number, palette, mono)
                                }),
                        ),
                    )
                }),
        )
}

/// A 28 px avatar: the person's initial in a circle, or the OneCode mark.
fn avatar(author: &Author, palette: &Palette) -> impl IntoElement {
    let tile = div()
        .flex()
        .flex_none()
        .items_center()
        .justify_center()
        .size_7();
    match author {
        Author::Person { name } => tile
            .rounded_full()
            .bg(palette.fill_strong)
            .text_xs()
            .font_semibold()
            .child(initial(name)),
        Author::Agent { .. } => tile.rounded_lg().bg(palette.accent).child(
            svg()
                .path(ONECODE_MARK)
                .size_4()
                .text_color(palette.on_accent),
        ),
    }
}

/// Who wrote the message, and for the agent which model answered and how fast.
fn byline(author: &Author, palette: &Palette) -> impl IntoElement {
    let row = h_flex()
        .gap_2()
        .text_size(TEXT_CAPTION)
        .text_color(palette.ink_3);
    match author {
        Author::Person { .. } => {
            row.child(div().font_semibold().text_color(palette.ink_2).child("You"))
        }
        Author::Agent { model, elapsed } => row
            .child(
                div()
                    .font_semibold()
                    .text_color(palette.ink_2)
                    .child("OneCode"),
            )
            .child(format!("{model} · {elapsed}")),
    }
}

/// A suggested next step; its number is the key that picks it.
fn suggestion_card(
    suggestion: &Suggestion,
    number: usize,
    palette: &Palette,
    mono: &SharedString,
) -> Button {
    let icon = match suggestion.kind {
        SuggestionKind::SetUpGit => IconName::GitBranch,
        SuggestionKind::ExploreProject => IconName::Compass,
        SuggestionKind::SaveScripts => IconName::SquarePlay,
    };
    let number_key = Keystroke {
        modifiers: Modifiers::default(),
        key: number.to_string(),
        key_char: None,
    };

    Button::new(SharedString::from(format!(
        "suggestion-{}",
        suggestion.title
    )))
    .outline()
    .flex_1()
    .min_w(SUGGESTION_MIN_WIDTH)
    .h_auto()
    .p_3p5()
    .rounded_xl()
    .bg(palette.surface)
    .shadow(palette.shadow_card.clone())
    .child(
        v_flex()
            .w_full()
            .gap_3()
            .whitespace_normal()
            .text_color(palette.ink)
            .child(
                h_flex()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .size(rems(1.875))
                            .rounded_lg()
                            .bg(palette.accent_soft)
                            .text_color(palette.accent)
                            .child(Icon::new(icon).size_4()),
                    )
                    .child(keycap(number_key, palette, mono.clone())),
            )
            .child(
                v_flex()
                    .gap_0p5()
                    .child(div().font_semibold().child(suggestion.title.clone()))
                    .child(
                        div()
                            .text_size(TEXT_CONTROL)
                            .line_height(relative(1.5))
                            .text_color(palette.ink_2)
                            .child(suggestion.description.clone()),
                    ),
            ),
    )
}
