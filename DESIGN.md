---
version: alpha
name: OneCode
description: >-
  Design system for OneCode, a desktop app where people hand coding tasks to an
  AI agent and review its work. Quiet neutral surfaces, one accent, dense but
  calm desktop chrome. Source: the "OneCode" design system attached to the
  "OneCode Design" canvas (7 artboards, 2026-11).
omitted:
  - section: motion
    reason: The design defines no transitions; state changes are immediate.
colors:
  primary: "#0F1115"
  light:
    bg: "#FAFAFA"
    surface: "#FFFFFF"
    fill: "#F2F3F5"
    fill-strong: "#E9EBEF"
    line: "#EAEBED"
    border: "#E2E5E9"
    ink: "#0F1115"
    ink-2: "#555B65"
    ink-3: "#666C76"
    ink-disabled: "#9CA2AB"
    accent: "#5B3DF5"
    accent-soft: "rgba(91, 61, 245, 0.11)"
    on-accent: "#FFFFFF"
    primary: "#0F1115"
    on-primary: "#FFFFFF"
    warn: "#9A4A06"
    warn-bg: "#FFF7ED"
    warn-border: "#F3DDBB"
    waiting: "#B45309"
    ring-track: "#D5D8DD"
    favorite: "#E09A2A"
  dark:
    bg: "#111214"
    surface: "#1A1B1E"
    fill: "#232428"
    fill-strong: "#2C2D32"
    line: "#26272B"
    border: "#34363B"
    ink: "#EDEEF0"
    ink-2: "#A9AEB6"
    ink-3: "#8F959E"
    ink-disabled: "#5D6169"
    accent: "#8B7BFF"
    accent-soft: "rgba(139, 123, 255, 0.16)"
    on-accent: "#14102E"
    primary: "#EDEEF0"
    on-primary: "#111214"
    warn: "#F5B36B"
    warn-bg: "rgba(245, 165, 36, 0.12)"
    warn-border: "rgba(245, 165, 36, 0.32)"
    waiting: "#F5A524"
    ring-track: "#3A3C42"
    favorite: "#F2B84B"
  accent-options:
    violet: "#5B3DF5"
    teal: "#0F766E"
    orange: "#C2410C"
    blue: "#1D4ED8"
typography:
  title:
    fontFamily: Geist
    fontSize: 15px
    fontWeight: 600
    lineHeight: 22px
  message:
    fontFamily: Geist
    fontSize: 15px
    fontWeight: 400
    lineHeight: 1.7
  body:
    fontFamily: Geist
    fontSize: 14px
    fontWeight: 400
    lineHeight: 20px
  body-strong:
    fontFamily: Geist
    fontSize: 14px
    fontWeight: 600
    lineHeight: 20px
  control:
    fontFamily: Geist
    fontSize: 13px
    fontWeight: 500
    lineHeight: 18px
  caption:
    fontFamily: Geist
    fontSize: 12.5px
    fontWeight: 400
    lineHeight: 18px
  label:
    fontFamily: Geist
    fontSize: 12px
    fontWeight: 600
    lineHeight: 16px
  kbd:
    fontFamily: Geist Mono
    fontSize: 11px
    fontWeight: 500
    lineHeight: 18px
  code:
    fontFamily: Geist Mono
    fontSize: 12.5px
    fontWeight: 400
    lineHeight: 20px
spacing:
  "1": 4px
  "1-5": 6px
  "2": 8px
  "2-5": 10px
  "3": 12px
  "4": 16px
  "5": 20px
  "6": 24px
  "8": 32px
  "11": 44px
  control:
    sm: 28px
    md: 32px
    lg: 36px
  layout:
    header-height: 56px
    sidebar-min: 264px
    sidebar-default: 272px
    sidebar-max: 440px
    panel-width: 440px
    chat-max: 760px
    thread-max: 720px
rounded:
  xs: 5px
  sm: 7px
  md: 9px
  lg: 12px
  xl: 14px
  2xl: 16px
  full: 999px
