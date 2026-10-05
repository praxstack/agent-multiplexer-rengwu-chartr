---
title: Skill source registry and precedence
section: Features
updated: 2026-10-05
basis: Source-reviewed migration baseline
---
# Skill source registry and precedence

Feature ID: `FEAT-skill-sources`

## Intent and basis

This record uses the inspected current implementation under
[the accepted migration decision](../decisions/index.md). It describes the
working-tree baseline, not a newly executed product result. See
[baseline identity and limitations](../verification/setup.md#baseline-and-build-identity).

Architectural basis: [ADR 0006](../decisions/adr/0006-native-plugin-services-and-wayfinder.md). Historical portions of ADRs remain dated rationale.

Implementation owners: [plugins/skills/src/lib.rs](https://github.com/rengwu/chartr/blob/main/plugins/skills/src/lib.rs), [plugins/skills/src/sources.rs](https://github.com/rengwu/chartr/blob/main/plugins/skills/src/sources.rs), [plugins/skills/chartr-plugin.toml](https://github.com/rengwu/chartr/blob/main/plugins/skills/chartr-plugin.toml).

Legacy story references: 95. See the [complete crosswalk](story-crosswalk.md).

## Entry points and prerequisites

Use the Skill sources gear in Settings → Plugins; register local directories or Git sources, rescan, refresh, edit, enable/disable and drag reorder.

Use [shared setup](../verification/setup.md) and a disposable profile/project for
stateful recipes. Related areas are linked from the [feature index](index.md).

## Expected outcomes

- Sources and managed checkouts are plugin-owned. Local folders are read-only; remote registrations keep independent refs/checkouts and explicit refresh.
- Enabled source order determines case-insensitive name precedence. Exact source/skill pins bypass bare-name precedence; missing pins fail visibly.
- Discovery searches the documented bounded depth, skips hidden/dependency directories and stops at found skill roots. Registration/refresh does not execute scripts.
- Reorder/edit/save failures preserve prior registry state; deletion confirms and never removes a local source folder.
- Skills exports combined and per-source PromptTemplates and notifies consumers; it creates no project mirror or generated context file.

## Verification recipe

**Unvalidated recipe — not executed during this migration.** Complete shared
setup first; use actual UI entry points and observe underlying state where relevant.

1. Create two local fixture sources with the same skill name, each with a SKILL.md. Register both; expect first enabled source precedence and shadowing indicators.
2. Reorder then cancel another drag; disable the winner. Expect committed precedence and cancellation to match the registry.
3. Edit a source, rescan, and delete a registration. Expect local source files unchanged and confirmation before deletion.
4. If a disposable Git fixture is available, register two refs and refresh one. Expect independent checkouts and failure/cancel preservation.
5. Open Markdown Prompt and verify combined/per-source templates refresh without writing its output file.

Preserve the selected scenario's logs/screenshots/file observations and exact
build identity before [cleanup](../verification/setup.md#cleanup). Record a fresh
run if this recipe is exercised; source inspection alone is not a pass.

## Coverage and limitations

Git authentication, cancellation/timeouts and write-failure rollback are unexecuted. Restored legacy Skill source tabs should retire without deleting registry data.

The [release matrix](../verification/acceptance.md) retains broader cross-feature,
platform, theme, pointer and accessibility coverage. Choose a bounded subset for
ordinary changes; release requirements remain separate from this advisory workflow.

## Verification history

No current product run was performed for this migration. Earlier
[research and audits](../archive/research/index.md) remain historical evidence for
their recorded snapshots; they do not certify this baseline.
