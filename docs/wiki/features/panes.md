---
title: Tabs, groups and pane layout
section: Features
updated: 2026-10-05
basis: Source-reviewed migration baseline
---
# Tabs, groups and pane layout

Feature ID: `FEAT-panes`

## Intent and basis

This record uses the inspected current implementation under
[the accepted migration decision](../decisions/index.md). It describes the
working-tree baseline, not a newly executed product result. See
[baseline identity and limitations](../verification/setup.md#baseline-and-build-identity).

Architectural basis: [ADR 0005](../decisions/adr/0005-spaces-follow-zed-multi-workspace.md). Historical portions of ADRs remain dated rationale.

Implementation owners: [crates/chartr/src/workspace.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/workspace.rs), [crates/chartr/src/app/panes.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/app/panes.rs), [crates/chartr/src/app/pane_drop_preview.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/app/pane_drop_preview.rs), [crates/chartr/src/item.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/item.rs).

Legacy story references: 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 28, 31, 32, 35, 36, 79. See the [complete crosswalk](story-crosswalk.md).

## Entry points and prerequisites

Create terminals or surfaces with the outer or pane-local add controls. Use pane menus, tab dragging, directional focus actions, and the command palette for split/join/move/zoom.

Use [shared setup](../verification/setup.md) and a disposable profile/project for
stateful recipes. Related areas are linked from the [feature index](index.md).

## Expected outcomes

- An outer entry is either a standalone item or a group containing recursive panes; items have one owning pane. Both chromes expose the same entries.
- Split edges, center drops, tab insertion and trailing-strip append preserve item identity. Escape cancels dragging; empty source panes/groups collapse as appropriate.
- Terminals move without cloning. Only opt-in plugins clone with the platform modifier. Cross-space item movement is absent.
- Joining moves items without killing their sessions. Layout, split ratios, active pane/item and group names persist. Bulk group close is scoped to that group's descendants.

## Verification recipe

**Unvalidated recipe — not executed during this migration.** Complete shared
setup first; use actual UI entry points and observe underlying state where relevant.

1. Create five disposable terminals. Move tabs 4 and 5 into tab 3 and split tab 4 right. Expect three outer entries: two standalone tabs and one three-item group.
2. Resize, focus each direction, join, zoom and unzoom. Expect stable ownership and usable keyboard/pointer paths.
3. Drag a standalone item into each pane-body edge and center; cancel another drag. Expect exactly one copy, correct selection, and empty-source collapse. Repeat with native and web surfaces.
4. Rename the group, then clear its name. Expect the explicit name or current item-count fallback; relaunch to check layout restoration.
5. Cancel a group close and then confirm it. Expect siblings outside the group to survive.

Preserve the selected scenario's logs/screenshots/file observations and exact
build identity before [cleanup](../verification/setup.md#cleanup). Record a fresh
run if this recipe is exercised; source inspection alone is not a pass.

## Coverage and limitations

Native child surfaces yielding during dragging, corner hit-testing, clone capability and focus/accessibility require real UI checks. Full visual and interaction variants remain in release acceptance.

The [release matrix](../verification/acceptance.md) retains broader cross-feature,
platform, theme, pointer and accessibility coverage. Choose a bounded subset for
ordinary changes; release requirements remain separate from this advisory workflow.

## Verification history

No current product run was performed for this migration. Earlier
[research and audits](../archive/research/index.md) remain historical evidence for
their recorded snapshots; they do not certify this baseline.
