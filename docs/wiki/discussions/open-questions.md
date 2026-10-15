---
title: Open questions
updated: 2026-10-15
status: Scope resolved; performance and attention verification gaps remain
---
# Open questions

There are no remaining consequential scope questions from the migration interview.
Reopen this register if implementation uncovers a material ambiguity.

## Performance verification questions

Implementation was authorized on October 11. These remain measurement questions,
not requests for implementation approval. The [current recheck](../verification/runs/2026-10-11-metal-frame-delivery-recheck.md)
records the correction after an activation failure and the completed real-window
verification: the selected rendering scenarios pass, with
zero acquisition samples across three active battery/Low Power Mode captures.
Its aggregate result is Failed because the exercised same-process last-window
reopen failed. Earlier failed results remain linked from it.

| ID | Question | Current evidence and next check |
| --- | --- | --- |
| Q-PERF-001 | Is a small Low Power Mode request-rate limit sufficient to restore input responsiveness? | Resolved for design selection: it remains an uncertain mitigation and is not the selected fix. Supplied drawables are now implemented and actual-app traces confirm removal of acquisition waiting. Native event-to-presentation percentiles remain unmeasured. |
| Q-PERF-002 | Does battery power alone reproduce the stall with Low Power Mode off? | The confirmed comparison changed both power source and policy. Add the missing power condition to controlled verification. |

Sources: [Low Power Mode investigation](../journal/2026-10-10-low-power-frame-investigation.md)
and [concrete fix design](../journal/2026-10-10-low-power-fix-design.md).

## Lifecycle follow-up outside the rendering patch

The October 11 recheck found that opening the app after its last window closes
does not recreate a window while the process remains alive. The unchanged app
entry point does not register `on_reopen`; a fresh process launch works. App Hide
was not established either: Cmd+H had no visible effect and the application menu
was empty. These observations are retained as separate follow-up work, not fixed
or counted as successful lifecycle behavior by the performance change.

The user requested the reopen fix on October 12. The
[implementation run](../verification/runs/2026-10-12-window-reopen.md) records a
registered reopen handler, shared startup/restoration path and current-window
quit handling. The user approved an isolated native AppKit test route after desktop tool
startup failed. Automated checks and six native reopen cycles passed, including
terminal identity/input, saved bounds/layout, existing/minimized-window behavior
and both quit policies. The reopen issue is resolved for this tested build;
App Hide remains a separate unverified follow-up.

## Agent attention design

The [interview and comparative research](agent-attention.md) settles the visible
behavior. On October 13 the user accepted reuse of Herdr and requested a concrete
work breakdown. The [implementation design](agent-attention.md#proposed-implementation)
was a proposal during the discussion. After the readiness exchange, the October
13 work-until-grill instruction authorizes implementation. Preparation exposed
the backend distribution choice Q-ATTENTION-004 before product edits.
The concise follow-up resolves retention and confirmed-failure scope. The
[contract investigation](../verification/runs/2026-10-13-agent-attention-contract.md)
then exercised the pinned backend and integration assets. It establishes concrete
contract gaps and revises the implementation design; no additional product choice
was required. The [October 14 recheck](../verification/runs/2026-10-14-agent-attention.md) records
the implementation and selected passing checks. Remaining gaps concern native
banners/clicks, layout/lifecycle variants, Linux and provider coverage; no new
product preference is needed to record these limits.

| ID | Question | Current evidence and next step |
| --- | --- | --- |
| Q-ATTENTION-001 | Can Chartr use one approach across agents without maintaining its own detector for each CLI? | Resolved for direction on October 13: reuse Herdr with conservative events, DEC-ATTENTION-002. This shares detection maintenance; it does not establish reliable automatic detection for every arbitrary CLI. |
| Q-ATTENTION-002 | Does the current backend expose sufficient event identity and evidence for conservative alerts? | Resolved for investigation: no. Executed probes show presentation/replay ambiguity, fallback and cancellation becoming done, loss of failure/cancellation distinctions, and no failure state. The revised design requires typed integration observations plus a qualified global snapshot/attention stream; adding output fields alone is insufficient. Real producer/version eligibility remains an implementation verification gate. |
| Q-ATTENTION-003 | How should attention survive state changes and reconnects? | Policy resolved in DEC-ATTENTION-003. The investigation proposes a separate versioned `app_state` attention value, atomic snapshot/stream startup, explicit gap recovery, and persistence of terminal identity/attention across live handoff. The original handoff changed terminal ID. The patched backend now preserves identity in the executed real-Pi handoff; native dot restoration and attention persistence tests pass. See the October 14 recheck for exact coverage. |
| Q-ATTENTION-004 | How should Chartr carry the required unreleased Herdr changes? | Resolved in DEC-ATTENTION-004: the user selected a tracked Herdr patch in Chartr in the concise question 14. The [executed plugin check](../verification/runs/2026-10-13-agent-attention-plugin.md) established the limits of current hooks before that choice. Local backend changes/builds are authorized; upstream submissions and a separate fork are not requested. |

## Lean attention reconsideration

Resolved for design on October 15 through
[questions 15–17](../journal/2026-10-15-grill-1cbbb041-3fb2-41c0-97c7-434cb1dbeff1.md):
use existing Herdr detection with best-effort alerts, notify once until the exact
terminal is visited, and validate Codex first. The
[accepted simplification](agent-attention.md#accepted-simplification-october-15)
supersedes the earlier strict-outcome requirements. The
[October 14 comparison](agent-attention.md#lean-reconsideration-october-14-proposal)
remains historical; explicit signals are not selected for this iteration.

The simplification is implemented. The [October 15 recheck](../verification/runs/2026-10-15-lean-attention.md)
records a one-second namespace-wide status poll, preserved foreground-focus and
unread retention, removal of the strict protocol/adapters, and real Codex checks.
Completion and approval requests produced unread attention; cancellation also
produced attention, within the accepted best-effort tradeoff. The only backend
patch preserves terminal identity across handoff.

No further product preference is unresolved. Native notification response routing,
direct inspection of banners, split/group focus, app Hide, graceful Quit and Linux
remain verification gaps. macOS accepted notifications and logged presentation;
the desktop tool could not expose the actual alert for a click. These are factual
coverage limits, not reasons to reopen the accepted design or invent a grill.
The October 14 verification applies only to its superseded implementation.

## Resolved questions

| ID | Question | Resolution on 2026-10-05 |
| --- | --- | --- |
| Q-JSTACK-008 | Should historical screenshots render inline in the reader? | Yes: simple local Markdown image support in shared jstack. DEC-JSTACK-007. |
| Q-JSTACK-009 | Should references outside the wiki use repository URLs or retain local checkout links? | GitHub URLs; no reader extension for repository links. DEC-JSTACK-008. |
| Q-JSTACK-010 | Does migration completion require fresh chartr execution? | No: documentation and changed-reader validation only. Q10 resolves the earlier selected option and qualification. DEC-JSTACK-009. |

Sources: [first interview](../journal/2026-10-05-grill-ab31b76a-3123-4152-9f0b-1c822da69430.md),
[second interview](../journal/2026-10-05-grill-fa77900d-80f2-4c30-95c5-cf40e9aa1ee9.md),
and [accepted decisions](../decisions/index.md).