components:
  kbd:
    textColor: "{colors.light.ink-2}"
    backgroundColor: "{colors.light.surface}"
    typography: "{typography.kbd}"
    rounded: "{rounded.xs}"
    padding: 0 6px
  tooltip:
    backgroundColor: "{colors.light.primary}"
    textColor: "{colors.light.on-primary}"
    typography: "{typography.label}"
    rounded: "{rounded.sm}"
    padding: 5px 7px 5px 9px
  button-icon:
    textColor: "{colors.light.ink-2}"
    size: 32px
    rounded: 8px
  button-icon-pressed:
    backgroundColor: "{colors.light.fill-strong}"
    textColor: "{colors.light.ink}"
  button-outline-sm:
    backgroundColor: "{colors.light.surface}"
    textColor: "{colors.light.ink}"
    typography: "{typography.caption}"
    height: 28px
    rounded: 8px
    padding: 0 10px
  button-primary:
    backgroundColor: "{colors.light.primary}"
    textColor: "{colors.light.on-primary}"
    typography: "{typography.control}"
    height: 32px
    rounded: 8px
    padding: 0 16px
  button-send:
    backgroundColor: "{colors.light.primary}"
    textColor: "{colors.light.on-primary}"
    size: 34px
    rounded: "{rounded.full}"
  sidebar-command:
    textColor: "{colors.light.ink-2}"
    height: 38px
    rounded: 10px
    padding: 0 8px 0 10px
  sidebar-task:
    textColor: "{colors.light.ink}"
    typography: "{typography.body}"
    rounded: "{rounded.md}"
    padding: 8px
  sidebar-task-selected:
    backgroundColor: "{colors.light.surface}"
    textColor: "{colors.light.ink}"
    typography: "{typography.body-strong}"
  project-badge:
    backgroundColor: "{colors.light.accent-soft}"
    textColor: "{colors.light.accent}"
    size: 16px
    rounded: 4px
  agent-avatar:
    backgroundColor: "{colors.light.accent}"
    textColor: "{colors.light.on-accent}"
    size: 28px
    rounded: 8px
  user-avatar:
    backgroundColor: "{colors.light.fill-strong}"
    textColor: "{colors.light.ink}"
    size: 28px
    rounded: "{rounded.full}"
  suggestion-card:
    backgroundColor: "{colors.light.surface}"
    textColor: "{colors.light.ink}"
    rounded: "{rounded.lg}"
    padding: 14px
  suggestion-icon:
    backgroundColor: "{colors.light.accent-soft}"
    textColor: "{colors.light.accent}"
    size: 30px
    rounded: 8px
  composer:
    backgroundColor: "{colors.light.surface}"
    textColor: "{colors.light.ink}"
    typography: "{typography.message}"
    rounded: "{rounded.2xl}"
    padding: 14px 12px 10px 14px
    width: 760px
  model-chip:
    backgroundColor: "{colors.light.fill}"
    textColor: "{colors.light.ink}"
    typography: "{typography.control}"
    height: 32px
    rounded: 8px
    padding: 0 10px
  model-chip-open:
    backgroundColor: "{colors.light.fill-strong}"
  popover-session:
    backgroundColor: "{colors.light.surface}"
    rounded: "{rounded.xl}"
    padding: 6px
    width: 288px
  menu:
    backgroundColor: "{colors.light.surface}"
    rounded: 10px
    padding: 4px
    width: 200px
  menu-item:
    textColor: "{colors.light.ink}"
    typography: "{typography.control}"
    height: 32px
    rounded: "{rounded.sm}"
    padding: 0 8px
  model-picker:
    backgroundColor: "{colors.light.surface}"
    textColor: "{colors.light.ink}"
    rounded: "{rounded.xl}"
    width: 720px
    height: 480px
  provider-tab:
    textColor: "{colors.light.ink-2}"
    typography: "{typography.control}"
    height: 30px
    rounded: 8px
    padding: 0 10px
  provider-tab-selected:
    backgroundColor: "{colors.light.primary}"
    textColor: "{colors.light.on-primary}"
  model-row:
    textColor: "{colors.light.ink}"
    typography: "{typography.body}"
    height: 36px
    rounded: 8px
  model-row-selected:
    backgroundColor: "{colors.light.accent-soft}"
    typography: "{typography.body-strong}"
  segmented-control:
    backgroundColor: "{colors.light.fill}"
    rounded: "{rounded.md}"
    padding: 2px
  segmented-thumb:
    backgroundColor: "{colors.light.surface}"
    textColor: "{colors.light.ink}"
    typography: "{typography.caption}"
    height: 28px
    rounded: "{rounded.sm}"
  permission-level:
    backgroundColor: "{colors.light.surface}"
    textColor: "{colors.light.ink}"
    rounded: 10px
    padding: 10px 12px
  permission-level-selected:
    backgroundColor: "{colors.light.accent-soft}"
  warn-pill:
    backgroundColor: "{colors.light.warn-bg}"
    textColor: "{colors.light.warn}"
    rounded: "{rounded.full}"
    padding: 0 7px
  tool-row:
    textColor: "{colors.light.ink}"
    typography: "{typography.body}"
    height: 40px
    rounded: "{rounded.md}"
    padding: 0 8px
  switch:
    backgroundColor: "{colors.light.accent}"
    width: 34px
    height: 20px
    rounded: "{rounded.full}"
  terminal:
    backgroundColor: "{colors.light.bg}"
    textColor: "{colors.light.ink-2}"
    typography: "{typography.code}"
    height: 240px
