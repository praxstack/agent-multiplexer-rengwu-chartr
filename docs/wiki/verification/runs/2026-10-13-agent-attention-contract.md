---
title: Agent attention contract investigation
section: Verification
updated: 2026-10-13
result: Failed
---
# Agent attention contract investigation

## Scope

Investigate whether the pinned Herdr status/API boundary can support the accepted
[agent attention behavior](../../discussions/agent-attention.md). The user
authorized investigation and validation with `work-until-grill`, not product
implementation. Related features: [agents](../../features/agents-inbox.md),
[terminals](../../features/terminals.md), [panes](../../features/panes.md).

**Failed means the existing candidate contract does not meet the proposed
attention requirements.** It does not claim that an implemented notification
feature regressed. No feature implementation or dependency pin was changed.

## Build and environment

- Chartr: `402ba90a25d82b527f68f23c1dc5f29e27065036`; existing planning-doc changes
  were present. Product diff under `crates/` and `vendor/` was empty.
- Exact tested backend: `target/debug/herdr`,
  `0.8.2-chartr.5a2dee700eee`, protocol 22, source revision
  `5a2dee700eeeea68267a4d16777307632f77172f`.
- Backend SHA-256:
  `d9c83f20c4a7daa88570e32ee7af35c86ad4c9ec5ff3ec212a783565db9ec754`.
- [Manifest](../../evidence/2026-10-13-agent-attention-contract/manifest.json)
  records host/toolchain, execution UTC, source/archive and test-binary hashes,
  tracked dirty-patch fingerprint, and evidence hashes. The task date is October
  13; generated timestamps retain the host's actual UTC values.
- Every backend probe used a fresh short `/tmp/ca-attn-*` namespace, isolated
  XDG directories, `/bin/sh`, cleared inherited Herdr variables, and disabled
  version/manifest updates. No personal agent configuration or credentials were
  changed, no paid provider calls were made, and `untitled` was untouched.

## Results

| Scenario | Expected for the proposed contract | Observed | Result |
| --- | --- | --- | --- |
| NDJSON lifecycle transport | Live changes can reach an independent reader | Explicit fixture working/idle/blocked reports produced status events | Passed |
| Attention identity and evidence | Distinguish new occurrences, evidence and replay from presentation | Events contain status/presentation and pane/workspace identity, but no attention ID, revision or evidence; unchanged pane `revision` remained 0 through the fixture transitions | Failed |
| Title-only change | Must not look like another completion | Changing metadata while done emitted another `pane.agent_status_changed` with `done` | Failed |
| Reconnect | Identify snapshot/replay separately | Unfiltered subscription emitted only its acknowledgment; a done-filtered subscription emitted the existing done state with the live-event shape | Failed |
| Identical blocked report | Do not alert twice for the same state report | Repeating blocked produced no additional status event | Passed |
| Unseen completion after resumed work | Preserve an unread occurrence independently of activity | Status returned to working; the payload has no retained attention record | Failed |
| Confirmed failure transport | Preserve failure separately from input and success | `pane.report_agent` rejected `failed`; the accepted states are idle, working, blocked and unknown | Failed |
| Unrecognized screen | No completion from uncertain fallback | A synthetic Codex approval screen followed by unmatched content produced `done`; live explain reported `default_known_agent_idle_fallback` | Failed |
| Cancellation with matched idle evidence | Cancellation must not count as completion | After acquisition, a working fixture followed by `Conversation interrupted` and a static title produced `done`; explain matched `osc_title_idle` with `visible_idle: true` | Failed |
| Known approval screen | Detect the tested input prompt | The synthetic Codex approval screen matched `live_strong_blocker`, with `visible_blocker: true`, and status became blocked | Passed |
| Existing integration outcome mapping | Preserve input/error/cancellation distinctions | Executing the pinned OpenCode asset mapped permission, question, API error and `MessageAbortedError` to blocked. Executing the Pi asset mapped both `agent_settled.aborted=false` and `true` to idle | Failed |
| Identity across live handoff | Keep the same terminal attention identity when the PTY survives | Same-binary handoff preserved `w1:p1` but changed its terminal ID; the old subscription ended | Failed |
| Existing Chartr persistence foundation | Snapshot round-trip, transaction rollback and final-flush ordering hold | All 17 tests selected by `persistence::` passed, including three plugin persistence tests | Passed |
| New attention persistence, native banners and actual agent runs | End-to-end accepted behavior | Not implemented/exercised in this investigation | Untested |

The screen cases used a controlled Python process with `HERDR_AGENT=codex` and
ANSI screen/title output through the actual PTY and detector. They establish
the backend's response to these fixtures, not reliability across real Codex
versions. The integration harness executed unmodified pinned JS/TS assets with
synthetic provider callbacks and a local capture socket. It establishes mapping
and discarded fields, not live provider emission or timing guarantees.

### Evidence and executed recipes

- [Wire results](../../evidence/2026-10-13-agent-attention-contract/wire-results-v2.json)
  and [driver](../../evidence/2026-10-13-agent-attention-contract/probe-v2.py.txt).
