---
title: Terminal interaction and search
section: Features
updated: 2026-10-16
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

## Agent attention — October 15 simplification

[DEC-ATTENTION-005](../decisions/index.md#dec-attention-005-reuse-existing-detection-with-best-effort-alerts)
selects best-effort Herdr detection. The working tree now reads existing agent
states about once a second. Working-to-inactive or entry into blocked marks
attention; ordinary shells, unknown states and initial/reconnected snapshots do
not produce alerts. Idle and done are equivalent here. Detection can misclassify
cancellation or unfamiliar screens and miss short or repeated blocked requests.

An accent dot means an agent may need attention. Exact terminal focus in an active
foreground window clears it and rearms desktop delivery. At most one desktop
notification is requested until that visit. Unread survives resumed work, app
relaunch against a surviving backend, and live handoff without catch-up banners.
Terminal/group dots share the same state; showing a split or opening a group
cannot acknowledge its unfocused children. Spaces have no aggregate dot.

The strict October 14 provider/outcome implementation is superseded. Agent
coverage follows existing upstream detection, including Codex. The backend patch
now only preserves terminal identity through handoff. The prototype's unreleased
version-1 attention storage is reset; version 2 restores its own unread state.

### Bounded attention recipe

Use a fresh explicitly isolated packaged app and an installed supported agent.
Start with Codex and observe its actual detected states.
Trigger a harmless input request while another terminal is selected. Expect a dot
and an OS notification; focus the exact terminal and expect the dot to clear.
Repeat with an adjacent split and enclosing group, then with the app inactive,
minimized and without windows. Relaunch with unread attention: the dot should
return without another banner. Close or replace the origin before clicking an
old notice: no unrelated terminal should open.

The recheck below identifies which portions were executed. Unexecuted portions
of this recipe remain unvalidated.

## Coverage and limitations

The [October 14 implementation recheck](../verification/runs/2026-10-14-agent-attention.md)
remains historical evidence for the superseded strict design. The [October 15 recheck](../verification/runs/2026-10-15-lean-attention.md)
records passing automated and real Codex completion/approval/cancellation checks,
plus dots/focus, persistence, handoff and minimized/closed-window monitoring.
Direct banner inspection/clicks, native split/group focus, Hide and graceful Quit
remain unverified. Broader provider and Linux coverage is not established.

Clipboard, IME, device scrolling, alternate-screen interaction and platform-specific key delivery remain unvalidated. Test helpers cannot certify the native paths they bypass.

The [release matrix](../verification/acceptance.md) retains broader cross-feature,
platform, theme, pointer and accessibility coverage. Choose a bounded subset for
ordinary changes; release requirements remain separate from this advisory workflow.

## Verification history

The [October 16 release-readiness run](../verification/runs/2026-10-16-release-readiness.md)
at `c11b2dc5` passes all three real-session tests with the exact patched sidecar,
including direct attach, crash recovery and shell-versus-TUI mouse handling.
The broader native terminal input/accessibility matrix is not rerun.

The [October 15 lean recheck](../verification/runs/2026-10-15-lean-attention.md)
records the current implementation and its exact coverage. Its
[setup attempts](../verification/runs/2026-10-15-lean-attention-attempts.md) preserve
the failed launch and observation limitations.

The [October 14 implementation recheck](../verification/runs/2026-10-14-agent-attention.md)
links exact build identities, automated checks and selected native observations.
The [failed attempts](../verification/runs/2026-10-14-agent-attention-attempts.md)
remain separate historical evidence.


The [plugin feasibility run](../verification/runs/2026-10-13-agent-attention-plugin.md)
confirms that executable Herdr event hooks work, but do not resolve the missing
attention facts. No terminal UI or native notification was exercised.

The [attention contract investigation](../verification/runs/2026-10-13-agent-attention-contract.md)
exercises the pinned backend's detector against controlled PTY fixtures. It
records false completion candidates from cancellation/fallback and the event
contract changes needed; native notification delivery remains untested.

No current product run was performed for this migration. Earlier
[research and audits](../archive/research/index.md) remain historical evidence for
their recorded snapshots; they do not certify this baseline.
