//! The OneCode palette and type scale from `DESIGN.md`, and their projection
//! onto the gpui-kit theme.
//!
//! This module is the only place raw color values live. Views read OneCode
//! roles from [`Palette`]; the roles that stock gpui-kit components already
//! understand (background, border, primary, …) are also written into the
//! gpui-kit theme so those components match the design without per-call styling.

use gpui_kit::component::{Theme, ThemeMode};
use gpui_kit::{App, BoxShadow, Global, Hsla, Pixels, Rems, point, px, rems, rgb, rgba};

/// `typography.message`: conversation messages and the composer (15 px).
pub(crate) const TEXT_MESSAGE: Rems = rems(0.9375);
/// `typography.control`: buttons, chips, tabs, menu items (13 px).
pub(crate) const TEXT_CONTROL: Rems = rems(0.8125);
/// `typography.caption` and `typography.code`: metadata, hints and terminal
/// output (12.5 px).
pub(crate) const TEXT_CAPTION: Rems = rems(0.78125);
/// `typography.kbd`: keycaps, counts and shortcut hints (11 px, mono).
pub(crate) const TEXT_KBD: Rems = rems(0.6875);

/// Corner radius gpui-kit controls use by default: icon buttons and chips (8 px).
const CONTROL_RADIUS: Pixels = px(8.);

/// OneCode color roles for the current light or dark mode.
pub(crate) struct Palette {
    pub(crate) bg: Hsla,
    pub(crate) surface: Hsla,
    pub(crate) fill: Hsla,
    pub(crate) fill_strong: Hsla,
    pub(crate) line: Hsla,
    pub(crate) border: Hsla,
    pub(crate) ink: Hsla,
    pub(crate) ink_2: Hsla,
    pub(crate) ink_3: Hsla,
    pub(crate) ink_disabled: Hsla,
    pub(crate) accent: Hsla,
    pub(crate) accent_soft: Hsla,
    pub(crate) on_accent: Hsla,
    pub(crate) primary: Hsla,
    pub(crate) on_primary: Hsla,
    pub(crate) warn: Hsla,
    pub(crate) warn_bg: Hsla,
    pub(crate) warn_border: Hsla,
    pub(crate) waiting: Hsla,
    pub(crate) ring_track: Hsla,
    pub(crate) favorite: Hsla,
    /// Lifts the selected sidebar task off the window background.
    pub(crate) shadow_raised: Vec<BoxShadow>,
    /// Floats the composer and the session control above the conversation.
    pub(crate) shadow_composer: Vec<BoxShadow>,
    /// Barely lifts suggestion cards; none in dark mode.
    pub(crate) shadow_card: Vec<BoxShadow>,
    /// The topmost decision layer: the model picker.
    pub(crate) shadow_popover: Vec<BoxShadow>,
    /// The selected option's thumb in a segmented control.
    pub(crate) shadow_thumb: Vec<BoxShadow>,
}

impl Global for Palette {}

impl Palette {
    fn light() -> Self {
        Self {
            bg: rgb(0xfafafa).into(),
            surface: rgb(0xffffff).into(),
            fill: rgb(0xf2f3f5).into(),
            fill_strong: rgb(0xe9ebef).into(),
            line: rgb(0xeaebed).into(),
            border: rgb(0xe2e5e9).into(),
            ink: rgb(0x0f1115).into(),
            ink_2: rgb(0x555b65).into(),
            ink_3: rgb(0x666c76).into(),
            ink_disabled: rgb(0x9ca2ab).into(),
            accent: rgb(0x5b3df5).into(),
            accent_soft: rgba(0x5b3df51c).into(),
            on_accent: rgb(0xffffff).into(),
            primary: rgb(0x0f1115).into(),
            on_primary: rgb(0xffffff).into(),
            warn: rgb(0x9a4a06).into(),
            warn_bg: rgb(0xfff7ed).into(),
            warn_border: rgb(0xf3ddbb).into(),
            waiting: rgb(0xb45309).into(),
            ring_track: rgb(0xd5d8dd).into(),
            favorite: rgb(0xe09a2a).into(),
            shadow_raised: vec![
                shadow(0x0f11150d, 0., 0., 1.),
                shadow(0x0f111514, 1., 3., 0.),
            ],
            shadow_composer: vec![
                shadow(0x0f111512, 8., 28., 0.),
                shadow(0x0f11150a, 1., 2., 0.),
            ],
            shadow_card: vec![shadow(0x0f11150a, 1., 2., 0.)],
            shadow_popover: vec![
                shadow(0x0f111529, 24., 64., 0.),
                shadow(0x0f11150f, 2., 6., 0.),
            ],
            shadow_thumb: vec![shadow(0x0f11151f, 1., 2., 0.)],
        }
    }

