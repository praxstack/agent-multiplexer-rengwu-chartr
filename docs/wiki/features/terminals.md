---
title: Terminal interaction and search
section: Features
updated: 2026-10-05
basis: Source-reviewed migration baseline
---
# Terminal interaction and search

Feature ID: `FEAT-terminals`

## Intent and basis

This record uses the inspected current implementation under
[the accepted migration decision](../decisions/index.md). It describes the
working-tree baseline, not a newly executed product result. See
[baseline identity and limitations](../verification/setup.md#baseline-and-build-identity).

Architectural basis: [ADR 0001](../decisions/adr/0001-a-private-herdr.md), [ADR 0002](../decisions/adr/0002-the-zed-layer.md), [ADR 0004](../decisions/adr/0004-the-zed-terminal-stack.md). Historical portions of ADRs remain dated rationale.

Implementation owners: [crates/chartr/src/session.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/session.rs), [crates/chartr/src/session/launch.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/session/launch.rs), [crates/chartr/src/app/terminal_search.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/app/terminal_search.rs), [vendor/zed-terminal-view/chartr-PATCH.md](https://github.com/rengwu/chartr/blob/main/vendor/zed-terminal-view/chartr-PATCH.md).

Legacy story references: 23, 37, 38, 81, 82, 93. See the [complete crosswalk](story-crosswalk.md).

## Entry points and prerequisites

Use + to create a terminal. Search with Cmd+F on macOS or Ctrl+Shift+F on Linux; use terminal menus and platform clipboard shortcuts.

Use [shared setup](../verification/setup.md) and a disposable profile/project for
stateful recipes. Related areas are linked from the [feature index](index.md).

## Expected outcomes

- The pinned Zed model/view owns emulation, selection, clipboard, IME, mouse reporting and scrollback. Its local PTY runs the private Herdr attach client.
- Terminal titles follow backend agent/foreground-process identity and fall back after exit. A terminal is non-cloneable.
- Literal search navigates matches without leaking search keystrokes into the shell. Font changes reflow the terminal grid live.
- Long/multiline agent launch commands use private staging rather than entering the complete command into interactive shell history.
- Attach failure is visible; retry targets the same confirmed session identity rather than silently creating a different one.

## Verification recipe

**Unvalidated recipe — not executed during this migration.** Complete shared
setup first; use actual UI entry points and observe underlying state where relevant.

1. Print several screens of styled Unicode in a disposable shell; select/copy/paste and search literal punctuation. Dismiss search and verify terminal focus.
2. Run less or another installed alternate-screen TUI. Exercise wheel/trackpad, keys, mouse and resize; expect input to reach the TUI without duplicate delivery.
3. Check multiline input, word movement, IME, file drag/paste, hyperlink actions and BEL indication using the acceptance matrix.
4. Change terminal font size, detach/reopen and repeat in standalone and split panes. Capture visible results and session identity, not screenshots alone.

Preserve the selected scenario's logs/screenshots/file observations and exact
build identity before [cleanup](../verification/setup.md#cleanup). Record a fresh
run if this recipe is exercised; source inspection alone is not a pass.

## Coverage and limitations

Clipboard, IME, device scrolling, alternate-screen interaction and platform-specific key delivery remain unvalidated. Test helpers cannot certify the native paths they bypass.

The [release matrix](../verification/acceptance.md) retains broader cross-feature,
platform, theme, pointer and accessibility coverage. Choose a bounded subset for
ordinary changes; release requirements remain separate from this advisory workflow.

## Verification history

No current product run was performed for this migration. Earlier
[research and audits](../archive/research/index.md) remain historical evidence for
their recorded snapshots; they do not certify this baseline.
