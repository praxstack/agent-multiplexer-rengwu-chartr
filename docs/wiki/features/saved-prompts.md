---
title: Saved prompt library
section: Features
updated: 2026-10-05
basis: Source-reviewed migration baseline
---
# Saved prompt library

Feature ID: `FEAT-saved-prompts`

## Intent and basis

This record uses the inspected current implementation under
[the accepted migration decision](../decisions/index.md). It describes the
working-tree baseline, not a newly executed product result. See
[baseline identity and limitations](../verification/setup.md#baseline-and-build-identity).

Architectural basis: [ADR 0006](../decisions/adr/0006-native-plugin-services-and-wayfinder.md). Historical portions of ADRs remain dated rationale.

Implementation owners: [plugins/prompts/src/lib.rs](https://github.com/rengwu/chartr/blob/main/plugins/prompts/src/lib.rs), [plugins/prompts/src/dialog.rs](https://github.com/rengwu/chartr/blob/main/plugins/prompts/src/dialog.rs), [plugins/prompts/src/store.rs](https://github.com/rengwu/chartr/blob/main/plugins/prompts/src/store.rs).

Legacy story references: 95. See the [complete crosswalk](story-crosswalk.md).

## Entry points and prerequisites

Use Saved Prompts in Settings → Plugins. Search, create/edit through the modal, copy the body, or delete with named confirmation.

Use [shared setup](../verification/setup.md) and a disposable profile/project for
stateful recipes. Related areas are linked from the [feature index](index.md).

## Expected outcomes

- Stable IDs survive renames and are never reused after deletion. The shared library is stored atomically in plugin-private prompts.json.
- Copy returns only the full body including whitespace; title is display metadata. Modal Enter inserts a newline, Cmd/Ctrl+Enter saves, Escape/Cancel abandons changes.
- Stale edits/deletions report conflicts; invalid/newer storage is not overwritten. Reload rereads external changes.
- Prompts and PromptTemplates services expose committed bodies and notify consumers. No workspace surface, project-file write or agent launch belongs to this plugin.

## Verification recipe

**Unvalidated recipe — not executed during this migration.** Complete shared
setup first; use actual UI entry points and observe underlying state where relevant.

1. Save a prompt containing Unicode, blank lines and leading spaces. Search by title/body and copy; compare clipboard text with the complete body.
2. Edit, cancel, then save via Cmd/Ctrl+Enter. Check focus trapping, icon tooltips, copy feedback and named deletion cancel/confirm.
3. Open a draft and modify the fixture record externally; expect Save to report a conflict without overwriting the newer value.
4. Change spaces and relaunch. Expect one shared library, stable IDs and reachable table actions at narrow widths.
5. Reference the prompt in Markdown Prompt and rename it; expect stable reference resolution and no automatic project rewrite.

Preserve the selected scenario's logs/screenshots/file observations and exact
build identity before [cleanup](../verification/setup.md#cleanup). Record a fresh
run if this recipe is exercised; source inspection alone is not a pass.

## Coverage and limitations

Clipboard, native modal focus and external-edit conflict recipes have not been executed. The private library path in setup must belong to the isolated profile.

The [release matrix](../verification/acceptance.md) retains broader cross-feature,
platform, theme, pointer and accessibility coverage. Choose a bounded subset for
ordinary changes; release requirements remain separate from this advisory workflow.

## Verification history

No current product run was performed for this migration. Earlier
[research and audits](../archive/research/index.md) remain historical evidence for
their recorded snapshots; they do not certify this baseline.