---

# OneCode

## Overview

OneCode is a desktop workspace for delegating coding tasks to an agent. The
person stays in one window: a task sidebar on the leading edge, a conversation
with a composer in the middle, and optional tools (terminal, workspace panel)
that open beside the chat instead of on top of it.

The look is native, quiet and precise. Surfaces are near-white (or near-black in
dark mode), separated by hairlines rather than shadows. One accent color marks
the agent, progress, and the current choice; everything else is neutral ink at
three strengths. Density is medium: 32 px controls, 36–40 px rows, 14 px body
text, with larger 15 px type reserved for conversation messages so the work
itself reads first.

Screens in the source canvas:

| Artboard | Size | What it shows |
| --- | --- | --- |
| Home · New chat | 1440 × 900 | Sidebar, header, session popover open, first agent reply with three suggestions, composer |
| Home · Model picker open | 1440 × 900 | Same, with the model picker anchored above the composer |
| Home · Workspace panel open | 1440 × 900 | Right panel with the "Open a tool" launcher |
| Home · Dark theme | 1440 × 900 | Workspace panel open, dark palette |
| Model picker | 720 × 480 | Provider tabs, model list with favorites, reasoning effort, permissions entry |
| Model picker · Permissions | 720 × 480 | Four permission levels with a capability matrix |
| Model picker · Dark theme | 720 × 480 | Model picker, dark palette |

## Colors

Two palettes, `light` and `dark`, share one set of role names. Components refer
to roles, never to raw values, so switching theme swaps the palette only.

- **Surfaces.** `bg` is the window and pane background. `surface` is anything
  raised from it: cards, the composer, popovers, the selected sidebar row.
  `fill` and `fill-strong` are quiet in-surface fills (chips, segmented track,
  pressed toggle buttons, avatars).
- **Structure.** `line` draws dividers between regions and inside popovers;
  `border` outlines controls, cards, keycaps and popovers.
- **Ink.** `ink` for primary text and icons that carry state; `ink-2` for
  secondary text and resting icon buttons; `ink-3` for metadata, section labels
  and hints; `ink-disabled` for disabled items and the "/" breadcrumb separator.
- **Accent.** `accent` is the brand and selection color: the agent avatar,
  progress rings, the selected-model check, switch tracks, the update dot.
  `accent-soft` is its tinted background for badges, icon tiles and selected
  rows. `on-accent` is text on a solid accent.
- **Primary.** `primary` is near-black in light and near-white in dark. It is
  the default commit (Send, Done), the selected provider tab, and tooltips.
  `primary` is not the accent.
