---
title: Herdr attention plugin feasibility
section: Verification runs
updated: 2026-10-13
result: Failed
---
# Herdr attention plugin feasibility

## Scope and result

The [question 13 answer](../../journal/2026-10-13-grill-6ad221ca-e028-42eb-95eb-6ec117d4f4bd.md)
asks what needs changing and whether this could be a Herdr plugin. This run
tests the pinned backend's plugin event boundary as a candidate for the
accepted [attention contract](../../discussions/agent-attention.md). Related
features: [FEAT-agents-inbox](../../features/agents-inbox.md),
[FEAT-terminals](../../features/terminals.md), and
[FEAT-panes](../../features/panes.md).

The aggregate is **Failed** for the candidate plugin-only contract: a plugin
can receive status events, but the observed payload does not supply the
occurrence/outcome facts required by the accepted design. The temporary probe
completed successfully and its characterization assertions passed. This is not
a regression result for an implemented attention feature.

## Identity and isolation

- Chartr revision: `402ba90a25d82b527f68f23c1dc5f29e27065036`, with dirty wiki
  records/evidence; no product edits.
- Executed binary: `target/debug/herdr`, version
  `0.8.2-chartr.5a2dee700eee`, protocol 22; source revision
  `5a2dee700eeeea68267a4d16777307632f77172f`.
- SHA-256:
  `d9c83f20c4a7daa88570e32ee7af35c86ad4c9ec5ff3ec212a783565db9ec754`.
- The [manifest](../../evidence/2026-10-13-agent-attention-plugin/manifest.json)
  contains the actual binary digest, host, PID, UTC timestamp, tracked dirty
  fingerprint and evidence hashes. The pre-existing untracked `untitled` was
  neither read nor included.
- Isolated root: `/tmp/ca-plug-9ifrtz71`; all five XDG roots, Herdr socket and
  config were private to this run. Inherited `HERDR_*` selectors were cleared.
  Shell: `/bin/sh`, `ENV=/dev/null`; version/manifest update checks disabled.
- Linked only a temporary local plugin that writes selected event/context JSON
  to its isolated state directory. No real agent, provider authentication,
  remote notification, or user's normal plugin registry was used.

Driver: [probe.py.txt](../../evidence/2026-10-13-agent-attention-plugin/probe.py.txt).
Execution: `python3 /tmp/chartr-attention-plugin-probe/probe.py`.
Observations: [results.json](../../evidence/2026-10-13-agent-attention-plugin/results.json).

## Executed scenarios

| Scenario | Expected for the candidate | Observed | Result |
| --- | --- | --- | --- |
| Link a local `pane.agent_status_changed` hook and report fixture states | Plugin can receive background pane state changes | Five hook commands succeeded and wrote captured JSON, including working, done and blocked | Passed |
| Event identity and outcome | Input must distinguish qualified occurrences and retain outcome information | Captured event/context has coarse status, pane/workspace and presentation data; no outcome, producer sequence or occurrence identity | Failed |
| Title update while done | No fresh completion candidate from presentation alone | A second done hook arrived with only the title changed | Failed |
| Repeat blocked report | Establish whether a report is observable independently of state changes | A second accepted blocked report produced no second hook. This is a duplicate-state fixture, not a real second provider request | Passed |
| Represent confirmed failure through current reporting method | A distinct failure can be reported | `state: failed` rejected; accepted states are idle, working, blocked, unknown | Failed |
| Cleanup | Stop only the disposable backend and retain evidence | Stop returned 0, server exited 0, socket removed, PID confirmed absent | Passed |

The JSON checks assert the observed deficiencies, not that the desired feature
passes. A plugin could filter title-only repeats; that still would not recover
missing cancellation, failure, or distinct request facts. The earlier
[contract investigation](2026-10-13-agent-attention-contract.md) separately
demonstrated cancellation/fallback ambiguity and information loss in integration
assets. Those probes were not repeated or reclassified here.

## Source review and interpretation

The pinned [plugin documentation](https://github.com/herdrdev/herdr/blob/5a2dee700eeeea68267a4d16777307632f77172f/docs/next/website/src/content/docs/plugins.mdx)
describes executable commands with manifest actions, startup hooks, event hooks,
panes, and plugin-owned durable files. Plugins call the existing CLI/socket API.
This is an executable workflow extension, not an in-process API for replacing
agent detection, registering socket methods or changing handoff serialization.

The [event schema](https://github.com/herdrdev/herdr/blob/5a2dee700eeeea68267a4d16777307632f77172f/src/api/schema/events.rs)
and [hook runtime](https://github.com/herdrdev/herdr/blob/5a2dee700eeeea68267a4d16777307632f77172f/src/app/api/plugins/runtime.rs)
show that plugins receive the same normalized status envelope. Commands launch
asynchronously, with a 32-command in-flight limit; exceeding it records an
error rather than a durable retry queue. This limit was source-reviewed, not
load-tested. Startup hooks run again after live handoff and are documented as
one-shot initialization, not supervised services.

The public [Telegram notification example](https://github.com/ogulcancelik/herdr-plugin-examples/tree/main/agent-telegram-notify)
demonstrates simple delivery from these events. Its inspected `notify.mjs`
accepts done and blocked. The example was read, not installed or executed.

A plugin-only bridge that consumes current Herdr statuses cannot satisfy all
accepted semantics. A separate executable could instead receive raw provider
events, own their normalization, retain state and serve Chartr over its own
IPC. That is technically possible, but adds independently maintained agent
integrations and a service lifecycle. It changes the accepted reuse direction;
packaging that code as a Herdr plugin does not supply the missing facts.

## Limits and next step

No product implementation, UI dot, exact-focus acknowledgment, packaged native
notification, real-provider event qualification, plugin saturation test, or
independent review was performed. The temporary profile and driver remain for
reproduction; all run-owned processes stopped and evidence was copied into the
wiki. Q-ATTENTION-004 remains unresolved: the user's notes did not accept a
tracked patch, fork, or revised plugin architecture.
