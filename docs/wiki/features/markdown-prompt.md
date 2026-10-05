---
title: Markdown composition and explicit saves
section: Features
updated: 2026-10-05
basis: Source-reviewed migration baseline
---
# Markdown composition and explicit saves

Feature ID: `FEAT-markdown-prompt`

## Intent and basis

This record uses the inspected current implementation under
[the accepted migration decision](../decisions/index.md). It describes the
working-tree baseline, not a newly executed product result. See
[baseline identity and limitations](../verification/setup.md#baseline-and-build-identity).

Architectural basis: [ADR 0006](../decisions/adr/0006-native-plugin-services-and-wayfinder.md). Historical portions of ADRs remain dated rationale.

Implementation owners: [plugins/markdown-prompt/src/lib.rs](https://github.com/rengwu/chartr/blob/main/plugins/markdown-prompt/src/lib.rs), [plugins/markdown-prompt/src/persistence.rs](https://github.com/rengwu/chartr/blob/main/plugins/markdown-prompt/src/persistence.rs), [crates/chartr-storage/src/lib.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr-storage/src/lib.rs).

Legacy story references: 96. See the [complete crosswalk](story-crosswalk.md).

## Entry points and prerequisites

Open Markdown Prompt from New surface in a folder space. Compose text and template chips; use Preview, Reset and Apply changes → Save.

Use [shared setup](../verification/setup.md) and a disposable profile/project for
stateful recipes. Related areas are linked from the [feature index](index.md).

## Expected outcomes

- Text and template bodies concatenate verbatim. Inline chips retain provider/reference identity through move/copy/undo; unavailable referenced providers block writing.
- Preview, refresh, open/close and restart never write project files. Only explicit Save resolves templates and applies the composition.
- Saves validate project-relative Markdown paths and marker structure, preserve surrounding content and existing permissions, and use atomic per-file publication.
- Filename changes clean the prior marked section; empty output does not create a new destination. Multi-file cleanup/publication is not one filesystem transaction.
- Stored composition revisions reject stale panes; malformed/newer config and altered legacy generated output are not silently overwritten.

## Verification recipe

**Unvalidated recipe — not executed during this migration.** Complete shared
setup first; use actual UI entry points and observe underlying state where relevant.

1. In a fixture project, compose plain text plus a Saved Prompt or Skills template. Insert/move/copy/undo/remove chips and Preview; expect expanded text but no destination file.
2. Apply changes, cancel, then Save to an existing .md file containing unrelated text. Expect only the marked section to change and surrounding content to survive.
3. Change a provider and refresh/reopen/restart. Compare the destination bytes; expect no change until another Save.
4. Change filename and then save empty output; check old-section cleanup and absence of unwanted new output.
5. Try duplicate/reversed markers, traversal, a symlink target, disabled provider and stale pane. Expect an error preserving previous valid content.

Preserve the selected scenario's logs/screenshots/file observations and exact
build identity before [cleanup](../verification/setup.md#cleanup). Record a fresh
run if this recipe is exercised; source inspection alone is not a pass.

## Coverage and limitations

All save/error recipes are unvalidated. Use only disposable project files and preserve evidence before removing fixtures; atomicity is per file, not across a rename cleanup pair.

The [release matrix](../verification/acceptance.md) retains broader cross-feature,
platform, theme, pointer and accessibility coverage. Choose a bounded subset for
ordinary changes; release requirements remain separate from this advisory workflow.

## Verification history

No current product run was performed for this migration. Earlier
[research and audits](../archive/research/index.md) remain historical evidence for
their recorded snapshots; they do not certify this baseline.