- [Detector results](../../evidence/2026-10-13-agent-attention-contract/screen-results-final.json)
  and [driver](../../evidence/2026-10-13-agent-attention-contract/screen-probe-final.py.txt).
- [Integration mapping](../../evidence/2026-10-13-agent-attention-contract/integration-results-v2.json)
  and [driver](../../evidence/2026-10-13-agent-attention-contract/integration-probe-v2.mjs.txt).
- [Live handoff](../../evidence/2026-10-13-agent-attention-contract/handoff-results.json)
  and [driver](../../evidence/2026-10-13-agent-attention-contract/handoff-probe.py.txt).
- [Persistence test log](../../evidence/2026-10-13-agent-attention-contract/persistence-tests.log),
  command: `cargo test -p chartr --locked persistence:: -- --test-threads=1`.
- [Observation checks](../../evidence/2026-10-13-agent-attention-contract/observation-checks.json):
  12 assertions confirm the captured observations used above. These assertions
  are not passes for the proposed feature.

Drivers were executed from `/tmp/chartr-attention-validation-l4lwufrh`; the text
copies preserve the exact paths/source revision and can be adapted for reruns.
The source archive was downloaded from Herdr's immutable GitHub revision. An
initial Python download hit a local CA configuration error; `curl` downloaded
the same archive with certificate verification enabled.

Earlier probes remain intact:
[wire v1](../../evidence/2026-10-13-agent-attention-contract/wire-results.json),
[screen v1](../../evidence/2026-10-13-agent-attention-contract/screen-results.json),
[screen v2](../../evidence/2026-10-13-agent-attention-contract/screen-results-v2.json),
and [integration v1](../../evidence/2026-10-13-agent-attention-contract/integration-results.json).
The first screen probe changed frames on fixed waits before acquisition had
settled. The second waited for observed working state. The final probe added a
second working/cancellation cycle after acquisition to establish the false-done
case. The first Pi harness supplied arbitrary reason fields; the final harness
used the actual `aborted` field confirmed in upstream types.

## Source findings and limits

These are source observations, separate from the executed results:

- Herdr's [EventHub](https://github.com/herdrdev/herdr/blob/5a2dee700eeeea68267a4d16777307632f77172f/src/api/event_hub.rs)
  retains only 512 events. The [subscription layer](https://github.com/herdrdev/herdr/blob/5a2dee700eeeea68267a4d16777307632f77172f/src/api/subscriptions.rs)
  consumes internal sequence numbers but does not expose them or a replay-gap
  contract. A new attention stream needs an atomic snapshot boundary and explicit
  resynchronization; local counter inference cannot repair a dropped event.
- The internal state-change counter advances on activity changes and is also
  unsuitable as an attention revision. Two different input requests can occur
  without changing blocked state. Hook report sequence numbers order reports;
  they are not provider occurrence IDs.
- The six hook-authoritative pairs in the pinned detector are Pi, OMP, MastraCode,
  OpenCode, Kilo and Kimi. Hook authority establishes state ownership, not
  successful completion or cancellation/failure discrimination. Other agents
  principally use manifests; no manifest outcome type represents failure.
- [OpenCode v1.2.27's event types](https://github.com/anomalyco/opencode/blob/v1.2.27/packages/sdk/js/src/gen/types.gen.ts)
  distinguish `MessageAbortedError` from other errors. Its
  [processor](https://github.com/anomalyco/opencode/blob/v1.2.27/packages/opencode/src/session/processor.ts)
  distinguishes retry from terminal error paths, but also publishes an error
  before automatic context compaction. An error event alone is therefore not
  proof of terminal failure. Outcome classification must track the run through
  recovery and settlement.
- [Pi's inspected extension types](https://github.com/badlogic/pi-mono/blob/7d75a0e3eb273dff753d49c066ea29fbe7b3b6f6/packages/coding-agent/src/core/extensions/types.ts)
  expose aborted settlement, outcome-bearing boundaries and UI prompt events.
  These show that richer evidence exists upstream; installed-version coverage
  and retry/continuation ordering remain untested. The current Chartr Pi
  compatibility shim for older versions does not add outcome discrimination.
- Chartr's `StateStore` already has a transactional `app_state` key/value table.
  Layout snapshots save only their owned keys. A separately owned versioned
  attention value can use this store without adding another database or mixing
  acknowledgments into layout flags. That new value is proposed, not tested.

## Cleanup and next step

Every probe sent `server stop` through its isolated socket. Exit status was 0
and each socket disappeared, including the handoff successor's socket. A process
check found no remaining fixture writers. Temporary profiles and downloaded
source remain for investigation; no running probe owns them. Evidence was copied
into the wiki before handoff.

The [revised design](../../discussions/agent-attention.md#proposed-implementation)
keeps Herdr as the detection owner but requires outcome-aware integrations,
qualified attention events, handoff continuity and Chartr-owned read persistence.
No further product preference was needed to reach this design. Implementing it,
certifying provider/version coverage, and exercising native notifications remain
future work requiring implementation scope.
