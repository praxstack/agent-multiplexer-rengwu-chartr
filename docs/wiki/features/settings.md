---
title: Settings, themes and keybindings
section: Features
updated: 2026-10-05
basis: Source-reviewed migration baseline
---
# Settings, themes and keybindings

Feature ID: `FEAT-settings`

## Intent and basis

This record uses the inspected current implementation under
[the accepted migration decision](../decisions/index.md). It describes the
working-tree baseline, not a newly executed product result. See
[baseline identity and limitations](../verification/setup.md#baseline-and-build-identity).

Architectural basis: [ADR 0002](../decisions/adr/0002-the-zed-layer.md), [ADR 0005](../decisions/adr/0005-spaces-follow-zed-multi-workspace.md). Historical portions of ADRs remain dated rationale.

Implementation owners: [crates/chartr/src/settings.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/settings.rs), [crates/chartr/src/settings_window.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/settings_window.rs), [crates/chartr/src/settings_window/pages.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/settings_window/pages.rs), [crates/chartr/src/keymap.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/keymap.rs), [crates/chartr/src/components.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/components.rs).

Legacy story references: 5, 42, 44, 53, 66, 67, 68, 69, 70, 71, 72, 74, 75, 76, 77, 78, 80, 92. See the [complete crosswalk](story-crosswalk.md).

## Entry points and prerequisites

Open the single Settings window with the gear, command palette or Cmd/Ctrl+,. Navigate General, Appearance, Terminal, Hotkeys and Plugins.

Use [shared setup](../verification/setup.md) and a disposable profile/project for
stateful recipes. Related areas are linked from the [feature index](index.md).

## Expected outcomes

- Settings are user-global, persisted atomically and applied live where supported. Reopening focuses the existing window; closing Settings does not close an underlying terminal.
- Defaults are chartr Dark, IBM Plex Sans and IBM Plex Mono; theme mode/pairs, fonts, UI/terminal scale and reduced motion are configurable.
- Hotkeys stores sparse overrides and rejects conflicts in the shared context. Unset actions retain platform defaults.
- Plugins contribute settings lazily. Skill sources and Saved Prompts are settings-only; restart-bound changes disclose their requirement.
- Middle-click tab closing and its dependent sidebar option follow their settings rather than acting unconditionally.

## Verification recipe

**Unvalidated recipe — not executed during this migration.** Complete shared
setup first; use actual UI entry points and observe underlying state where relevant.

1. Open Settings twice from different entry points and close with Cmd/Ctrl+W. Expect one window and no terminal termination.
2. Change theme, fonts, scale and Reduce Motion; observe workspace updates and relaunch persistence. Check light/dark contrast and narrow-window controls.
3. Rebind a harmless action, try a conflicting chord, then restore it. Expect persisted accepted edits and visible rejection of the conflict.
4. Exercise the parent middle-click option with/without its sidebar child, then disable it. Expect only enabled targets to close.
5. Visit each plugin settings page and use keyboard focus/page cycling; verify modal focus remains within the active dialog.

Preserve the selected scenario's logs/screenshots/file observations and exact
build identity before [cleanup](../verification/setup.md#cleanup). Record a fresh
run if this recipe is exercised; source inspection alone is not a pass.

## Coverage and limitations

Native focus, accessibility roles, font rendering, theme refresh and malformed-file recovery are not executed. Dirty keymap additions are included in the current source baseline.

The [release matrix](../verification/acceptance.md) retains broader cross-feature,
platform, theme, pointer and accessibility coverage. Choose a bounded subset for
ordinary changes; release requirements remain separate from this advisory workflow.

## Verification history

No current product run was performed for this migration. Earlier
[research and audits](../archive/research/index.md) remain historical evidence for
their recorded snapshots; they do not certify this baseline.
