---
title: Views, chrome and navigation
section: Features
updated: 2026-10-05
basis: Source-reviewed migration baseline
---
# Views, chrome and navigation

Feature ID: `FEAT-workspace-views`

## Intent and basis

This record uses the inspected current implementation under
[the accepted migration decision](../decisions/index.md). It describes the
working-tree baseline, not a newly executed product result. See
[baseline identity and limitations](../verification/setup.md#baseline-and-build-identity).

Architectural basis: [ADR 0005](../decisions/adr/0005-spaces-follow-zed-multi-workspace.md). Historical portions of ADRs remain dated rationale.

Implementation owners: [crates/chartr/src/mode.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/mode.rs), [crates/chartr/src/app/window_chrome.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/app/window_chrome.rs), [crates/chartr/src/app/view.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/app/view.rs), [crates/chartr/src/chrome/sidebar_pane.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/chrome/sidebar_pane.rs).

Legacy story references: 27, 29, 30, 34, 73, 80, 89. See the [complete crosswalk](story-crosswalk.md).

## Entry points and prerequisites

Use the Tabs / Spaces / Chats selector, the sidebar, title-bar controls, and command palette. Settings and Hotkeys expose view actions.

Use [shared setup](../verification/setup.md) and a disposable profile/project for
stateful recipes. Related areas are linked from the [feature index](index.md).

## Expected outcomes

- Tabs focuses the active space; Spaces and Chats show all spaces. Changing projection does not recreate or move items. Spaces is the initial view.
- Tabs on macOS places its space picker beside the traffic lights. Spaces and Chats have no title-bar space picker.
- Add/surface controls follow tabs while they fit and remain reachable when tabs overflow. Surface pickers show enabled surfaces and their icons/descriptions.
- Native Settings remains in front when opened, including while the gear tooltip is active. Geometry, sidebar width and mode persist.
- The committed keymap includes Cycle view modes (Cmd+~ on macOS, Ctrl+Alt+~ on Linux); Chats includes space labels under conversation titles at the recorded HEAD. These are source observations, not executed results.

## Verification recipe

**Unvalidated recipe — not executed during this migration.** Complete shared
setup first; use actual UI entry points and observe underlying state where relevant.

1. Keep a running disposable terminal and a split group; switch Tabs → Spaces → Chats → Tabs. Expect the same terminal and restored pane arrangement.
2. Resize to 700×900 and 1100×720, overflow tabs, and open the surface chooser in wide/narrow panes. Expect reachable controls and scrolling without duplicated bars.
3. Open Settings using the gear and platform shortcut while hovering the gear; expect Settings to retain focus. Repeat with a web pane visible.
4. Change mode/sidebar width, relaunch, and inspect the same state. Exercise the current cycle-view binding and its Hotkeys entry.

Preserve the selected scenario's logs/screenshots/file observations and exact
build identity before [cleanup](../verification/setup.md#cleanup). Record a fresh
run if this recipe is exercised; source inspection alone is not a pass.

## Coverage and limitations

Animation, native title-bar placement, tooltips, keyboard access, theme contrast and accessibility trees are unexecuted. Source presence does not establish that any release or platform was tested.

The [release matrix](../verification/acceptance.md) retains broader cross-feature,
platform, theme, pointer and accessibility coverage. Choose a bounded subset for
ordinary changes; release requirements remain separate from this advisory workflow.

## Verification history

No current product run was performed for this migration. Earlier
[research and audits](../archive/research/index.md) remain historical evidence for
their recorded snapshots; they do not certify this baseline.
