---
title: Agent launching and Inbox history
section: Features
updated: 2026-10-05
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

Configure Agent through Settings → Plugins, open its surface, or switch to Chats for searchable Inbox/Archive and the original terminal.

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

## Coverage and limitations

Paid/authenticated providers are not invoked by migration. Live-provider coverage differs by installed adapter; the public Inbox guide retains provider-specific limits. Source-derived discovery is not a guarantee for every provider.

The [release matrix](../verification/acceptance.md) retains broader cross-feature,
platform, theme, pointer and accessibility coverage. Choose a bounded subset for
ordinary changes; release requirements remain separate from this advisory workflow.

## Verification history

No current product run was performed for this migration. Earlier
[research and audits](../archive/research/index.md) remain historical evidence for
their recorded snapshots; they do not certify this baseline.
