---
title: Wayfinder maps, launch and claim recovery
section: Features
updated: 2026-10-05
basis: Source-reviewed migration baseline
---
# Wayfinder maps, launch and claim recovery

Feature ID: `FEAT-wayfinder`

## Intent and basis

This record uses the inspected current implementation under
[the accepted migration decision](../decisions/index.md). It describes the
working-tree baseline, not a newly executed product result. See
[baseline identity and limitations](../verification/setup.md#baseline-and-build-identity).

Architectural basis: [ADR 0006](../decisions/adr/0006-native-plugin-services-and-wayfinder.md). Historical portions of ADRs remain dated rationale.

Implementation owners: [plugins/wayfinder/src/model.rs](https://github.com/rengwu/chartr/blob/main/plugins/wayfinder/src/model.rs), [plugins/wayfinder/src/prompt.rs](https://github.com/rengwu/chartr/blob/main/plugins/wayfinder/src/prompt.rs), [plugins/wayfinder/src/lib.rs](https://github.com/rengwu/chartr/blob/main/plugins/wayfinder/src/lib.rs), [plugins/wayfinder/app.js](https://github.com/rengwu/chartr/blob/main/plugins/wayfinder/app.js), [plugins/wayfinder/starmap.js](https://github.com/rengwu/chartr/blob/main/plugins/wayfinder/starmap.js).

Legacy story references: 97. See the [complete crosswalk](story-crosswalk.md).

## Entry points and prerequisites

Open Wayfinder in a folder space with Agent and Skill sources enabled. Pick a map/ticket, Review & launch, Open session or Release claim….

Use [shared setup](../verification/setup.md) and a disposable profile/project for
stateful recipes. Related areas are linked from the [feature index](index.md).

## Expected outcomes

- Discovery remains .plan/maps/<slug>/map.md with numbered tickets. Closure/frontier derives from Answer, Ruled out, blockers and claims, not a status field.
- Browsing can work with empty registries, but the pane requires enabled providers. Launch also requires configured agent/method prerequisites.
- Review includes source provenance and current ticket context; launch rechecks disk state and writes a real session claim before prompt delivery. One claimed ticket per space is allowed.
- Release confirmation names the claim and refuses stale replacement claims; releasing never terminates the running agent. Failed delivery releases only its own claim.
- Map geometry and camera remain stable on status refresh; responsive details, keyboard navigation and reduced-motion behavior belong to the web UI. Jstack does not replace this product format.

## Verification recipe

**Unvalidated recipe — not executed during this migration.** Complete shared
setup first; use actual UI entry points and observe underlying state where relevant.

1. In a fixture project, create a map and a small ticket dependency chain using the public tracker convention. Include a missing blocker and an empty Answer to inspect warnings/frontier.
2. Open the map and exercise pan/zoom, recenter, keyboard ticket selection and details resizing in wide/narrow panes. Expect refresh not to reset camera or notes.
3. Review a ready ticket with a harmless configured agent. Expect source/method provenance and an on-disk claimed_by before actual prompt delivery.
4. End the disposable session, open Release claim, cancel, then confirm. Expect cancel preservation and valid release restoring readiness.
5. Change the claim externally while confirmation is open; expect refusal preserving the newer claim. Disable a required provider and check pane/catalog unavailability.

Preserve the selected scenario's logs/screenshots/file observations and exact
build identity before [cleanup](../verification/setup.md#cleanup). Record a fresh
run if this recipe is exercised; source inspection alone is not a pass.

## Coverage and limitations

No map launches or claims were executed. A bridge-only release check cannot prove disabled-provider UI availability. Layout tests and plugin bridge tests are listed in shared setup.

The [release matrix](../verification/acceptance.md) retains broader cross-feature,
platform, theme, pointer and accessibility coverage. Choose a bounded subset for
ordinary changes; release requirements remain separate from this advisory workflow.

## Verification history

No current product run was performed for this migration. Earlier
[research and audits](../archive/research/index.md) remain historical evidence for
their recorded snapshots; they do not certify this baseline.
