---
title: Plugin discovery and installation
section: Features
updated: 2026-10-05
basis: Source-reviewed migration baseline
---
# Plugin discovery and installation

Feature ID: `FEAT-plugin-packages`

## Intent and basis

This record uses the inspected current implementation under
[the accepted migration decision](../decisions/index.md). It describes the
working-tree baseline, not a newly executed product result. See
[baseline identity and limitations](../verification/setup.md#baseline-and-build-identity).

Architectural basis: [ADR 0003](../decisions/adr/0003-two-plugin-tiers.md), [ADR 0007](../decisions/adr/0007-prebuilt-native-surfaces.md). Historical portions of ADRs remain dated rationale.

Implementation owners: [crates/chartr/src/plugin_installer.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/plugin_installer.rs), [crates/chartr/src/plugin_installer/prebuilt.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/plugin_installer/prebuilt.rs), [crates/chartr/src/app/bundled_plugins.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/app/bundled_plugins.rs), [crates/chartr-plugin-host/src/lib.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr-plugin-host/src/lib.rs).

Legacy story references: 45, 54, 55, 64, 65. See the [complete crosswalk](story-crosswalk.md).

## Entry points and prerequisites

Use Settings → Plugins to prepare/install a local folder or Git package, inspect authority, enable/disable, replace, or restart for activation.

Use [shared setup](../verification/setup.md) and a disposable profile/project for
stateful recipes. Related areas are linked from the [feature index](index.md).

## Expected outcomes

- All installation is prebuilt-only; missing platform payloads fail without compiling. External GPUI dylibs are rejected. Built-in native modules are part of the application build.
- Web packages run through the web host; embedded packages load prebuilt C-ABI native child surfaces with user authority. Plugin data lives outside installation payloads and survives replacement.
- Preparation and activation are separated; Later leaves restart-required state and Restart preserves workspace state before relaunch.
- Required provider disablement disables dependent contributions. User installations take precedence over system packages.

## Verification recipe

**Unvalidated recipe — not executed during this migration.** Complete shared
setup first; use actual UI entry points and observe underlying state where relevant.

1. Prepare the local Clock web example in an isolated profile. Inspect permissions, install with Later, then restart; expect its surface to become available.
2. Change its persisted setting and replace the package. Expect plugin data to survive and the managed installation to be independent of the source folder.
3. Supply an external native GPUI library or source-only embedded package. Expect rejection before loading or compilation.
4. Install a real matching prebuilt embedded fixture if available. Confirm authority disclosure, activation and removal. Mark this path untested if no fixture is available.

Preserve the selected scenario's logs/screenshots/file observations and exact
build identity before [cleanup](../verification/setup.md#cleanup). Record a fresh
run if this recipe is exercised; source inspection alone is not a pass.

## Coverage and limitations

Git/network errors, platform package selection, replacement and external embedded fixtures require real checks. The application ships no external plugin engines or assets.

The [release matrix](../verification/acceptance.md) retains broader cross-feature,
platform, theme, pointer and accessibility coverage. Choose a bounded subset for
ordinary changes; release requirements remain separate from this advisory workflow.

## Verification history

No current product run was performed for this migration. Earlier
[research and audits](../archive/research/index.md) remain historical evidence for
their recorded snapshots; they do not certify this baseline.
