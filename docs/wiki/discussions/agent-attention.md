---
title: Agent attention and notifications
section: Discussions
updated: 2026-10-19
status: Best-effort simplification implemented; selected Codex checks passed
---
# Agent attention and notifications

The user reports missing desktop notifications and no clear indication of which
terminal sessions need attention. The initial scope covered discussion,
investigation and validation. After the readiness discussion, the user's
October 13 instruction to continue with work-until-grill authorized implementation.
Question 14 resolved the backend distribution choice: carry a tracked Herdr patch
in Chartr. The October 15 simplification below replaces that first implementation.

## Notification context — October 19

The user requested the project/space in the title and a shortened first response
line in the body. In the [submitted form](../journal/2026-10-19-grill-fb098581-9bb9-448d-87d2-391b21a21ae7.md),
they chose supported logs with a generic fallback when no current reply is
available. [DEC-ATTENTION-006](../decisions/index.md#dec-attention-006-space-context-and-supported-log-response-previews)
records that accepted choice. No terminal-text guessing is selected.

The working tree implements this through the existing conversation readers,
background reads and live/persisted space ownership. The [feature record](../features/panes.md#notification-content)
defines truncation, fallback and stale-result handling. Detection and unread
semantics remain those accepted October 15. Fixture-based automated checks are
separate from native banner appearance, which remains unverified for this change.

October 19 automated verification passed `cargo test -p chartr -p chartr-conversations --locked`
(348 passed, six environment-dependent tests ignored). After the final timeout/title
handling change, all 11 attention tests passed again. `cargo build -p chartr --locked`,
formatting, diff checks and wiki build/freshness checks also passed. These results
cover the local working tree, including the preceding unreleased UI changes;
they do not establish native banner appearance or click navigation.

### Local bundle attempt and user deferral

The assistant prepared `target/notification-dev/chartr Local.app` with bundle ID
`dev.chartr.notificationdev`, then supplied a bare executable launch command
without isolated XDG directories. A separate bundle identifier does not isolate
Chartr's profile or Herdr namespace; that launch instruction reused the user's
normal environment and was unsuitable for an isolated test.

The user reports that opening it took over their existing Herdr sessions and
required restarting and resuming sessions one by one. This is a user-reported
session disruption, not a conclusively observed notification result. The exact
takeover mechanism has not been investigated. On October 19 the user explicitly
deferred further notification testing until the next release. Native banners,
preview appearance and click navigation remain unverified; earlier automated
passes are unchanged. No further app launch or session manipulation was performed
in response to that deferral.

## Accepted simplification — October 15

The [latest answers](../journal/2026-10-15-grill-1cbbb041-3fb2-41c0-97c7-434cb1dbeff1.md)
settle the reconsideration below, recorded as
[DEC-ATTENTION-005](../decisions/index.md#dec-attention-005-reuse-existing-detection-with-best-effort-alerts):

- Use existing Herdr detection, accepting false or missed alerts. Attention means
  an agent may need a visit; it does not certify completion or failure.
- Notify once per terminal until it is visited. Another event while still unread
  does not produce another desktop notification.
- Use Codex alone for the first real-agent validation. Other agents can benefit
  from upstream detection; they are not thereby verified.

Exact foreground terminal focus, terminal/group dots, retained unread state during
resumed work, restored dots without catch-up notifications, and operation while
Chartr remains running retain their earlier requirements. The strict exclusion of
all uncertain outcomes and dependable failure classification are superseded.
Explicit terminal signals and combined detection are not part of this iteration.

The engineering proposal is to consume existing Herdr states with one small
Chartr policy: normalize idle/done, detect working-to-inactive and entry into
blocked, keep unread until focus, and suppress repeated notifications while unread.
Initial or reconnected snapshots establish a baseline without replaying alerts.
These transition and transport details remain implementation choices to validate;
the form did not independently accept every detail of the earlier proposal.

Implementation was completed under the October 15 `work-until-grill` instruction.
The strict producer/outcome protocol has been replaced with one app-owned,
namespace-wide activity snapshot about once a second. Chartr normalizes idle/done,
marks working-to-inactive or entry into blocked, and coalesces attention until
exact foreground focus. Startup/reconnect establish a silent baseline. Short
transitions between reads can be missed; this is within the accepted best-effort
scope. Real Codex completion and approval transitions were observed; cancellation
also produced attention, as allowed by that tradeoff.

The only remaining backend patch preserves terminal IDs through live handoff.
Provider adapters, registered producers, qualified occurrences, sequence/gap
recovery, and backend attention persistence are removed. Chartr retains one unread
bit and notification-episode identity per terminal. The unreleased prototype's
version-1 attention storage is discarded; version 2 restores its own unread dots.
The [October 15 recheck](../verification/runs/2026-10-15-lean-attention.md) records
425 passing workspace tests, real Codex checks, native dots/focus, persistence,
live handoff and minimized/closed-window monitoring. macOS accepted requests and
logged presentation; direct banner inspection/click navigation remain unverified,
along with native split/group focus, Hide, graceful Quit and Linux.
Historical October 14 verification remains evidence only for that older build.

## Historical implementation checkpoint — October 14

The tracked patch and Chartr attention service implement the accepted policy.
The [recheck](../verification/runs/2026-10-14-agent-attention.md) records passing
automated checks, real Pi prompt/handoff checks, and native dot/focus/relaunch
checks. Desktop banners/clicks and the full native lifecycle/layout matrix remain
unverified. The [earlier failed attempts](../verification/runs/2026-10-14-agent-attention-attempts.md)
preserve build/test failures and a repaired launch-isolation error.

Real-agent coverage is deliberately narrow: Pi input prompts are exercised;
installed Pi 0.99.2 cannot qualify completion/failure because its settlement lacks
an explicit cancellation field. OpenCode root permission/question events have
fixture coverage with a foreground-process ownership gate. Other agents retain
activity indicators but do not generate qualified attention yet.

The research and proposed implementation below are preserved as dated planning
history. Statements there about unimplemented work describe their original
checkpoint; the October 15 section and its recheck describe the current tree.

## Lean reconsideration — October 14 proposal

After learning that Codex was excluded, the user requested jstack's `rule-lean`,
invited alternative implementations, allowed earlier constraints to be relaxed
for this discussion, and questioned the maintenance of per-harness adapters.
At this checkpoint, reconsideration was authorized but no replacement or relaxed
semantics had been accepted. The October 15 decision above now selects existing
Herdr detection; the alternatives below preserve the discussion's history.

The retrospective finding is that the implementation pursued proof of agent
outcomes before proving a useful notification path for the user's harnesses.
That reliability machinery has a concrete cost: producer registrations and
ordering, provider-specific outcome interpretation, retained backend occurrences,
and handoff transfer. The existing dot/focus and native presentation work remains
useful, but the richer backend contract should not be assumed necessary for a
generic attention indicator.

| Approach | Coverage and maintenance | Meaning and tradeoff |
| --- | --- | --- |
| Standard terminal signals, starting with BEL or OSC 9 | One receiver; cooperating harnesses use built-in settings or emit the protocol. Does not require Chartr to understand each agent's lifecycle. | Unread request for attention. A signal is not proof of successful completion. Silent harnesses remain unsupported without configuration/integration. Ordinary terminal bells can also qualify if agent-only scope is relaxed. |
| Existing Herdr state transitions | Reuses the detector already maintained upstream, including Codex's activity path; no new Chartr-owned adapter collection. | Best-effort attention. Previously observed false-done cases and missed short/repeated transitions remain possible. Generic wording avoids a false success claim but does not remove false alerts. |
| Unseen terminal output | Works for any harness that produces observable output, with no agent-specific setup. | Indicates unread output, not completion or a request. Spinners/logs can keep it active. Quiet-period desktop alerts are heuristics and can misfire during silent work. |
| Strict typed outcomes (current implementation) | Requires dependable provider facts and version-specific integration work. | Stronger semantics, narrower coverage, higher maintenance. Not the default recommendation if broad simple attention is the priority. |

Primary-source precedents inspected October 14:

- [Zed terminal threads](https://zed.dev/docs/ai/terminal-threads#notifications)
  notify on an unfocused terminal bell and focus the terminal when clicked.
- [cmux](https://cmux.com/docs/notifications#sending-notifications) provides a
  common notification command and OSC 777/99 receivers. These protocols do not
  imply every harness automatically emits them.
- The fresh official [Codex manual](https://developers.openai.com/codex/codex-manual.md),
  [notification section](https://learn.chatgpt.com/docs/config-file/config-advanced#notifications),
  documents TUI completion/approval notifications via BEL or OSC 9, with an
  `always` emission option so the terminal can apply its own focus rule. The
  separate external `notify` command currently covers turn completion only.
  These are documented capabilities, not executed Chartr compatibility results.

Recommendation for further evaluation: make the dot mean unread attention, use
standard terminal signals as the primary input, and keep one small focus/unread/
OS-delivery policy. Coalesce repeat requests while unread. A Herdr state-based
fallback is an optional coverage/false-positive tradeoff, not a second bespoke
provider framework. Persistence, distinct failure labeling and replay recovery
can be deferred if the user accepts those scope reductions.

The first experiment should prove a real Codex completion and approval signal
through the exact Herdr attach path into a native banner and the correct tab's
unread dot, including a background terminal. Chartr already observes terminal
bell state, but end-to-end origin-preserving delivery is unverified; one inspected
Herdr bell route forwards to its foreground client. Do not promise that BEL/OSC
passes through every hidden/unmounted session without validating that boundary.
A small forwarding repair may still be necessary.

No mechanism can reliably infer every arbitrary harness's internal state without
its cooperation or heuristics. A shared protocol standardizes transport; it does
not supply missing events. This distinction should be explicit in the product
promise and in any choice to replace the current implementation.

### What reuse of existing Herdr detection would mean

The user requested more detail on this alternative. Source inspection of the
pinned backend shows a shared detector that first identifies the foreground
process, then uses either an active full-lifecycle integration or per-agent TOML
screen rules. Codex is a screen-manifest agent: its bundled rules recognize
working-title spinners, an Action Required title, known approval/question text,
and a working footer. An unrecognized screen can fall back to idle. Rules inspect
the live bottom buffer independently of the user's scrolled viewport. Remote
compatible manifest updates can refresh existing agents without a new binary;
new agent process identities still require backend support.

Chartr already translates the backend's idle/working/blocked/done/unknown statuses
without inference and refreshes session snapshots about every two seconds.
Herdr also has a status subscription. Importantly, `done` is the projection of
`idle` with Herdr's own `seen=false`; it is not a successful-outcome type. Herdr's
seen policy should not silently replace Chartr's exact-terminal focus policy.

A lean proposed consumer could normalize `done` and `idle` to one inactive state,
remember the previous state plus one unread bit per terminal, and mark attention
on working-to-inactive or entry into blocked. A focused terminal would suppress
attention; focusing later would clear unread. Repeated state/presentation updates
would do nothing. Initial/reconnected snapshots would establish a baseline with
no catch-up notifications. Unread could remain while work resumes.

Reusing the existing poll is the smallest implementation, with up to roughly a
poll interval of latency and the possibility of missed transitions. The existing
status stream avoids that polling blind spot during a healthy connection, but
still requires baseline/reconnect handling and deduplication of presentation-only
updates. Neither approach identifies a second request while state remains blocked,
provides reliable failure classification, or guarantees events during disconnection.

This moves agent-rule maintenance to the upstream dependency; it does not remove
that maintenance from the ecosystem or support every arbitrary harness. The
previous cancellation/fallback false-done observations remain relevant. This
alternative becomes reasonable only with an explicit best-effort product promise,
such as “may need attention,” and a validation pass using the user's real agents.
It remains a proposal; no implementation was changed by this discussion.

Sources: [pinned agent documentation](https://github.com/herdrdev/herdr/blob/5a2dee700eeeea68267a4d16777307632f77172f/docs/versions/0.8.2/website/src/content/docs/agents.mdx),
[Codex manifest](https://github.com/herdrdev/herdr/blob/5a2dee700eeeea68267a4d16777307632f77172f/src/detect/manifests/codex.toml),
[status projection](https://github.com/herdrdev/herdr/blob/5a2dee700eeeea68267a4d16777307632f77172f/src/app/api_helpers.rs),
and the earlier [executed contract investigation](../verification/runs/2026-10-13-agent-attention-contract.md).

## Accepted behavior

This table preserves the October 12–13 strict policy implemented on October 14.
The [October 15 simplification](#accepted-simplification-october-15) supersedes its
confirmed-event, uncertainty and failure-classification requirements and coalesces
desktop notifications until the terminal is visited. Other behavior remains.

The [first grill-form answers](../journal/2026-10-12-grill-b4cd88bb-d438-4828-a304-f5b77f4dae14.md)
and concise follow-up below settle these requirements:

| Area | Accepted requirement |
| --- | --- |
| Dot meaning | An accent-color dot means an unseen completion, request for input/approval, or confirmed agent failure. Agent state and unseen attention are separate. |
| Acknowledgment | Clear the dot when that terminal receives focus in an active, foreground window. Merely showing it in another split is insufficient. |
| Desktop delivery | Notify when the originating terminal is not focused, or Chartr is inactive. Working in another Chartr tab still qualifies. Deduplicate unchanged events. |
| Uncertainty | Avoid false alerts; uncertain detection remains unknown. |
| Scope | Agents only. Ordinary build/test command completion is separate work. |
| Placement | Show dots on the terminal tab and its enclosing group, without a space-level dot. Opening the group does not acknowledge all children. |
| App lifetime | Notifications require Chartr to remain running, including hidden/minimized operation. No notification owner after full quit is requested. |
| Resumed work | Keep unseen attention until the terminal is focused, even if the agent starts working again. |
| Relaunch | Restore unread dots for restored sessions without catch-up desktop notifications. |
| Failures | Include confirmed agent failures, labeled as failures. Normal exits, user cancellations and uncertain detection do not qualify. |

The [concise follow-up answers](../journal/2026-10-12-grill-5f286d71-92ef-499d-a8fd-4591f968fade.md)
settle resumed work, relaunch and failure behavior on October 13. They extend the
first round's event scope without authorizing implementation.

Question 1 was not a selection of initial supported agents. The user challenged
the premise of maintaining integrations for each harness and requested research
into the products below before choosing an architecture. Neither universal
automatic detection nor permission to implement was accepted by that answer.

## Comparative research

Research date: 2026-10-12. This is public documentation and source inspection,
not executed verification of these applications. Source links use the inspected
commit where available; documentation pages are live references.

| Product | What the inspected material establishes | Implication for Chartr |
| --- | --- | --- |
| Herdr GPUI | Its [notification policy](https://github.com/penso/herdr-gpui/blob/94a8923aca3ab60e924e21eaf7362951a386169f/crates/herdr-gpui/src/notifications.rs) consumes semantic daemon notices and checks projected agent state before delivery. Its [OS delivery layer](https://github.com/penso/herdr-gpui/blob/94a8923aca3ab60e924e21eaf7362951a386169f/crates/herdr-gpui/src/window/system_notifications.rs) routes clicks and fences stale connections. | A GUI can share Herdr's detection and own presentation. Its tab-based suppression/acknowledgment must not replace our accepted terminal-focus rule. |
| Tern | The public [Agent cx API](https://docs.stencil.so/tern/reference/api-window.html#agent-cx) derives working/idle from OSC 9;4 progress and waiting from paused progress or an ask overlay. Agent blocks and omp shells expose this state. | A common terminal signal is possible. This does not establish automatic lifecycle detection for every arbitrary CLI. The public SDK does not expose the whole app implementation. |
| T3 Code | The [driver registry](https://github.com/pingdotgg/t3code/blob/5753207d53362608f281738eda3a9faf4e13b6be/apps/server/src/orchestration-v2/builtInProviderAdapterDrivers.ts) has native provider drivers plus ACP. The [notification coordinator](https://github.com/pingdotgg/t3code/blob/5753207d53362608f281738eda3a9faf4e13b6be/apps/web/src/components/ThreadNotificationCoordinator.tsx) uses run status, completion timestamps and attention states, skipping initial observations. | A shared UI model is backed by provider integrations and known runs, rather than passive observation of arbitrary terminal output. |
| Soft Machine | The official [release repository](https://github.com/Soft-Machine-io/desktop-releases) states that source is private; it publishes installers and update manifests. | No internal detection mechanism established from the public material inspected. |
| Zed | [Terminal Threads](https://zed.dev/docs/ai/terminal-threads) use BEL from an unfocused terminal for attention. Some agents need configuration or a plugin/extension to emit it. [External Agents](https://zed.dev/docs/ai/external-agents) use ACP in the agent UI. | BEL is a common attention signal, but not a typed completion event. ACP requires a cooperating agent integration and is a different product surface from retaining native terminal TUIs. |
| Cursor | Public [hooks](https://cursor.com/docs/hooks) expose agent lifecycle events through JSON scripts, including prompt submission and stop. | Explicit lifecycle signals exist. These docs do not reveal Cursor's internal notification detector or establish a universal terminal detector. |
| Synara | Its [provider architecture](https://github.com/Emanuele-web04/synara/blob/d4509644c8b7c2ba419680f7da8c2441f230d67e/.docs/provider-architecture.md) normalizes adapters into one runtime-event stream, sharing ACP infrastructure where supported. [Completion logic](https://github.com/Emanuele-web04/synara/blob/d4509644c8b7c2ba419680f7da8c2441f230d67e/apps/web/src/notifications/taskCompletion.logic.ts) excludes interrupted/error settlements and considers background work. | Shared policy can remain independent of provider names while integration details live behind a boundary. “Stopped” is not enough to label success. |
| Kero | Its [lifecycle integration source](https://github.com/egoist/kero/blob/50f7302350ff6e318526cb5bab16c1c13d347f26/kero/KeroAgentIntegrations.swift) explicitly uses native lifecycle events and avoids rendered-text inference; it contains managed integrations for Pi, OpenCode and Grok. [Automation docs](https://kero.sh/docs/automation/) distinguish lifecycle coverage from launching/controlling agents. | Conservative detection limits coverage to dependable signals. Being able to launch a CLI does not establish completion detection. |
| bb | A [common adapter contract](https://github.com/get-bb/bb/blob/4b12265e5fb82c01ea67e67cfe8d935533406937/packages/agent-runtime/src/provider-adapter.ts) controls threads and turns. Separate [Codex](https://github.com/get-bb/bb/blob/4b12265e5fb82c01ea67e67cfe8d935533406937/plugins/provider-codex/src/bridge/bridge.ts) and [ACP](https://github.com/get-bb/bb/blob/4b12265e5fb82c01ea67e67cfe8d935533406937/packages/provider-bridge-acp/src/bridge/bridge.ts) bridges handle native interactions. | Common orchestration still has integration-specific work. This inspection establishes that boundary, not every desktop notification behavior. |
| Orca | Its [notification docs](https://github.com/stablyai/orca/blob/f8daed0d986a5f06d58f1299771c856e9cbe77b4/docs/site/content/docs/notifications.mdx) describe completion alerts and unread markers. Its [managed hook registry](https://github.com/stablyai/orca/blob/f8daed0d986a5f06d58f1299771c856e9cbe77b4/src/main/agent-hooks/managed-agent-hook-registry.ts) registers many distinct agent hook services. | Broad terminal support and common notification UX do not eliminate agent-specific integration maintenance. |
| Conductor | Its [January 5 changelog](https://www.conductor.build/changelog/0.28.7-change-target-branch) documents notifications for questions and plan review. | Useful behavior precedent; internal detection implementation was not established. |
| cmux, additional comparison | The [official overview](https://www.cmux.dev/) describes OSC 9/99/777, CLI and agent hooks. Its [notification documentation](https://cmux.com/docs/notifications) specifies unread state, delivery, and hook examples. | A generic notification transport can serve many programs. Signals must still be emitted, and workspace-level read rules differ from our terminal-level requirement. |

### What can be common

The inspected systems support a common attention interface, but do not establish
a reliable, configuration-free detector for every terminal agent. There are
three distinct mechanisms:

1. Programs emit a shared terminal signal, such as BEL, progress OSC, or a
   notification OSC. The terminal consumes it without understanding the agent.
2. A managed runtime receives explicit lifecycle events through ACP or a native
   provider API and normalizes them into a shared model.
3. A terminal supervisor combines integrations with agent-specific screen/title
   recognition. Herdr already fills this role for Chartr.

These are not interchangeable. A bell requests attention without identifying
whether a turn completed or permission is needed. Progress is meaningful only
when the producer uses it consistently. A quiet output stream, a living process,
or return to the shell does not describe every interactive agent turn.

## Chartr and its pinned backend

Chartr source was inspected at
`402ba90a25d82b527f68f23c1dc5f29e27065036`. Product files were unchanged;
the pre-existing untracked `untitled` was left untouched. Relevant observations:

- [The backend boundary](https://github.com/rengwu/chartr/blob/402ba90a25d82b527f68f23c1dc5f29e27065036/crates/chartr-herdr/src/control.rs)
  already maps Herdr's idle/working/blocked/done/unknown states to sessions.
  Chartr does not own state inference there.
- [Supervision](https://github.com/rengwu/chartr/blob/402ba90a25d82b527f68f23c1dc5f29e27065036/crates/chartr/src/app/backend.rs)
  currently reads session snapshots every two seconds. Comparing only these
  snapshots could miss intermediate transitions; no missed-event reproduction
  was performed.
- [Session items](https://github.com/rengwu/chartr/blob/402ba90a25d82b527f68f23c1dc5f29e27065036/crates/chartr/src/item.rs)
  carry terminal bell state, while [chrome](https://github.com/rengwu/chartr/blob/402ba90a25d82b527f68f23c1dc5f29e27065036/crates/chartr/src/chrome.rs)
  displays agent activity. Neither implements the accepted independent unseen
  attention record. Enclosing groups currently do not aggregate that attention.
- The vendored GPUI platform includes system notification support, but Chartr's
  app code has no delivery wiring for it. Native delivery remains untested here.

Herdr is pinned to `0.8.2-chartr.5a2dee700eee`, revision
`5a2dee700eeeea68267a4d16777307632f77172f`, protocol 22. Its
[agent documentation](https://github.com/herdrdev/herdr/blob/5a2dee700eeeea68267a4d16777307632f77172f/docs/versions/0.8.2/website/src/content/docs/agents.mdx)
describes one state authority per pane: complete lifecycle integrations when
available, otherwise screen manifests, sometimes with title/progress evidence.
A recognized agent with no matching rule can fall back to idle, with the
diagnostic reason `default_known_agent_idle_fallback`. Therefore idle alone is
not proof of a completed turn under the user's conservative requirement.

The pinned [wire protocol](https://github.com/herdrdev/herdr/blob/5a2dee700eeeea68267a4d16777307632f77172f/src/protocol/wire.rs)
already defines `Finished` and `NeedsAttention` semantic notices, and the
[headless server](https://github.com/herdrdev/herdr/blob/5a2dee700eeeea68267a4d16777307632f77172f/src/server/headless/notifications.rs)
forwards agent transitions to client shells. This is a separate persistent
client-shell path, not a subscription already exposed by Chartr's small NDJSON
request client. The inspected notice payload includes pane targeting, but no
event ID, turn ID or detection-evidence field.

The pinned backend also has an existing
[API status subscription](https://github.com/herdrdev/herdr/blob/5a2dee700eeeea68267a4d16777307632f77172f/src/api/subscriptions.rs),
`PaneAgentStatusChanged`. It follows the backend event hub with an internal
sequence and returns pane state/presentation; it can also synthesize events from
snapshots. The emitted subscription envelope inspected here does not carry that
internal sequence or detection evidence, and presentation-only changes can
produce an event. This is another existing transport candidate, closer to
Chartr's current API boundary than the full client shell. Neither path needs to
be invented from scratch, but identity, provenance and recovery semantics remain
to be settled before using either as a dependable attention feed.

Herdr's [state actions](https://github.com/herdrdev/herdr/blob/5a2dee700eeeea68267a4d16777307632f77172f/src/app/actions.rs)
also maintain their own seen state: active-tab status can acknowledge completion,
and non-idle changes reset that marker. That is not the accepted Chartr rule of
acknowledging an unseen completion or input request only by focusing its terminal.
Raw semantic notices and projected `Done` cannot be assumed to satisfy the full
Chartr contract without examining these differences.

## Chosen direction

On 2026-10-13 the user confirmed reuse of Herdr with conservative attention
events and asked what work is needed to deliver the dots and notifications.
The [follow-up answer](../journal/2026-10-12-grill-ff575bf5-cacf-4ab8-868e-46d9ca93c63e.md)
was collected from the round created on October 12. This accepts the shared
backend boundary, not implementation work or every design detail below.

Use Herdr as the shared detector and give Chartr one attention policy, independent
of the provider. Prefer explicit lifecycle or terminal signals when they are
available; resolve missing evidence/coverage in the backend integration boundary.
Do not create a second collection of screen parsers or CLI hooks in Chartr's UI.

The conceptual flow is:

```text
Agent lifecycle / terminal signals / backend detection
                        |
               Herdr event + evidence
                        |
          Chartr attention and focus policy
                  /              \
      terminal + group dot     desktop notification
```

The shared policy should distinguish current activity from an unseen event, use
stable terminal/session identity, and recognize a new event rather than a
repeated snapshot. Viewing an input request should clear its unseen dot while
leaving the current waiting state intact. Desktop delivery and dot visibility
should derive from the same accepted event and focus facts.

BEL/OSC can be an interoperability route, but supporting a protocol does not
make every agent emit it. A generic bell must not be relabeled as confirmed
successful completion, and ordinary shell bells must not silently broaden the
accepted agents-only scope. Whether to enable that additional route is still a
proposal; forwarding through the existing Herdr attach path was not exercised.

The implementation design must address:

- How the existing backend exposes ordered, identifiable attention events and
  their evidence to Chartr, including reconnects and rapid transitions. The
  API status subscription and semantic client-shell stream are existing
  candidates, not proven drop-ins.
- How uncertain screen fallback is excluded without discarding useful explicit
  lifecycle signals. A semantic event name alone does not prove its source.
- Ownership of acknowledgment so Herdr's active-tab behavior cannot consume
  attention for an unfocused split. Preserve unread state across resumed work
  and app restarts, and distinguish confirmed failures from successful completion.

Future verification should cover exact-terminal focus, an unfocused visible
split, another tab, an inactive/minimized app, group aggregation, duplicate
events, reconnects, uncertain detection, and clicking a notification after its
session closes. These are proposed checks, not executed outcomes.

## Proposed implementation

### Implementation checkpoint: backend distribution

On October 13, implementation preparation uncovered a delivery choice omitted
from the investigation. `vendor/herdr/fetch.sh` downloads an unchanged immutable
revision from `herdrdev/herdr`; it has no patch or alternative repository path.
The authenticated `rengwu` account has pull access but no push access to that
repository. A lookup of `rengwu/herdr` returned 404; this establishes only that
no fork at that path is accessible to the authenticated account.

The pinned Herdr [contributor instructions](https://github.com/herdrdev/herdr/blob/5a2dee700eeeea68267a4d16777307632f77172f/CONTRIBUTING.md)
restrict implementation PRs to maintainers and approved contributors; `rengwu`
does not appear in the inspected approved-contributor list. These rules govern
upstream submissions, not local patch development. No external submission or
repository creation has been attempted.

The recommendation awaiting question 13 is a tracked patch against the existing
Herdr pin, applied reproducibly by acquisition with a distinct build identity.
Agent-specific interpretation stays inside the patched backend. This adds
maintenance responsibility when updating Herdr, so it is a proposed change to
the unmodified-upstream acquisition design, not an already accepted detail.
Alternatives are a separately maintained fork or deferring implementation until
upstream support exists. The product behavior and Herdr reuse decision remain
settled. Implementation is paused at this scope choice; no product tests or
native checks were run during this initial preparation.

The [question 13 response](../journal/2026-10-13-grill-6ad221ca-e028-42eb-95eb-6ec117d4f4bd.md)
does not select an option. The user asks what changes are needed and whether a
Herdr plugin could implement them. The subsequent
[plugin feasibility run](../verification/runs/2026-10-13-agent-attention-plugin.md)
links and executes a disposable plugin against the exact backend: hooks work,
but carry the same coarse status and presentation data, without outcome or
occurrence identity. A title-only update delivers another done hook; repeated
blocked reporting has no separate occurrence event. The reporting API rejects
a failure state. These are executed observations, not a claim that plugin code
cannot filter duplicates or maintain its own files.

The concrete backend work remains four areas: preserve integration outcomes and
request IDs; retain typed attention independently of activity/focus; expose a
snapshot/change stream with deduplication and reconnect recovery; and preserve
identity/attention during live handoff. The existing plugin interface does not
extend those internal contracts. A separate plugin executable could obtain raw
provider events through its own agent integrations and maintain its own service,
state and IPC, but that changes the accepted shared-detection approach and adds
maintenance. Basic done/blocked notifications are already feasible as a plugin;
their semantics do not meet the accepted conservative contract.

The user requested a shorter presentation with jstack's concise-writing skill.
In the [re-presented question 14](../journal/2026-10-13-grill-1a7ca491-2a9a-41df-bf41-6294ae40628a.md),
they selected **Keep a Herdr patch in Chartr**, without qualification. This
settles Q-ATTENTION-004 and authorizes local implementation and verification of
that route under the continuing work-until-grill instruction. The earlier long
question 14 was superseded without an answer; its polling was stopped. No
upstream submission or separate fork is requested.

The October 13 [contract investigation](../verification/runs/2026-10-13-agent-attention-contract.md)
exercised the pinned backend and unmodified integration assets. Product files
remain at the revision above. The existing status contract fails the proposed
attention requirements; the five-part design below incorporates those findings.

### Findings that change the earlier proposal

- Excluding unmatched idle fallback is necessary but insufficient. The live
  detector also reported done for a synthetic cancellation screen matched as
  `osc_title_idle`, with `visible_idle: true`.
- Hook authority is not sufficient evidence of an outcome. The pinned OpenCode
  integration collapses input requests, errors and cancellations into blocked;
  Pi discards the settled event's aborted flag. These details are lost before
  Chartr receives state, so an output-only API extension cannot recover them.
- Title changes can emit another done event, and a filtered reconnect emits an
  old done state with the same shape as a live update. Event identity and snapshot
  boundaries must be supplied by the backend.
- Live handoff changes the terminal ID today, even though the pane survives.
  The new attention identity and record must be transferred explicitly.
- The existing Chartr persistence tests pass. They verify the storage foundation,
  not the new unread state or notification behavior.

The design therefore needs changes at both ends of Herdr's boundary: preserve
typed outcomes/request identity when integrations report them, then publish
qualified attention occurrences to Chartr. Agent-specific knowledge stays in
Herdr. The user-visible rules remain unchanged.

### 1. Expose qualified attention through the existing Herdr API

Extend the existing NDJSON surface with a dedicated global attention subscription,
using Herdr's event infrastructure. Prefer that boundary over adding the complete
client-shell protocol to Chartr just for notifications. The existing per-pane
state subscription mixes status and presentation, and cannot serve as the event
contract itself.

The proposed contract supplies a latest attention record per terminal, tagged
with its originating agent instance, in the initial snapshot and subsequent
updates. Resumed work must not erase an unseen event; a confirmed failure must
remain representable after the agent stops.

| Information | Purpose |
| --- | --- |
| Attention epoch, terminal ID, agent-instance identity | Key attention independently of tab placement. Preserve epoch/identities through live handoff; replace them on a cold backend restart or actual agent replacement. Current terminal IDs need explicit handoff transfer. |
| Monotonic per-terminal attention revision and source occurrence key | Recognize a new turn result or input request even when activity stays blocked; duplicate reports, snapshots and title updates keep the same revision. Existing activity/report counters are not substitutes. |
| Kind: turn finished, input needed or confirmed failure | Drive dot semantics and accurate notification wording. Approval is an input-needed case. |
| Evidence and producer/rule version | Preserve the semantic source event or explicitly qualified detector rule. A source label, matched idle rule or full-lifecycle flag alone does not qualify an outcome. Chartr consumes qualified kinds without provider-name branching. |

Extend integration reports with typed attention observations and source occurrence
identity, either alongside lifecycle state or through a dedicated report method.
Normalize final turn completion, input requests and terminal failure in Herdr;
retain cancellation/retry/continuation facts so they cannot become success or
premature failure. For OpenCode, preserve error type and permission/question IDs
and correlate settlement after recovery. For compatible Pi versions, preserve
aborted settlement and outcome/UI-prompt events. Older integrations remain
state-only until their evidence is qualified.

The backend must produce attention independently of its own active-tab/seen
policy. A foreground Herdr tab must not hide an occurrence from an unfocused
Chartr split. Repeated reports of one provider request do not create another
occurrence; a different request can do so without a blocked-to-working edge.

For manifests, explicitly qualify the semantic rule/version; do not equate
`visible_idle` with completed work or all blocked rules with input requests.
Unknown fallback, normal process exit, user cancellation, retrying errors and
intermediate tool failures do not qualify. The tested strong Codex approval rule
is a candidate for input attention, not certification of all prompts or versions.
Completion/failure coverage requires outcome evidence absent from the current
status model. Keep unqualified paths silent rather than weakening the requirement.

A latest record plus change notifications is sufficient for binary unread dots;
a new persistent notification-history service is not required. The new stream
should begin with one authoritative snapshot plus a sequence boundary, then
deliver live changes after that boundary. Register the reader and capture the
snapshot in the same serialized backend operation to avoid a discovery gap.
Include terminal creation/removal in that subscription so new terminals require
no per-pane reconnection.

The existing event buffer holds 512 entries; add explicit gap detection and
resynchronization instead of silently continuing after an overflow. On an app
launch, reconcile the snapshot into dots with no catch-up banners. On a same-app
reconnect, compare revisions with the retained delivered/acknowledged record;
notify at most once for a newly unseen latest occurrence per terminal. This is a
proposed recovery rule, not a promise to replay every intermediate event during
a disconnection. The requested UI is a binary unread marker, not event history.

Carry terminal identity, agent generation, latest attention, attention epoch and
sequence state in Herdr's live-handoff data. A cold restart creates a new epoch
and new terminal lifetimes; stale attention must never attach to replacement
shells that reuse a pane ID. Producer occurrence keys should also survive handoff
so a retried hook delivery cannot manufacture a new event.

This requires additive Herdr input/output API and integration changes, handoff
state changes and a new immutable backend pin, followed through the existing
`vendor/herdr/fetch.sh` workflow. It is shared backend work, not a new collection
of Chartr-specific CLI hooks or screen rules. Exact method names and field
spelling can follow upstream conventions during implementation.

### 2. Connect that feed once, outside terminal rendering

Add typed attention data and a cancellable subscription reader in
`crates/chartr-herdr/src/protocol.rs` and `control.rs` (or a focused sibling module).
Keep socket reading on the background executor. The global snapshot/stream covers
initial discovery and session creation/closure; hidden and unmounted terminals
receive the same coverage as the visible terminal.

Coordinate the reader and deduplication at app lifetime, one owner per private
backend namespace, initialized from `main.rs`. Windows register actual focus and
navigation targets. This avoids duplicate banners from multiple window readers
and keeps monitoring alive while the app is hidden/minimized. It ends on quit.
The existing two-second `app/backend.rs` supervision can retain its health and
recovery role; it should not be the source of fabricated completion transitions.

### 3. Keep unseen attention separate from current activity

Add a small attention model, for example `crates/chartr/src/attention.rs`, keyed by
the stable backend/terminal/agent identity. Track latest processed attention,
latest acknowledged attention, and delivery deduplication. A single policy
function takes an event and current focus facts and returns model changes plus
optional OS delivery. Painting a tab must not emit notifications or acknowledge it.

Persist a versioned `attention` value in the existing SQLite `app_state` table,
owned by the app-lifetime coordinator. Store latest attention, acknowledged
revision and delivery watermark per terminal lifetime. Keep this key separate
from window/layout snapshots so a stale layout save cannot clear acknowledgments.
Use background transactional writes and a final app-quit flush, following the
existing persistence discipline; normal window closure must not end monitoring.

Reconcile identities against the stream's authoritative snapshot before projecting
dots. Keep unread state through working transitions, and acknowledge only the
revision actually observed when focus is processed. A later event stays unread
if that terminal has since lost foreground focus. Remove records for confirmed
closed terminals or a cold backend restart, never merely because the connection
is unavailable. Restoring an active terminal applies the normal focus rule;
other restored terminals keep their dots. These are proposed storage mechanics
for the accepted retention behavior, not implemented or verified persistence.

| Input | Proposed effect |
| --- | --- |
| New qualified live event; exact terminal focused in an active window | Treat as seen; no unread dot or desktop banner for that occurrence. |
| New qualified live event; terminal is not focused or its window is inactive | Mark unread and request one desktop notification. |
| Same revision again, or only title/presentation changes | No additional attention or delivery. |
| Agent resumes work before focus | Preserve its unread dot; current activity may show working. |
| Relaunch restores an unread terminal | Restore its dot; suppress catch-up notifications. Normal focus acknowledgment still applies. |
| Actual terminal focus in an active window | Acknowledge its latest attention and remove the dot; leave current working/blocked state intact. |
| Unknown evidence or stale backend/agent identity | Do not create an alert. |

Observe actual terminal focus in `app.rs` alongside view installation, and window
activation through GPUI's existing observer. Test both together: selected layout
items, background window focus handles and visible adjacent splits are not proof
of user attention. Recheck the focused terminal when a window becomes active.

### 4. Project the unread bit into existing chrome

Expose the attention result to `SessionItem`/`Space` and add `unseen_attention` to
the shared chrome data in `chrome.rs`. Keep a single authoritative attention
model; any item value is a projection rather than a second independent read flag.

Render a small theme-accent dot beside the existing activity/provider marker in
the shared tab/row indicator. Keep its layout stable, and include unseen attention
in the accessible label. Use the same result for outer terminal tabs and pane-local
tabs in `app/panes.rs`. In `Space::entries`, derive a group's dot by checking all
descendant terminal items, rather than its representative item alone. Derive that
aggregate from children instead of storing another independently cleared flag.
The space heading receives no dot.

Opening a group can focus and acknowledge one terminal normally; it does not
clear its other children's unread events. Moving or regrouping a terminal carries
its attention with its identity.

### 5. Wire native delivery and exact-session navigation

Use GPUI's existing `show_system_notification`, `dismiss_system_notification`
and app-wide response callback. The inspected macOS implementation already
handles authorization and foreground banner presentation, so the first pass
does not require a new platform notification library or another background app.

Use the terminal label and event kind for short notification text. Resolve clicks
to the current owning window, space, group and exact terminal; activate/focus it
through the normal navigation path and let the same focus rule acknowledge it.
Revalidate identity at click time. A closed or replaced target must not cause a
new terminal to be created or an unrelated session to be focused. OS permission
denial affects banners, not the in-app unread dot.

### Implementation order and verification

First implement and verify the richer backend report/subscription contract and
qualified producer paths, then build the shared attention model, then wire
dots/focus, and finally native delivery/click routing. The investigation has
established the current contract's gaps and concrete changes; it has not supplied
a tested replacement. No product implementation is requested by this plan.

Use meaningful model tests for duplicate events, initial snapshots, reconnects,
stale identities, resumed work, relaunch restoration and focus/activation ordering.
Verify actual backend signals for completion, input requests and confirmed
failures, including exclusions for cancellation and unmatched-screen fallback;
synthetic UI events alone cannot establish detector reliability. Exercise the
finished app with two terminals in a split/group, another tab, background and
minimized windows, and a stale notification click. On macOS, use an isolated
packaged `.app`: the existing platform notification implementation deliberately
disables delivery outside an app bundle, so `cargo run` cannot establish delivery.

The concise round resolves the remaining user-level choices. This investigation
requires no new preference round: it establishes why the original state feed is
insufficient and how the shared boundary must change. Real provider/version
coverage, post-change handoff continuity, new attention persistence and native
notification behavior remain implementation verification gates. They must not be
reported as validated from the synthetic detector/integration fixtures.

## Investigation record and verification status (historical)

Related features: [agents](../features/agents-inbox.md),
[terminals](../features/terminals.md), [panes](../features/panes.md).
Accepted requirements are indexed in [DEC-ATTENTION-001](../decisions/index.md#dec-attention-001-agent-attention-behavior),
the backend boundary in [DEC-ATTENTION-002](../decisions/index.md#dec-attention-002-reuse-herdr-for-attention-detection),
and retention/failures in [DEC-ATTENTION-003](../decisions/index.md#dec-attention-003-retain-unseen-attention-and-include-failures).
Remaining questions are in the [question register](open-questions.md#agent-attention-design).
No Chartr behavior, agent configuration, hook installation or dependency pin was
changed. The [contract investigation](../verification/runs/2026-10-13-agent-attention-contract.md)
records executed backend/fixture probes and baseline persistence tests, with
explicit failures and limits. No new attention feature or native notification
delivery was verified. Documentation build/check validates these records only.
