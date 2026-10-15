---
title: Views, chrome and navigation
section: Features
updated: 2026-10-16
basis: Source-reviewed migration baseline
---
# Views, chrome and navigation

Feature ID: `FEAT-workspace-views`

## Intent and basis

This record uses the inspected current implementation under
[the accepted migration decision](../decisions/index.md). It describes the
working-tree baseline, not a newly executed product result. See
[baseline identity and limitations](../verification/setup.md#baseline-and-build-identity).

Architectural basis: [ADR 0005](../decisions/adr/0005-spaces-follow-zed-multi-workspace.md). Historical portions of ADRs remain dated rationale.

Implementation owners: [crates/chartr/src/mode.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/mode.rs), [crates/chartr/src/app/window_chrome.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/app/window_chrome.rs), [crates/chartr/src/app/view.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/app/view.rs), [crates/chartr/src/chrome/sidebar_pane.rs](https://github.com/rengwu/chartr/blob/main/crates/chartr/src/chrome/sidebar_pane.rs).

Legacy story references: 27, 29, 30, 34, 73, 80, 89. See the [complete crosswalk](story-crosswalk.md).

## Entry points and prerequisites

Use the Tabs / Spaces selector, the sidebar, title-bar controls, and command palette. Settings and Hotkeys expose view actions.

Use [shared setup](../verification/setup.md) and a disposable profile/project for
stateful recipes. Related areas are linked from the [feature index](index.md).

## Expected outcomes

- The fixed “Spaces” header overlays the top of the scroll viewport. The list reserves matching top padding, so the first card is fully visible at rest. Scrolling moves cards behind an always-active fade. At rest the fade ends at the first card; over the first 32 px of scrolling it grows 16 px below the header while its opaque region grows from 30% to 60% of the header height, then fades to transparent. These values preserve the user's manual tuning, confirmed on 2026-10-16. Header clicks cannot reach covered cards; scrolling over the header still moves the list (accepted layout, 2026-10-16).

- While reordering a session in Spaces or an outer tab in Tabs, the carried item has an opaque rounded background within its existing bounds, with no broad wrapper shadow. Its size, outer margins and selection state stay unchanged; the list still reserves its destination slot (accepted drag appearance correction, 2026-10-16).

- Clicking an inactive space heading activates that space and its current item without changing its expanded/collapsed state. Clicking the active heading toggles expansion; the chevron toggles expansion independently for either space. Heading hover does not change its background (accepted interaction update, 2026-10-15).

- Each space has a very subtle, slightly rounded backdrop around its heading and visible sessions. The active space has slightly stronger contrast. This uses the existing theme palette with no added border. Session rows have explicit outer margins of 4 px on the left and 6 px on the right, outside their backgrounds and borders. Their leading icon has 3 px beyond the normal compact row padding. Space cards have an additional 2 px outer right margin and 2 px below the last visible session (accepted UI changes, 2026-10-15).

- Tabs focuses the active space; Spaces shows all spaces. Changing projection does not recreate or move items. Spaces is the initial view.
- Empty spaces show only their heading, with no placeholder row or empty tree beneath it. Collapsing or expanding an empty space leaves its height and neighboring positions unchanged. Populated spaces hide their session rows when collapsed. The accepted 2026-10-08 UI change also increases the shared gap between spaces to 0.5 rem, including drag-sort geometry.
- Tabs on macOS places its space picker beside the traffic lights. Spaces has no title-bar space picker.
- Add/surface controls follow tabs while they fit and remain reachable when tabs overflow. Surface pickers show enabled surfaces and their icons/descriptions.
- Native Settings remains in front when opened, including while the gear tooltip is active. Geometry, sidebar width and mode persist.
- Cycle view modes (Cmd+~ on macOS, Ctrl+Alt+~ on Linux) alternates between Tabs and Spaces.
- Chats is disabled by the accepted 2026-10-08 UI change: selectors, Settings, the command palette and Hotkeys offer only the two enabled views. Saved `inbox` or `conversations` preferences fall back to Spaces, while history and stored legacy shortcut entries remain intact. Legacy Chats shortcuts are inactive.
- Scrolling and view switching must remain responsive on battery with Low Power Mode enabled. Reduced rendering throughput must not monopolize input dispatch; this requirement was confirmed by the user on 2026-10-10.
- Sidebar scroll thumbs are translucent, with stronger hover and drag emphasis. They remain visible while the pointer is in the list or a thumb drag is active, then fade out over 800 ms on exit/release outside. Re-entry reveals them immediately; background content changes do not reveal them. Reduced motion skips the fade. This is the accepted October 15 scrollbar treatment.

## Verification recipe

**Unvalidated recipe — not executed during this migration.** Complete shared
setup first; use actual UI entry points and observe underlying state where relevant.

1. Keep a running disposable terminal and a split group; switch Tabs → Spaces → Tabs. Expect the same terminal and restored pane arrangement.
2. Resize to 700×900 and 1100×720, overflow tabs, and open the surface chooser in wide/narrow panes. Expect reachable controls and scrolling without duplicated bars.
3. Open Settings using the gear and platform shortcut while hovering the gear; expect Settings to retain focus. Repeat with a web pane visible.
4. Change mode/sidebar width, relaunch, and inspect the same state. Exercise the current cycle-view binding and its Hotkeys entry.
5. Restore a disposable profile saved in Chats (`inbox` or legacy `conversations`). Expect Spaces, with existing spaces, pane layout and history retained. Confirm Chats is absent from both selectors, the command palette and Hotkeys, and its old shortcut cannot enter the disabled view.
6. On macOS, repeat sidebar scrolling and view switching on battery with Low Power Mode both enabled and disabled, then on AC and across a live power-source transition. Expect responsive input in each condition; record frame waits and input latency separately from CPU utilization.
7. Keep an empty space beside a populated space. Click an inactive heading to switch spaces without changing its expansion. Use the active heading or either chevron to collapse/expand: the empty space stays header-only without shifting its neighbor, while the populated space hides/restores its rows. Expect visible separation between spaces, including during drag sorting. The add button must not toggle or drag its space.
8. Overflow the sidebar and scroll its first 1–30 px: cards should move behind the stationary Spaces header and fade there, without clipping at the header’s bottom. Scroll with the pointer over the header; its add button should remain clickable without triggering a covered card. Return to the top and expect a fully visible first card. Hover the list and thumb, then leave the container. Expect translucent normal/hover/drag colors and a smooth 800 ms exit fade. Re-enter midway to restore visibility immediately; drag beyond the container and release to start the fade. Keep the pointer outside during content changes and expect no reveal. Repeat with reduced motion (immediate hide) and a fitting list (no thumb).

Preserve the selected scenario's logs/screenshots/file observations and exact
build identity before [cleanup](../verification/setup.md#cleanup). Record a fresh
run if this recipe is exercised; source inspection alone is not a pass.

## Coverage and limitations

Animation, native title-bar placement, tooltips, keyboard access, theme contrast and accessibility trees are unexecuted. Source presence does not establish that any release or platform was tested.

The [release matrix](../verification/acceptance.md) retains broader cross-feature,
platform, theme, pointer and accessibility coverage. Choose a bounded subset for
ordinary changes; release requirements remain separate from this advisory workflow.

## Verification history

The [October 16 expectation correction](../verification/runs/2026-10-16-fade-expectations.md)
preserves the user-confirmed manual fade tuning. The corrected gradient test and
all 428 exercised workspace tests pass; production rendering is unchanged.

The [October 16 release-readiness run](../verification/runs/2026-10-16-release-readiness.md)
at `c11b2dc5` reproduces the header-fade rendering test failure. Current code uses
16 px growth and a 30–60% opaque region; the test and feature expectations
at that revision retained the older 20 px and 75–100% treatment. The user
confirmed the current code was intentional manual tuning on October 16 and
requested updating the test and docs. The failed run records an expectation
mismatch, not a new native clipping observation. The earlier passing
logs below apply only to their recorded tuning and source snapshots.

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

2026-10-15: Translucent sidebar thumbs and the 800 ms exit fade implemented.
`cargo test -p chartr --bin chartr --locked -- --nocapture` passed 311 tests
(one ignored), including rendered alpha, intermediate fade progress, re-entry,
outside dragging/release, reduced motion, fitting content, and composited
contrast across the registered themes. The initial opacity failed light-theme
contrast; higher light-palette opacity passed the recheck. Formatting checks
passed. These are automated GPUI checks of the working tree; the native visual
recipe and full platform matrix were not exercised for this change.

2026-10-15: Subtle space backdrops implemented. Three existing sidebar tests passed
(collapse/add/sort, scrollbar dragging, theme scrollbar contrast), and the normal
app build passed. Native empty/collapsed card appearance was inspected in
[dark](../evidence/2026-10-15-space-backdrops/dark.png) and
[light](../evidence/2026-10-15-space-backdrops/light.png) themes.
[Instance and source identity](../evidence/2026-10-15-space-backdrops/instance.json),
[test log](../evidence/2026-10-15-space-backdrops/sidebar-tests.log), and
[build log](../evidence/2026-10-15-space-backdrops/build.log) record this bounded check;
no full theme or platform matrix was run. The isolated preview was stopped.

2026-10-12: The [sidebar CI fixture repair](../verification/runs/2026-10-12-sidebar-ci.md)
reproduced the stale empty-space assertion from both CI runners, then passed
with explicit empty-space geometry and populated collapse/add/sort coverage.
The local macOS workspace suite passed 419 tests; three explicit real-session
tests and the host-compatible remaining CI checks also passed. A fresh remote
CI run on commit `bcab81d0` passed macOS, Ubuntu and Arch/AUR packaging, including
the Linux-only checks and all steps the original failure had skipped.

2026-10-11: The [first implementation run](../verification/runs/2026-10-11-metal-frame-delivery.md)
eliminated sampled acquisition waits in two active captures, but failed on
reactivation: pausing the link did not allow direct AppKit acquisition. The
transaction-specific invalidation/recreation correction builds; its
[recheck](../verification/runs/2026-10-11-metal-frame-delivery-recheck.md) passed
the selected rendering scenarios on battery with Low Power Mode: zero acquisition
samples in three complete active captures (12,539 main-thread samples), plus
native lifecycle and actual-window checks. Both runs have aggregate result Failed:
the recheck's exercised same-process last-window reopen failed, despite the
passing rendering scenarios. This
bounded pass does not certify input-to-presentation percentiles, power transitions
or the full platform/release matrix. App Hide and same-process last-window reopen
were not established; the latter lacks an app reopen handler. Fresh-process
relaunch restored the disposable session.

2026-10-10 design continuation: The [concrete fix design](../journal/2026-10-10-low-power-fix-design.md)
selects supplied-drawable frame delivery for macOS 14+. A standalone lifecycle
probe passed skip/resume/pause/resize and exactly-once invalidation after exposing
a double-invalidation crash in its first version. This is API evidence; Chartr
has not been patched or validated with the proposed change.

2026-10-10 follow-up: [Standalone Metal timing and the resolution plan](../journal/2026-10-10-low-power-frame-investigation.md)
reproduced drawable backpressure under battery/Low Power Mode without application
content. Request pacing reduced the wait with variation; supplied-drawable
callbacks avoided synchronous acquisition in the diagnostic. No Chartr fix or
end-to-end input-latency validation was executed.

2026-10-10: A user-driven [running-build investigation](../journal/2026-10-10-running-build-lag.md)
reproduced delayed scrolling and view switching in installed build 755 on battery
with Low Power Mode enabled. Main-thread Metal drawable acquisition accounted for
70.59% of samples, compared with 16.24% on AC when the user reported normal
responsiveness. This is an observed failure on that installed build, not a fixed
or passing result for the current checkout. Battery with Low Power Mode disabled
and a corrected build remain untested.

2026-10-08: On the working tree with Chats disabled, 25 targeted tests passed
using `cargo test -p chartr --bin chartr --locked` with the filters `mode::`,
`app::persistence::tests::`, `keymap::tests::`, and `responsive_controls_tests`.
These cover view cycling, both legacy saved mode names with workspace and history
selection retained, inactive shortcut preservation, and the two-option selector's
responsive layout. `cargo check -p chartr --locked` and formatting checks passed.
The responsive check uses GPUI's test harness; no live app session was exercised,
so the manual recipe remains unvalidated.

No current product run was performed for this migration. Earlier
[research and audits](../archive/research/index.md) remain historical evidence for
their recorded snapshots; they do not certify this baseline.