- **Status.** `waiting` marks a task that needs the person (paused-circle
  icon). `warn`, `warn-bg` and `warn-border` mark risky settings such as Full
  access. `ring-track` is the unfilled part of progress rings and the idle task
  dot. `favorite` colors starred models.

The accent is a user preference with four options (`accent-options`). The
default violet becomes `#8B7BFF` in dark mode; any other option is lightened
30 % toward white for dark mode. `accent-soft` is the accent at 11 % opacity in
light mode and 16 % in dark mode.

macOS window controls (red, yellow, green) are platform chrome and not part of
the palette.

## Typography

Interface text uses **Geist**, falling back to PingFang SC for Chinese and then
the system UI font. Code, keycaps, counts and shortcuts use **Geist Mono**. Base
size is 14 px with 1.5 line height.

| Token | Use |
| --- | --- |
| `title` | Task title in the header, panel and picker headings |
| `message` | Conversation messages and the composer text |
| `body` | Sidebar rows, menu-like rows, model names |
| `body-strong` | Selected sidebar task, card titles, permission level names |
| `control` | Buttons, chips, tabs, menu items |
| `caption` | Metadata, hints, row descriptions, segmented options |
| `label` | Section labels ("Needs you", "Reasoning"), tooltips |
| `kbd` | Keycaps and shortcut hints (⌘K, ⌘1, Esc) |
| `code` | Terminal output |

Weights stay at 400, 500 and 600; 700 appears only in one- or two-letter
monograms (project and provider tiles). No uppercase transforms; section labels
use weight and `ink-3` instead.

## Layout

The window is a three-column desktop shell.

```text
┌───────────┬──────────────────────────────────────┬────────────────┐
│ Sidebar   │ Header (56)          [▤][▭][◫]       │ Workspace      │
│ 264–440   │──────────────────────────────────────│ panel          │
│ (272)     │ Conversation, max 720, centered      │ flex 1 1 440   │
│           │                                      │ (optional)     │
│           │ Composer, max 760, centered          │                │
│           │──────────────────────────────────────│                │
│           │ Terminal, 240 high (optional)        │                │
└───────────┴──────────────────────────────────────┴────────────────┘
```

- **Sidebar.** Resizable from 264 to 440 px (default 272; double-click resets,
  arrow keys step 16 px). Padding 14/10/12/12, 14 px between blocks. It sits on
  `bg` with no fill of its own; the main area draws a `line` on its leading edge.
- **Main.** Takes the remaining width (minimum 640 px before wrapping). Header
  is 56 px with 20 px leading and 14 px trailing padding. The conversation column
  has 44 px top padding and 32 px side padding.
- **Workspace panel.** Opens beside the chat with a `line` on its leading edge,
  basis 440 px. Its launcher content is centered and at most 384 px wide.
- **Terminal.** Opens under the conversation, 240 px high, with a `line` on
  top.

Spacing follows a 4 px base scale (`spacing.1` … `spacing.11`). Inside a
control use 4–8 px; between related controls 8–10 px; between groups 12–16 px;
between sections 24–32 px. Control heights are 28 (small), 32 (default) and
36 px (large); list rows are 36–40 px.

## Elevation & Depth

The window is flat. Regions are separated by `bg` versus `surface` and by
hairlines; shadows only explain stacking.

| Shadow | Light | Dark | Used on |
| --- | --- | --- | --- |
| raised | `0 0 0 1px rgba(15,17,21,.05), 0 1px 3px rgba(15,17,21,.08)` | `0 0 0 1px rgba(255,255,255,.06), 0 1px 3px rgba(0,0,0,.4)` | Selected sidebar task |
| thumb | `0 1px 2px rgba(15,17,21,.12)` | `0 1px 2px rgba(0,0,0,.5)` | Segmented control thumb |
| card | `0 1px 2px rgba(15,17,21,.04)` | none | Suggestion cards |
| composer | `0 8px 28px rgba(15,17,21,.07), 0 1px 2px rgba(15,17,21,.04)` | `0 8px 28px rgba(0,0,0,.35), 0 1px 2px rgba(0,0,0,.3)` | Composer, session popover |
| popover | `0 24px 64px rgba(15,17,21,.16), 0 2px 6px rgba(15,17,21,.06)` | `0 24px 64px rgba(0,0,0,.55), 0 2px 6px rgba(0,0,0,.4)` | Model picker, menus, tooltips |

