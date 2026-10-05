---
title: Background plugin status
section: Features
updated: 2026-10-05
basis: Source-reviewed migration baseline
---
# Background plugin status

Feature ID: `FEAT-status-bar`

## Intent and basis

This record uses the inspected current implementation under
[the accepted migration decision](../decisions/index.md). It describes the
working-tree baseline, not a newly executed product result. See
[baseline identity and limitations](../verification/setup.md#baseline-and-build-identity).

Architectural basis: [ADR 0006](../decisions/adr/0006-native-plugin-services-and-wayfinder.md). Historical portions of ADRs remain dated rationale.

Implementation owners: [crates/chartr/src/app/status_bar.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/app/status_bar.rs), [crates/chartr/src/settings.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/settings.rs), [crates/chartr-plugin/src/lib.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr-plugin/src/lib.rs).

## Entry points and prerequisites

Enable Show status bar in General or use Workspace: Toggle status bar. Click a contributed service or right-click the bar to hide it.

Use [shared setup](../verification/setup.md) and a disposable profile/project for
stateful recipes. Related areas are linked from the [feature index](index.md).

## Expected outcomes

- The bar is hidden by default; visibility is global and persisted. Hiding it does not stop background services.
- Plugins with background_status contribute buttons; the host polls once per second and redraws changed values.
- Clicking a service opens its settings. Companion is excluded from the desktop build, so historical mobile status/counts are absent.

## Verification recipe

**Unvalidated recipe — not executed during this migration.** Complete shared
setup first; use actual UI entry points and observe underlying state where relevant.

1. Toggle the bar through Settings and palette, hide through its context menu, and relaunch. Expect consistent persisted visibility.
2. With a real status-contributing fixture available, click its button and observe a status update; expect its settings and changed values without restarting the service.
3. Disable the fixture provider; expect its contribution to disappear. If no provider contributes status, record that scenario as untested rather than inventing a button.

Preserve the selected scenario's logs/screenshots/file observations and exact
build identity before [cleanup](../verification/setup.md#cleanup). Record a fresh
run if this recipe is exercised; source inspection alone is not a pass.

## Coverage and limitations

Service-specific behavior needs a fixture; Companion cannot serve as evidence for the current desktop build.

The [release matrix](../verification/acceptance.md) retains broader cross-feature,
platform, theme, pointer and accessibility coverage. Choose a bounded subset for
ordinary changes; release requirements remain separate from this advisory workflow.

## Verification history

No current product run was performed for this migration. Earlier
[research and audits](../archive/research/index.md) remain historical evidence for
their recorded snapshots; they do not certify this baseline.
