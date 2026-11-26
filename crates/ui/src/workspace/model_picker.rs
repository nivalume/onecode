//! The model picker: choose the model, its reasoning effort and favorites,
//! and go on to the task's permissions.

mod permissions;

use gpui_kit::assets::IconName;
use gpui_kit::component::button::{Button, ButtonVariants as _};
use gpui_kit::component::checkbox::Checkbox;
use gpui_kit::component::input::{Input, InputEvent, InputState};
use gpui_kit::component::scroll::ScrollableElement as _;
use gpui_kit::component::{
    ActiveTheme as _, Icon, Selectable as _, Sizable as _, StyledExt as _, h_flex, v_flex,
};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    AnyElement, App, AppContext as _, Context, Entity, FocusHandle, Focusable as _,
    InteractiveElement as _, IntoElement, Keystroke, Modifiers, ParentElement as _, Rems, Render,
    SharedString, Styled as _, Subscription, Window, div, px, relative, rems,
};
use onecode_core::{Effort, Model, PermissionLevel, Provider, sample};

use super::{icon_button, initial, keycap, shortcut};
use crate::theme::{Palette, TEXT_CAPTION, TEXT_CONTROL, TEXT_KBD, TEXT_MESSAGE};

/// Height of the picker's header rows (52 px).
const HEADER_HEIGHT: Rems = rems(3.25);
/// Width of the selected model's details column (256 px).
const DETAILS_WIDTH: Rems = rems(16.);

/// Which page of the picker is showing.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Page {
    Models,
    Permissions,
}

/// Which models the list shows.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ModelFilter {
    All,
    Favorites,
    Provider(Provider),
}

/// Picks the model, reasoning effort and permission level for new messages
/// in a task. It owns that choice; the composer's chip reads it from here.
pub(super) struct ModelPicker {
    query: Entity<InputState>,
    page: Page,
    filter: ModelFilter,
    models: Vec<Model>,
    /// Starred model names, in the order ⌘1–9 picks them.
    favorites: Vec<String>,
    model: String,
    effort: Effort,
    level: PermissionLevel,
    /// The open task's project, which "Use for new tasks" applies to.
    project: Option<SharedString>,
    use_for_new_tasks: bool,
    confirm_destructive_actions: bool,
    _query_changed: Subscription,
}

impl ModelPicker {
    pub(super) fn new(
        project: Option<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let query =
            cx.new(|cx| InputState::new(window, cx).placeholder("Search models or providers"));
        let _query_changed = cx.subscribe(&query, |_, _, event: &InputEvent, cx| {
            if matches!(event, InputEvent::Change) {
                cx.notify();
            }
        });
        Self {
            query,
            page: Page::Models,
            filter: ModelFilter::All,
            models: sample::models(),
            favorites: sample::FAVORITE_MODELS.map(String::from).to_vec(),
            model: sample::MODEL.into(),
            effort: sample::EFFORT,
            level: sample::PERMISSION_LEVEL,
            project,
            use_for_new_tasks: false,
            confirm_destructive_actions: true,
            _query_changed,
        }
    }

    pub(super) fn model(&self) -> &str {
        &self.model
    }

    pub(super) fn effort(&self) -> Effort {
        self.effort
    }

    /// The search field, which takes focus when the picker opens.
    pub(super) fn search_focus_handle(&self, cx: &App) -> FocusHandle {
        self.query.read(cx).focus_handle(cx)
    }

    fn show_page(&mut self, page: Page, cx: &mut Context<Self>) {
        self.page = page;
        cx.notify();
    }

    /// The models the current filter and search query let through.
    fn visible_models(&self, cx: &App) -> Vec<&Model> {
        let query = self.query.read(cx).value().trim().to_lowercase();
        let matches_query = |model: &&Model| {
            query.is_empty()
                || model.name.to_lowercase().contains(&query)
                || model.provider.name().to_lowercase().contains(&query)
        };
        match self.filter {
            ModelFilter::All => self.models.iter().filter(matches_query).collect(),
            ModelFilter::Favorites => self
                .favorites
                .iter()
                .filter_map(|name| self.models.iter().find(|model| &model.name == name))
                .filter(matches_query)
                .collect(),
            ModelFilter::Provider(provider) => self
                .models
                .iter()
                .filter(|model| model.provider == provider)
                .filter(matches_query)
                .collect(),
        }
    }

