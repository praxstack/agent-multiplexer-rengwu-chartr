---
title: Spaces and folder ownership
section: Features
updated: 2026-10-22
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

- The fixed “Spaces” header overlays the top of the scroll viewport. The list reserves matching top padding, so the first card is fully visible at rest. Scrolling moves cards behind an always-active fade. At rest the fade ends at the first card; over the first 32 px of scrolling it grows 16 px below the header while its opaque region grows from 30% to 60% of the header height, then fades to transparent. These values preserve the user's manual tuning, confirmed on 2026-10-16. Header clicks cannot reach covered cards; scrolling over the header still moves the list (accepted layout, 2026-10-16).

- While reordering a session in Spaces or an outer tab in Tabs, the carried item has an opaque rounded background within its existing bounds, with no broad wrapper shadow. Its size, outer margins and selection state stay unchanged; the list still reserves its destination slot (accepted drag appearance correction, 2026-10-16).

- Clicking an inactive space heading activates that space and its current item without changing its expanded/collapsed state. Clicking the active heading toggles expansion; the chevron toggles expansion independently for either space. Heading hover does not change its background (accepted interaction update, 2026-10-15).

- Space titles use medium-weight text at 11 px at the default interface scale, with a color 75% of the way from the theme's normal text color toward its muted text color, to distinguish them from session labels (accepted appearance change, 2026-10-22).

- Each space has a very subtle, slightly rounded backdrop around its heading and visible sessions. The active space has slightly stronger contrast. This uses the existing theme palette with no added border. Session rows have explicit outer margins of 4 px on the left and 6 px on the right, outside their backgrounds and borders. Their leading icon has 3 px beyond the normal compact row padding. Space cards have an additional 2 px outer right margin and 2 px below the last visible session (accepted UI changes, 2026-10-15).

- Free sessions is permanent, has no folder identity, and starts empty. Its terminals use the home directory or the configured replacement.
- Folder identity uses canonical paths; aliases do not create duplicate spaces. Renaming changes the label, and removing a space leaves its folder untouched.
- Each live item belongs to one space. Creating from an inactive space activates the intended owner. Missing folders retain unavailable state and can be recovered.
- The complete sidebar order includes Free sessions. Dragging uses variable-height midpoints, edge autoscroll, release position, cancel/rollback, and Reduce Motion behavior.
- Empty spaces show only their heading, with no placeholder session row or tree guide. A 0.5rem gap separates adjacent spaces, including collapsed ones (accepted UI change, 2026-10-07; source-observed, not live-verified).

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

The [October 16 expectation correction](../verification/runs/2026-10-16-fade-expectations.md)
preserves the user-confirmed manual fade tuning. The corrected gradient test and
all 428 exercised workspace tests pass; production rendering is unchanged.

2026-10-16 header/fade follow-up: the
[initial regression](../evidence/2026-10-16-header-fade/before.log) reproduced
the viewport clipping below the header. The first overlapping preview was
[rejected as too transparent](../evidence/2026-10-16-header-fade/weak-preview-user-feedback.png)
during user inspection. The revised treatment keeps the header text on an
opaque ground and extends the fade as scrolling increases.
[19 chrome tests](../evidence/2026-10-16-header-fade/chrome-tests.log) and
[3 scrolling-list tests](../evidence/2026-10-16-header-fade/scroll-tests.log)
passed, including offsets 0/1/8/30/80 px at two UI scales, clipping/paint bounds,
header input, fitting content, scrollbar behavior and existing drag checks.
The [normal build](../evidence/2026-10-16-header-fade/build.log) passed.
A [native scrolled snapshot](../evidence/2026-10-16-header-fade/strong-scrolled.png)
was inspected; precise scroll offsets and input assertions above are automated
GPUI evidence, not measured native gestures. The
[preview](../evidence/2026-10-16-header-fade/instance.json) remains open for user
inspection with its four terminal identities preserved across refresh.
[Build/source identity and limits](../evidence/2026-10-16-header-fade/manifest.json).
No full theme/platform matrix is claimed.

2026-10-16 drag appearance correction: the new GPUI rendering regression
[failed before the fix](../evidence/2026-10-16-tab-drag-surfaces/before.log)
for an unselected sidebar row. After the fix,
[18 relevant tests passed](../evidence/2026-10-16-tab-drag-surfaces/tests.log),
including actual sidebar and outer-tab rendering with selected/unselected
items, opaque rounded bounds, unchanged dimensions and cancellation, plus
existing reorder/commit and pane-drop checks. The
[normal build](../evidence/2026-10-16-tab-drag-surfaces/build.log) passed.
[Source and binary identity](../evidence/2026-10-16-tab-drag-surfaces/manifest.json).
This is automated rendering/interaction coverage; no fresh native screenshot
or full platform/theme matrix is claimed.

2026-10-15 heading interaction follow-up: inactive headings now select their
space without changing expansion; chevrons remain independent collapse controls.
Three sidebar tests passed, including new real-renderer click assertions for an
inactive collapsed heading, its chevron, and drag/drop without accidental
activation. The normal app build passed. This used the GPUI harness; no fresh
native interaction result is claimed.
[Tests](../evidence/2026-10-15-space-heading/sidebar-tests.log),
[build](../evidence/2026-10-15-space-heading/build.log),
[source identity](../evidence/2026-10-15-space-heading/manifest.json).

2026-10-15: Subtle space backdrops implemented. Three existing sidebar tests passed
(collapse/add/sort, scrollbar dragging, theme scrollbar contrast), and the normal
app build passed. Native empty/collapsed card appearance was inspected in
[dark](../evidence/2026-10-15-space-backdrops/dark.png) and
[light](../evidence/2026-10-15-space-backdrops/light.png) themes.
[Instance and source identity](../evidence/2026-10-15-space-backdrops/instance.json),
[test log](../evidence/2026-10-15-space-backdrops/sidebar-tests.log), and
[build log](../evidence/2026-10-15-space-backdrops/build.log) record this bounded check;
no full theme or platform matrix was run. The isolated preview was stopped.

No current product run was performed for this migration. Earlier
[research and audits](../archive/research/index.md) remain historical evidence for
their recorded snapshots; they do not certify this baseline.
