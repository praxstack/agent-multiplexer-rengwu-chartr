---
title: Tabs, groups and pane layout
section: Features
updated: 2026-10-15
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

Native child surfaces yielding during dragging, corner hit-testing, clone capability and focus/accessibility require real UI checks. Full visual and interaction variants remain in release acceptance.

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
tests background pane event delivery to a Herdr plugin; it does not implement
or verify terminal/group unread dots.

The [attention contract investigation](../verification/runs/2026-10-13-agent-attention-contract.md)
shows that current live handoff changes terminal identity and runs baseline
persistence tests. The proposed terminal/group dots and exact-focus
acknowledgment remain unimplemented and untested.

No current product run was performed for this migration. Earlier
[research and audits](../archive/research/index.md) remain historical evidence for
their recorded snapshots; they do not certify this baseline.