    fn render_models(&self, cx: &Context<Self>) -> impl IntoElement {
        let palette = Palette::global(cx);
        let mono = cx.theme().mono_font_family.clone();
        v_flex()
            .size_full()
            .child(
                h_flex()
                    .flex_none()
                    .h(HEADER_HEIGHT)
                    .pl_4()
                    .pr_3p5()
                    .gap_2p5()
                    .border_b_1()
                    .border_color(palette.line)
                    .child(
                        Icon::new(IconName::Search)
                            .size_4()
                            .text_color(palette.ink_2),
                    )
                    .child(
                        div().flex_1().min_w_0().child(
                            Input::new(&self.query)
                                .appearance(false)
                                .aria_label("Search models"),
                        ),
                    )
                    .child(keycap(shortcut("escape"), palette, mono.clone())),
            )
            .child(self.render_filters(palette, &mono, cx))
            .child(
                h_flex()
                    .flex_1()
                    .min_h_0()
                    .items_stretch()
                    .child(self.render_model_list(palette, &mono, cx))
                    .child(self.render_details(palette, cx)),
            )
            .child(self.render_footer(palette, &mono, cx))
    }

    fn render_filters(
        &self,
        palette: &Palette,
        mono: &SharedString,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let mut filters = vec![
            (ModelFilter::All, "All", self.models.len()),
            (ModelFilter::Favorites, "Favorites", self.favorites.len()),
        ];
        for provider in Provider::ALL {
            let count = self
                .models
                .iter()
                .filter(|m| m.provider == provider)
                .count();
            filters.push((ModelFilter::Provider(provider), provider.name(), count));
        }

        h_flex()
            .flex_none()
            .gap_1()
            .px_3()
            .py_2()
            .border_b_1()
            .border_color(palette.line)
            .children(filters.into_iter().map(|(filter, label, count)| {
                let selected = filter == self.filter;
                Button::new(SharedString::from(format!("filter-{label}")))
                    .ghost()
                    .small()
                    .h_7()
                    .px_2p5()
                    .accessibility_label(label)
                    .selected(selected)
                    .when(selected, |tab| tab.bg(palette.primary))
                    .child(
                        h_flex()
                            .gap_1p5()
                            .text_size(TEXT_CONTROL)
                            .font_medium()
                            .text_color(if selected {
                                palette.on_primary
                            } else {
                                palette.ink_2
                            })
                            .child(label)
                            .child(
                                div()
                                    .font_family(mono.clone())
                                    .text_size(TEXT_KBD)
                                    .text_color(if selected {
                                        palette.on_primary.opacity(0.7)
                                    } else {
                                        palette.ink_3
                                    })
                                    .child(count.to_string()),
                            ),
                    )
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.filter = filter;
                        cx.notify();
                    }))
            }))
    }

    fn render_model_list(
        &self,
        palette: &Palette,
        mono: &SharedString,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let visible = self.visible_models(cx);
        let mut rows: Vec<AnyElement> = Vec::new();
        if visible.is_empty() {
            let query = self.query.read(cx).value();
            rows.push(
                div()
                    .my_6()
                    .mx_2p5()
                    .text_size(TEXT_CONTROL)
                    .text_color(palette.ink_3)
                    .child(format!("No models match “{query}”."))
                    .into_any_element(),
            );
        }
        let mut current_provider = None;
        for model in visible {
            // "All" labels each provider's run of models.
            if self.filter == ModelFilter::All && current_provider != Some(model.provider) {
                current_provider = Some(model.provider);
                rows.push(
                    div()
                        .pt_2p5()
                        .px_2()
                        .pb_1()
                        .text_xs()
                        .font_semibold()
                        .text_color(palette.ink_3)
                        .child(model.provider.name())
                        .into_any_element(),
                );
            }
            rows.push(
                self.render_model_row(model, palette, mono, cx)
                    .into_any_element(),
            );
        }

        v_flex()
            .id("model-list")
            .flex_1()
            .min_w_0()
            .px_2()
            .pt_1()
            .pb_2()
            .overflow_y_scrollbar()
            .children(rows)
    }

    fn render_model_row(
        &self,
        model: &Model,
        palette: &Palette,
        mono: &SharedString,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let selected = model.name == self.model;
        let favorite_ix = self.favorites.iter().position(|name| name == &model.name);
        let favorite_label: &'static str = if favorite_ix.is_some() {
            "Remove from favorites"
        } else {
            "Add to favorites"
        };
        let name = model.name.clone();
        let toggled_name = model.name.clone();

        h_flex()
            .h_9()
            .gap_0p5()
            .rounded_lg()
            .when(selected, |row| row.bg(palette.accent_soft))
            .child(
                Button::new(SharedString::from(format!("model-{}", model.name)))
                    .ghost()
                    .flex_1()
                    .h_9()
                    .pl_2()
                    .pr_1p5()
                    .selected(selected)
                    .when(selected, |row| row.bg(cx.theme().transparent))
                    .child(
                        h_flex()
                            .flex_1()
                            .min_w_0()
                            .gap_2p5()
                            .text_color(palette.ink)
                            .child(
                                div()
                                    .flex()
                                    .flex_none()
                                    .items_center()
                                    .justify_center()
                                    .size_5()
                                    .rounded(rems(0.375))
                                    .bg(palette.fill)
                                    .text_color(palette.ink_2)
                                    .text_size(TEXT_KBD)
                                    .font_bold()
                                    .child(initial(model.provider.name())),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .min_w_0()
                                    .text_ellipsis()
                                    .when(selected, |name| name.font_semibold())
                                    .child(model.name.clone()),
                            )
                            // ⌘1–9 picks the first nine favorites.
                            .when_some(favorite_ix.filter(|ix| *ix < 9), |row, ix| {
                                row.child(keycap(
                                    Keystroke {
                                        modifiers: Modifiers::command(),
                                        key: (ix + 1).to_string(),
                                        key_char: None,
                                    },
                                    palette,
                                    mono.clone(),
                                ))
                            })
                            .when(selected, |row| {
                                row.child(
                                    Icon::new(IconName::Check)
                                        .size_3p5()
                                        .text_color(palette.accent),
                                )
                            }),
                    )
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.model = name.clone();
                        cx.notify();
                    })),
            )
            .child(
                icon_button(
                    format!("favorite-{}", model.name),
                    IconName::Star,
                    favorite_label,
                )
                .size_7()
                .mr_1()
                .text_color(if favorite_ix.is_some() {
                    palette.favorite
                } else {
                    palette.ink_disabled
                })
                .on_click(cx.listener(move |this, _, _, cx| {
                    match this.favorites.iter().position(|name| name == &toggled_name) {
                        Some(ix) => {
                            this.favorites.remove(ix);
                        }
                        None => this.favorites.push(toggled_name.clone()),
                    }
                    cx.notify();
                })),
            )
    }

    fn render_details(&self, palette: &Palette, cx: &Context<Self>) -> impl IntoElement {
        let selected = self.models.iter().find(|model| model.name == self.model);
        let new_tasks_label = match &self.project {
            Some(project) => format!("Use for new tasks in {project}"),
            None => "Use for new tasks".to_string(),
        };

        v_flex()
            .id("model-details")
            .flex_none()
            .w(DETAILS_WIDTH)
            .px_4()
            .py(rems(1.125))
            .gap(rems(1.125))
            .border_l_1()
            .border_color(palette.line)
            .overflow_y_scrollbar()
            .when_some(selected, |column, model| {
                column.child(
                    v_flex()
                        .gap_1()
                        .child(
                            h_flex()
                                .gap_2()
                                .child(
                                    div()
                                        .flex()
                                        .flex_none()
                                        .items_center()
                                        .justify_center()
                                        .size(rems(1.375))
                                        .rounded(rems(0.375))
                                        .bg(palette.accent_soft)
                                        .text_color(palette.accent)
                                        .text_size(TEXT_KBD)
                                        .font_bold()
                                        .child(initial(model.provider.name())),
                                )
                                .child(
                                    div()
                                        .text_size(TEXT_MESSAGE)
                                        .font_semibold()
                                        .child(model.name.clone()),
                                ),
                        )
                        .child(
                            div()
                                .text_size(TEXT_CAPTION)
                                .text_color(palette.ink_3)
                                .child(model.provider.name()),
                        ),
                )
            })
            .child(
                v_flex()
                    .w_full()
                    .min_w_0()
                    .gap_2()
                    .child(
                        div()
                            .text_xs()
                            .font_semibold()
                            .text_color(palette.ink_3)
                            .child("Reasoning"),
                    )
                    .child(
                        h_flex()
                            .w_full()
                            .min_w_0()
                            .p_0p5()
                            .gap_0p5()
                            .rounded(rems(0.5625))
                            .bg(palette.fill)
                            .children(Effort::ALL.map(|effort| {
                                let selected = effort == self.effort;
                                let label = effort_label(effort);
                                Button::new(SharedString::from(format!("effort-{label}")))
                                    .ghost()
                                    .small()
                                    .flex_1()
                                    .min_w_0()
                                    .h_7()
                                    .px_1()
                                    .rounded(px(7.))
                                    .accessibility_label(label)
                                    .selected(selected)
                                    .when(selected, |segment| {
                                        segment
                                            .bg(palette.surface)
                                            .shadow(palette.shadow_thumb.clone())
                                    })
                                    .child(
                                        div()
                                            .text_size(TEXT_CAPTION)
                                            .map(|text| {
                                                if selected {
                                                    text.text_color(palette.ink).font_semibold()
                                                } else {
                                                    text.text_color(palette.ink_2).font_medium()
                                                }
                                            })
                                            .child(label),
                                    )
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.effort = effort;
                                        cx.notify();
                                    }))
                            })),
                    )
                    .child(
                        div()
                            .w_full()
                            .min_w_0()
                            .whitespace_normal()
                            .text_size(TEXT_CAPTION)
                            .line_height(relative(1.5))
                            .text_color(palette.ink_2)
                            .child(effort_hint(self.effort)),
                    ),
            )
            .child(
                Checkbox::new("use-for-new-tasks")
                    .label(new_tasks_label)
                    .checked(self.use_for_new_tasks)
                    .text_size(TEXT_CAPTION)
                    .on_change(cx.listener(|this, checked: &bool, _, cx| {
                        this.use_for_new_tasks = *checked;
                        cx.notify();
                    })),
            )
    }

    fn render_footer(
        &self,
        palette: &Palette,
        mono: &SharedString,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let risky = self.level == PermissionLevel::FullAccess;
        h_flex()
            .flex_none()
            .h_12()
            .pl_2()
            .pr_3p5()
            .gap_2p5()
            .border_t_1()
            .border_color(palette.line)
            .child(
                Button::new("permissions")
                    .outline()
                    .small()
                    .h_7()
                    .pl_2p5()
                    .pr_2()
                    .map(|button| {
                        if risky {
                            button
                                .bg(palette.warn_bg)
                                .border_color(palette.warn_border)
                                .text_color(palette.warn)
                        } else {
                            button.bg(palette.surface).text_color(palette.ink_2)
                        }
                    })
                    .child(
                        h_flex()
                            .gap_1p5()
                            .text_size(TEXT_CONTROL)
                            .child(
                                Icon::new(if risky {
                                    IconName::ShieldOff
                                } else {
                                    IconName::Shield
                                })
                                .size_3p5(),
                            )
                            .child("Permissions")
                            .child(
                                div()
                                    .font_semibold()
                                    .when(!risky, |name| name.text_color(palette.ink))
                                    .child(permissions::level_name(self.level)),
                            )
                            .child(Icon::new(IconName::ChevronRight).size_3()),
                    )
                    .on_click(cx.listener(|this, _, _, cx| this.show_page(Page::Permissions, cx))),
            )
            .child(div().flex_1())
            .child(
                div()
                    .font_family(mono.clone())
                    .text_size(TEXT_KBD)
                    .text_color(palette.ink_3)
                    .child("↑↓ move · ⏎ choose · ⌘1–9 favorites"),
            )
    }
}

impl Render for ModelPicker {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match self.page {
            Page::Models => self.render_models(cx).into_any_element(),
            Page::Permissions => self.render_permissions(cx).into_any_element(),
        }
    }
}

/// How the picker and the composer's chip name a reasoning effort.
pub(super) fn effort_label(effort: Effort) -> &'static str {
    match effort {
        Effort::Low => "Low",
        Effort::Medium => "Medium",
        Effort::High => "High",
        Effort::Extra => "Extra",
    }
}

/// When to pick each effort, shown under the segmented control.
fn effort_hint(effort: Effort) -> &'static str {
    match effort {
        Effort::Low => "Fastest. For quick questions and small edits.",
        Effort::Medium => "Balanced. Good for most everyday tasks.",
        Effort::High => "Thinks longer. For multi-file changes and debugging.",
        Effort::Extra => "Slowest and most thorough. For the hardest problems.",
    }
}
