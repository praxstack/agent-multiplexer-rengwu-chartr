---
title: Agent launching and Inbox history
section: Features
updated: 2026-10-15
basis: Source-reviewed migration baseline
---
# Agent launching and Inbox history

Feature ID: `FEAT-agents-inbox`

## Intent and basis

This record uses the inspected current implementation under
[the accepted migration decision](../decisions/index.md). It describes the
working-tree baseline, not a newly executed product result. See
[baseline identity and limitations](../verification/setup.md#baseline-and-build-identity).

Architectural basis: [ADR 0006](../decisions/adr/0006-native-plugin-services-and-wayfinder.md). Historical portions of ADRs remain dated rationale.

Implementation owners: [plugins/agent/src/lib.rs](https://github.com/rengwu/chartr/blob/main/plugins/agent/src/lib.rs), [crates/chartr/src/conversations.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/conversations.rs), [crates/chartr/src/conversations/view.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/conversations/view.rs), [crates/chartr/src/app/conversations.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/app/conversations.rs), [crates/chartr-conversations/src/lib.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr-conversations/src/lib.rs).

Legacy story references: 94. See the [complete crosswalk](story-crosswalk.md).

## Entry points and prerequisites

Configure Agent through Settings → Plugins and open its surface.

Chats was disabled by the accepted 2026-10-08 UI change. Saved Chats preferences
reopen in Spaces; existing history and selected-history state remain stored.
The Inbox-specific outcomes and recipe steps below describe the retained
implementation and are inactive while the view is disabled. Agent launching
remains available from its surface in Tabs and Spaces.

Use [shared setup](../verification/setup.md) and a disposable profile/project for
stateful recipes. Related areas are linked from the [feature index](index.md).

## Expected outcomes

- The shared registry stores adapter, arguments, environment and prompt delivery. An empty registry disables launch and directs users to Settings.
- Launch creates a chartr-owned terminal in the selected owning space; provider glyphs follow recognized identity. Queue acceptance alone does not establish agent readiness.
- Inbox/Archive tracks session identity and history; selecting a live entry mounts the original terminal. Rich-chat rendering, composer transport and conversation renaming are removed.
- Archive state and history persist; provider logs are read-only adapters. Ended or stale bindings show unavailable/recovery state rather than targeting an unrelated terminal.
- Current HEAD shows the space label beneath chat titles; all-spaces scope remains shared with workspace ownership.

## Verification recipe

**Unvalidated recipe — not executed during this migration.** Complete shared
setup first; use actual UI entry points and observe underlying state where relevant.

1. In the isolated profile, open Agent with an empty registry and follow its setup link. Expect the existing Settings window and disabled launch until configured.
2. Register an installed test adapter with harmless arguments and known delivery mode. Review launch content before sending a disposable prompt; capture actual startup, not only launch acceptance.
3. Switch to Chats, search/select the resulting entry, archive and restore it. Expect the same running terminal across views and persistent archive state after relaunch.
4. End a disposable session and select its history. Expect no accidental attachment to another session. Confirm no rename action or rich-chat composer is offered.

Preserve the selected scenario's logs/screenshots/file observations and exact
build identity before [cleanup](../verification/setup.md#cleanup). Record a fresh
run if this recipe is exercised; source inspection alone is not a pass.

## Optional provider adapter checks

These existing opt-in tests require installed tools and real sessions. They do
not start an agent or editor, but read local provider data. They were not run for
the migration:

```sh
cargo test -p chartr-conversations installed_opencode_exports_a_real_session -- --ignored
cargo test -p chartr-conversations installed_grok_log_matches_its_directory_identity -- --ignored
```

Existing tests cover identity/promotion, ownership/archive persistence, legacy
compatibility, launch arguments and stale bindings. Their presence is not an
execution result; the removed rich-chat transport fixtures are historical.

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

Paid/authenticated providers are not invoked by migration. Live-provider coverage differs by installed adapter; the public Inbox guide retains provider-specific limits. Source-derived discovery is not a guarantee for every provider.

The [release matrix](../verification/acceptance.md) retains broader cross-feature,
platform, theme, pointer and accessibility coverage. Choose a bounded subset for
ordinary changes; release requirements remain separate from this advisory workflow.

## Verification history

The [October 15 lean recheck](../verification/runs/2026-10-15-lean-attention.md)
records the current implementation and its exact coverage. Its
[setup attempts](../verification/runs/2026-10-15-lean-attention-attempts.md) preserve
the failed launch and observation limitations.

The [October 14 implementation recheck](../verification/runs/2026-10-14-agent-attention.md)
links exact build identities, automated checks and selected native observations.
The [failed attempts](../verification/runs/2026-10-14-agent-attention-attempts.md)
remain separate historical evidence.


The [plugin feasibility run](../verification/runs/2026-10-13-agent-attention-plugin.md)
executes a disposable Herdr hook and finds that its status payload cannot meet
the accepted outcome/occurrence contract. A plugin with separate provider
integrations is a different, unaccepted architecture.

The [attention contract investigation](../verification/runs/2026-10-13-agent-attention-contract.md)
executes isolated backend and integration-fixture checks. It establishes gaps in
the candidate attention contract; it does not certify real-agent notification
coverage or implement the feature.

No current product run was performed for this migration. Earlier
[research and audits](../archive/research/index.md) remain historical evidence for
their recorded snapshots; they do not certify this baseline.