The strongest shadow belongs to the topmost decision layer (the model picker).
Never stack a popover shadow on a card inside a popover.

## Shapes

Corners come from one scale: `xs` 5 px for keycaps, `sm` 7 px for tooltips,
small buttons and segmented options, `md` 9 px for sidebar rows and segmented
tracks, `lg` 12 px for cards, `xl` 14 px for popovers and the model picker,
`2xl` 16 px for the composer, and `full` for avatars, the send button, switches
and pills. Icon buttons and chips use 8 px; sidebar command buttons, menus and
permission rows use 10 px.

Nested shapes stay concentric: a 28 px thumb with 7 px corners sits in a 2 px
padded track with 9 px corners.

## Voice & Content Strategy

- Sentence case everywhere: "New task", "Open a tool", "Add project script".
- Name the object and the result. Buttons are verbs ("Set up", "Generate",
  "Connect", "Done"); destinations are nouns ("Context", "Files").
- Commands that need more input end with an ellipsis character: "Search or jump
  to…".
- Metadata joins with a middle dot: "notes-app · Cloud · step 3 of 8".
- Show shortcuts as keycaps next to the command: ⌘K, ⌘N, ⌘B, ⌘I, ⌘J, ⌥⌘B, ⌘1–9,
  Esc.
- The agent speaks plainly and briefly: "Hi. I took a look at jarvis: it’s a
  JavaScript project, and Git isn’t set up yet."
- Empty states say what is missing and what to do: "Git isn’t set up yet",
  "No AGENTS.md found", "No models match “gpt”."
- Use typographic quotes and apostrophes (’ “ ”), not straight ones.

## Components

**Sidebar.** From top: window controls row (24 px) with the sidebar toggle;
"Search or jump to…" command (38 px, outlined, `kbd` ⌘K); "New task" command
(38 px, outlined, `ink`, 600, ⌘N); task groups. Each group has a label row
(`label`, `ink-3`, mono count on the trailing edge) and task rows: a 16 px
status icon, a title (500) and a `caption`-size meta line in `ink-3`. Status
icons: a paused circle in `waiting` for "Needs you", an accent arc on a
`ring-track` circle for "Running", a 7 px `ring-track` dot for idle. Project
groups start with a 16 px `accent-soft` monogram badge and end with a "+"
button. The selected task uses `surface`, the raised shadow and 600 weight; do
not add a leading bar. The footer is a row of 32 px icon buttons (Settings,
Reviews, Issues, Automations, Usage, Theme) with "Update available" on the
trailing edge marked by a 7 px accent dot.

**Header.** Breadcrumb: an 18 px project badge, project name in `ink-2`, a "/"
in `ink-disabled`, then the task title as a 28 px button (`body-strong`) with a
chevron. Trailing group of three 32 px icon toggles: session control (⌘I),
terminal (⌘J), right panel (⌥⌘B). A toggle that is on uses `fill-strong` and
`ink`.

**Tooltip.** `primary` background with `on-primary` text, `label` type, the
shortcut in `kbd` at 65 % opacity, popover shadow.

**Session control popover.** 288 px wide, anchored under the header toggles.
Rows are 36 px: machine name with a "Show project folder" icon button; "Open in
VS Code" split into a main action and a 32 px chevron that opens an "Open in"
menu (VS Code, Finder; the current choice has an accent check and `fill`
background); "Add project script"; a `line` divider; "Set up Git".

**Message.** A 28 px avatar (user: `fill-strong` circle with initial; agent:
`accent` rounded square with the OneCode mark), then name in `ink-2` 600 at
12.5 px with model and elapsed time in `ink-3`, then the message in `message`
type. Messages are 30 px apart.

