---
title: Session lifecycle, recovery and persistence
section: Features
updated: 2026-10-05
basis: Source-reviewed migration baseline
---
# Session lifecycle, recovery and persistence

Feature ID: `FEAT-session-lifecycle`

## Intent and basis

This record uses the inspected current implementation under
[the accepted migration decision](../decisions/index.md). It describes the
working-tree baseline, not a newly executed product result. See
[baseline identity and limitations](../verification/setup.md#baseline-and-build-identity).

Architectural basis: [ADR 0001](../decisions/adr/0001-a-private-herdr.md), [ADR 0004](../decisions/adr/0004-the-zed-terminal-stack.md), [ADR 0005](../decisions/adr/0005-spaces-follow-zed-multi-workspace.md). Historical portions of ADRs remain dated rationale.

Implementation owners: [crates/chartr/src/app/backend.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/app/backend.rs), [crates/chartr/src/app/persistence.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/app/persistence.rs), [crates/chartr/src/persistence.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/persistence.rs), [crates/chartr-herdr/src/namespace.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr-herdr/src/namespace.rs), [crates/chartr/tests/live_session.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/tests/live_session.rs), [crates/chartr/src/app.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/app.rs).

Legacy story references: 26, 35, 36, 37, 39, 40, 41, 42, 43, 51, 52, 81, 82, 83, 84, 85, 86, 87, 89, 90. See the [complete crosswalk](story-crosswalk.md).

## Entry points and prerequisites

Close an item/group/space, quit/reopen the app, or use the Problems menu and terminal recovery actions. Use only an isolated profile and disposable processes for failure checks.

Use [shared setup](../verification/setup.md) and a disposable profile/project for
stateful recipes. Related areas are linked from the [feature index](index.md).

## Expected outcomes

- Normal app exit detaches sessions by default; the configured exit policy may terminate them. Closing a terminal terminates that session, while closing a plugin destroys its view.
- Session-bound plugin items cascade when their subject closes. Bulk operations confirm before terminating multiple sessions.
- Backend recovery permits one clean restart before a stable crash-loop state; Retry is explicit. Live backend identity overrides stale records while unrelated layout survives.
- Layout writes coalesce, serialize in the background and flush on shutdown; stale queued revisions cannot overwrite newer state. Missing plugins/sessions are summarized.
- The pre-existing dirty Close All Sessions action gathers terminal items across spaces, prompts above one target, keeps spaces open and invokes normal close/cascade behavior. This extension is source-observed only.

## Verification recipe

**Unvalidated recipe — not executed during this migration.** Complete shared
setup first; use actual UI entry points and observe underlying state where relevant.

1. Start two disposable sessions in different fixture spaces and a bound plugin if available. Quit normally and reopen; expect identity/layout continuity.
2. Close one terminal and verify the other survives. Cancel and then confirm a multi-session group/space close; verify only selected descendants terminate.
3. For the dirty Close All Sessions action, use the command palette; cancel first, then confirm in a disposable profile. Expect spaces and unrelated unbound tools to remain.
4. For backend failure scenarios, run the real-sidecar suite from shared setup or exercise only the isolated daemon. Check identity replacement, stale-session rejection and visible recovery states.
5. Relaunch after changing layout and verify persisted state and registry order. Never infer persistence solely from the pre-exit screenshot.

Preserve the selected scenario's logs/screenshots/file observations and exact
build identity before [cleanup](../verification/setup.md#cleanup). Record a fresh
run if this recipe is exercised; source inspection alone is not a pass.

## Coverage and limitations

No daemon was started or killed during migration. Crash loops, exit-policy variants, dirty bulk-close behavior, stale writes and restoration all need execution. Existing tests do not establish the dirty action passes.

The [release matrix](../verification/acceptance.md) retains broader cross-feature,
platform, theme, pointer and accessibility coverage. Choose a bounded subset for
ordinary changes; release requirements remain separate from this advisory workflow.

## Verification history

No current product run was performed for this migration. Earlier
[research and audits](../archive/research/index.md) remain historical evidence for
their recorded snapshots; they do not certify this baseline.
