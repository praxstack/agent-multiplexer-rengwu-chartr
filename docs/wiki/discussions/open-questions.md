---
title: Open questions
updated: 2026-10-11
status: Migration resolved; performance verification open
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

## Resolved questions

| ID | Question | Resolution on 2026-10-05 |
| --- | --- | --- |
| Q-JSTACK-008 | Should historical screenshots render inline in the reader? | Yes: simple local Markdown image support in shared jstack. DEC-JSTACK-007. |
| Q-JSTACK-009 | Should references outside the wiki use repository URLs or retain local checkout links? | GitHub URLs; no reader extension for repository links. DEC-JSTACK-008. |
| Q-JSTACK-010 | Does migration completion require fresh chartr execution? | No: documentation and changed-reader validation only. Q10 resolves the earlier selected option and qualification. DEC-JSTACK-009. |

Sources: [first interview](../journal/2026-10-05-grill-ab31b76a-3123-4152-9f0b-1c822da69430.md),
[second interview](../journal/2026-10-05-grill-fa77900d-80f2-4c30-95c5-cf40e9aa1ee9.md),
and [accepted decisions](../decisions/index.md).