**Suggestion card.** Grid of cards, at least 200 px each, 10 px apart. Card:
`surface`, `border`, 12 px corners, 14 px padding, card shadow. Top row: a
30 px `accent-soft` icon tile and a keycap with the card number (press 1–3 to
pick). Then a `body-strong` title and a `caption`-size description in `ink-2`.

**Composer.** Max 760 px, `surface`, `border`, 16 px corners, composer shadow.
Two-line text area in `message` type with the placeholder "Describe the task.
Type @ to mention files, $ for skills, / for commands". Bottom row: Attach,
Share screen and Voice as 32 px icon buttons; then on the trailing edge the
model chip (model name, effort in `ink-3`, chevron; `fill`, `fill-strong` while
the picker is open) and a 34 px round Send button in `primary`.

**Model picker.** A 720 × 480 popover above the composer. Header row (52 px):
search field "Search models or providers" and an Esc keycap. Provider tabs (All,
Favorites, Google, OpenAI, Anthropic) with mono counts; the selected tab is
`primary`. The model list groups by provider; rows are 36 px with a 20 px
monogram tile, the name, a ⌘n keycap for favorites 1–9, an accent check when
selected, and a star toggle (`favorite` when on, `ink-disabled` when off). The
selected row uses `accent-soft` and 600 weight. A 256 px detail column shows the
model, its provider, a four-option reasoning segmented control (Low, Medium,
High, Extra) with a one-line explanation, and "Use for new tasks in jarvis". The
footer has the Permissions button (warn styling when the level is Full access)
and the hint "↑↓ move · ⏎ choose · ⌘1–9 favorites".

**Permissions view.** Same popover. Header: back button "Models", divider,
"Permissions" title, "Applies to this task" on the trailing edge. Four level
rows (Ask first, Auto-edit, Auto, Full access) in a grid: radio, name and
description, then four 52 px capability columns (Read, Edit, Run, Web). A
capability is a 10 px dot: filled accent = does it on its own; half-filled =
safe commands only; `ink-disabled` ring = asks first. A legend repeats the three
meanings in words. The selected row has an accent border and `accent-soft`
fill; Full access carries a "Use with care" warn pill. Footer: the checkbox
"Always ask before deleting files or pushing to Git" and a primary Done button.

**Workspace panel.** Header (56 px): "+" (Open a tool), the open tool's chip
with a close button, then Expand and Close on the trailing edge. The launcher
lists tools in groups (Build: Terminal, Files, Browser; Review: Changes, Pull
request, Issues; This task: Context, Side chat, Device). Each 40 px row: icon,
name (500), hint in `ink-3`, and a single-letter keycap. Unavailable tools use
`ink-disabled` and keep their hint as the reason ("Set up Git first").

**Context view.** Sections Project, Knowledge and Tools; each row has an
`ink-2` icon, a 500-weight name with an `ink-3` status line, and a trailing
28 px outline button (Set up, Add, Generate, Connect), a switch, or a count. A
footer shows "Context window" usage as a 6 px accent bar on `fill`.

**Terminal.** 240 px region: a tab chip ("jarvis — zsh") on `fill`, New
terminal, Split right, Split down and Close as 28 px icon buttons, and `code`
output with the prompt user in `accent`.

## Do's and Don'ts

- Do keep one accent focal point per surface; everything else is neutral ink.
- Do use `primary` only for the default commit of a decision area (Send, Done)
  and for selected provider tabs; ordinary actions are outline or ghost.
- Do show selection on the item's own surface (`surface` + raised shadow, or
  `accent-soft`); don't add a leading-edge bar.
- Do pair every status color with an icon or words; never rely on color alone.
- Do reserve `warn` for settings with real risk (Full access); don't use it as
  decoration.
- Do keep disabled tools visible with the reason as their hint.
- Do keep shortcuts visible next to commands and in tooltips.
- Don't put shadows on regions of the base window; use `line` and `border`.
- Don't introduce new radii, sizes or colors at call sites; add a token first.
- Don't use links for in-app commands; links are only for external URLs.
