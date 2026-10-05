---
title: Plugin surfaces, services and permissions
section: Features
updated: 2026-10-05
basis: Source-reviewed migration baseline
---
# Plugin surfaces, services and permissions

Feature ID: `FEAT-plugin-runtime`

## Intent and basis

This record uses the inspected current implementation under
[the accepted migration decision](../decisions/index.md). It describes the
working-tree baseline, not a newly executed product result. See
[baseline identity and limitations](../verification/setup.md#baseline-and-build-identity).

Architectural basis: [ADR 0003](../decisions/adr/0003-two-plugin-tiers.md), [ADR 0006](../decisions/adr/0006-native-plugin-services-and-wayfinder.md), [ADR 0007](../decisions/adr/0007-prebuilt-native-surfaces.md). Historical portions of ADRs remain dated rationale.

Implementation owners: [crates/chartr/src/app/plugins.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/app/plugins.rs), [crates/chartr/src/native_webview.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/native_webview.rs), [crates/chartr/src/native_plugin.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/native_plugin.rs), [crates/chartr-plugin/src/services.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr-plugin/src/services.rs), [crates/chartr-plugin-host/src/lib.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr-plugin-host/src/lib.rs).

Legacy story references: 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65. See the [complete crosswalk](story-crosswalk.md).

## Entry points and prerequisites

Open enabled tools from New surface; inspect contributed settings/services. Use web permission controls and native/embedded surfaces with declared capabilities.

Use [shared setup](../verification/setup.md) and a disposable profile/project for
stateful recipes. Related areas are linked from the [feature index](index.md).

## Expected outcomes

- Catalog entries are global; each opened item owns stable space/pane context. Per-space singleton reopening focuses its existing instance; declared multi-instance/clone support controls alternatives.
- Restorable items persist; unavailable/unrestorable items are omitted with a summary. Disablement or permission revocation closes live instances and revokes brokers.
- Safe web access is scoped to the owning project and plugin-private data; Free sessions does not implicitly expose HOME. Unsafe access is per plugin.
- Agent, Skills, Prompts and PromptTemplates services are catalog-scoped and reflect live provider availability.
- Embedded plugins use the versioned C ABI, lazy library loading and native child lifecycle; GPUI objects do not cross that boundary. Surfaces close before parents, with libraries retained until process exit.

## Verification recipe

**Unvalidated recipe — not executed during this migration.** Complete shared
setup first; use actual UI entry points and observe underlying state where relevant.

1. Open a per-space tool twice in one fixture space, then in a second. Expect focus of the first instance and distinct ownership in the second space.
2. Resize/split/hide/show a web pane and an available embedded pane. Check focus, title/state and close-before-parent behavior.
3. In safe mode, exercise only declared operations against fixture project files and plugin data. Try an outside path and a symlink escape; expect rejection.
4. Revoke permission or disable the provider while its pane is open. Expect immediate instance/broker closure and dependent unavailability.
5. Relaunch with a missing plugin fixture and inspect restoration summary without losing surviving layout.

Preserve the selected scenario's logs/screenshots/file observations and exact
build identity before [cleanup](../verification/setup.md#cleanup). Record a fresh
run if this recipe is exercised; source inspection alone is not a pass.

## Coverage and limitations

Mock bridge tests cover parts of the contract, not native lifecycle or UI permission paths. Available fixture permissions determine which process/network/session operations can be exercised.

The [release matrix](../verification/acceptance.md) retains broader cross-feature,
platform, theme, pointer and accessibility coverage. Choose a bounded subset for
ordinary changes; release requirements remain separate from this advisory workflow.

## Verification history

No current product run was performed for this migration. Earlier
[research and audits](../archive/research/index.md) remain historical evidence for
their recorded snapshots; they do not certify this baseline.
