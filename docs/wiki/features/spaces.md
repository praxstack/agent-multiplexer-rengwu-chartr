---
title: Spaces and folder ownership
section: Features
updated: 2026-10-05
basis: Source-reviewed migration baseline
---
# Spaces and folder ownership

Feature ID: `FEAT-spaces`

## Intent and basis

This record uses the inspected current implementation under
[the accepted migration decision](../decisions/index.md). It describes the
working-tree baseline, not a newly executed product result. See
[baseline identity and limitations](../verification/setup.md#baseline-and-build-identity).

Architectural basis: [ADR 0005](../decisions/adr/0005-spaces-follow-zed-multi-workspace.md). Historical portions of ADRs remain dated rationale.

Implementation owners: [crates/chartr/src/spaces.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/spaces.rs), [crates/chartr/src/space.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/space.rs), [crates/chartr/src/app.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/app.rs).

Legacy story references: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 33, 43, 88, 91, 92. See the [complete crosswalk](story-crosswalk.md).

## Entry points and prerequisites

Open a folder through the space controls or the folder argument at launch. Use Free sessions for folderless work; rename/recover/remove folder spaces through their menus.

Use [shared setup](../verification/setup.md) and a disposable profile/project for
stateful recipes. Related areas are linked from the [feature index](index.md).

## Expected outcomes

- Free sessions is permanent, has no folder identity, and starts empty. Its terminals use the home directory or the configured replacement.
- Folder identity uses canonical paths; aliases do not create duplicate spaces. Renaming changes the label, and removing a space leaves its folder untouched.
- Each live item belongs to one space. Creating from an inactive space activates the intended owner. Missing folders retain unavailable state and can be recovered.
- The complete sidebar order includes Free sessions. Dragging uses variable-height midpoints, edge autoscroll, release position, cancel/rollback, and Reduce Motion behavior.

## Verification recipe

**Unvalidated recipe — not executed during this migration.** Complete shared
setup first; use actual UI entry points and observe underlying state where relevant.

1. In the isolated setup, open two disposable folders and an alias of the first. Expect two folder spaces plus Free sessions, with no duplicate alias space.
2. Create a terminal from the inactive folder card. Expect activation and a standalone terminal owned by that folder; switch away and back to check ownership.
3. Rename, reorder, cancel a reorder, then relaunch. Expect committed labels/order to persist and cancelled movement to leave the prior order.
4. Quit, rename a fixture folder externally, and reopen. Expect an unavailable space with recovery; locate the renamed folder. Remove only this fixture space after checking its folder survives.

Preserve the selected scenario's logs/screenshots/file observations and exact
build identity before [cleanup](../verification/setup.md#cleanup). Record a fresh
run if this recipe is exercised; source inspection alone is not a pass.

## Coverage and limitations

Folder recovery, write-failure rollback, autoscroll and reduced-motion pointer behavior need live execution. Removing a populated space is destructive to its sessions; use disposable sessions. See panes and lifecycle for shared ownership checks.

The [release matrix](../verification/acceptance.md) retains broader cross-feature,
platform, theme, pointer and accessibility coverage. Choose a bounded subset for
ordinary changes; release requirements remain separate from this advisory workflow.

## Verification history

No current product run was performed for this migration. Earlier
[research and audits](../archive/research/index.md) remain historical evidence for
their recorded snapshots; they do not certify this baseline.