    fn dark() -> Self {
        Self {
            bg: rgb(0x111214).into(),
            surface: rgb(0x1a1b1e).into(),
            fill: rgb(0x232428).into(),
            fill_strong: rgb(0x2c2d32).into(),
            line: rgb(0x26272b).into(),
            border: rgb(0x34363b).into(),
            ink: rgb(0xedeef0).into(),
            ink_2: rgb(0xa9aeb6).into(),
            ink_3: rgb(0x8f959e).into(),
            ink_disabled: rgb(0x5d6169).into(),
            accent: rgb(0x8b7bff).into(),
            accent_soft: rgba(0x8b7bff29).into(),
            on_accent: rgb(0x14102e).into(),
            primary: rgb(0xedeef0).into(),
            on_primary: rgb(0x111214).into(),
            warn: rgb(0xf5b36b).into(),
            warn_bg: rgba(0xf5a5241f).into(),
            warn_border: rgba(0xf5a52452).into(),
            waiting: rgb(0xf5a524).into(),
            ring_track: rgb(0x3a3c42).into(),
            favorite: rgb(0xf2b84b).into(),
            shadow_raised: vec![
                shadow(0xffffff0f, 0., 0., 1.),
                shadow(0x00000066, 1., 3., 0.),
            ],
            shadow_composer: vec![
                shadow(0x00000059, 8., 28., 0.),
                shadow(0x0000004d, 1., 2., 0.),
            ],
            shadow_card: Vec::new(),
            shadow_popover: vec![
                shadow(0x0000008c, 24., 64., 0.),
                shadow(0x00000066, 2., 6., 0.),
            ],
            shadow_thumb: vec![shadow(0x00000080, 1., 2., 0.)],
        }
    }

    /// The palette installed by the last [`set_mode`].
    pub(crate) fn global(cx: &App) -> &Self {
        cx.global::<Self>()
    }

    /// Writes the roles gpui-kit components read into its theme.
    fn project_onto(&self, theme: &mut Theme) {
        theme.background = self.bg;
        theme.foreground = self.ink;
        theme.border = self.border;
        theme.input = self.border;
        theme.ring = self.accent;
        theme.muted = self.fill;
        theme.muted_foreground = self.ink_3;
        theme.popover = self.surface;
        theme.popover_foreground = self.ink;
        theme.primary = self.primary;
        theme.primary_foreground = self.on_primary;
        // Primary buttons (Send, Done) are the near-black `primary`, dimmed a
        // little on hover and press.
        theme.button_primary = self.primary;
        theme.button_primary_foreground = self.on_primary;
        theme.button_primary_hover = self.primary.opacity(0.88);
        theme.button_primary_active = self.primary.opacity(0.8);
        // Outline and ghost buttons: resting ink, hover fill, pressed or
        // selected fill.
        theme.button_foreground = self.ink;
        theme.secondary_foreground = self.ink_2;
        theme.accent = self.fill;
        theme.accent_foreground = self.ink;
        theme.button_active = self.fill_strong;
        theme.secondary_active = self.fill_strong;
    }
}

/// A drop shadow below its element, from a `0xRRGGBBAA` color.
fn shadow(color: u32, offset_y: f32, blur: f32, spread: f32) -> BoxShadow {
    BoxShadow {
        color: rgba(color).into(),
        offset: point(px(0.), px(offset_y)),
        blur_radius: px(blur),
        spread_radius: px(spread),
        inset: false,
    }
}

/// Switches between the light and dark palettes and refreshes every window.
pub(crate) fn set_mode(mode: ThemeMode, cx: &mut App) {
    // `change` reloads gpui-kit's base theme for `mode`, replacing its colors,
    // so the OneCode roles go on top in a separate `update`.
    Theme::change(mode, None, cx);
    let palette = if mode.is_dark() {
        Palette::dark()
    } else {
        Palette::light()
    };
    Theme::update(cx, |theme| {
        palette.project_onto(theme);
        theme.radius = CONTROL_RADIUS;
    });
    cx.set_global(palette);
}
