---
title: Release readiness after attention and chrome commits
section: Verification
updated: 2026-10-16
result: Failed
---
# Release readiness after attention and chrome commits

**Not ready for a stable release.** The current workspace suite fails its
header-fade rendering test. The candidate has no matching CI or package-build
results, native attention interaction coverage remains incomplete, and the
workspace still uses the already-released version `0.3.0`. Automatic AUR
publication is also missing its required repository configuration.

## Scope

This is a release-readiness assessment of `c11b2dc5`, following the commit split.
It runs the locally available automated gates from
[release acceptance](../acceptance.md), checks current GitHub CI/package identity,
and reviews the remaining attention and distribution requirements. It does not
claim completion of the full native visual, accessibility, installation or
cross-platform matrix.

Affected records: [distribution](../../features/distribution.md)
(`FEAT-distribution`), [workspace views](../../features/workspace-views.md)
(`FEAT-workspace-views`), [agent attention](../../features/agents-inbox.md)
(`FEAT-agents-inbox`), [session lifecycle](../../features/session-lifecycle.md)
(`FEAT-session-lifecycle`) and [terminals](../../features/terminals.md)
(`FEAT-terminals`).

## Build and environment

- Revision: `c11b2dc582a2e96f8d1a7d6fcafea66dc17037e5`.
- Tree: `3584a1f1db42e2e2f53ce172ef37972a68d3b94c`; clean before this run.
- macOS 27.0, Apple silicon; Rust 1.96.0, Node.js 22.19.0, Python 3.13.0.
- Herdr: `0.8.2-chartr.5a2dee700eee.identity.9ed02822c877`, protocol 22.
- [Manifest](../../evidence/2026-10-16-release-readiness/manifest.json) records the
  exact source tree, environment, final normal-app binary and sidecar hashes. Only verification records
  are added by this assessment; product sources and tests are not modified.
- [Executed runner](../../evidence/2026-10-16-release-readiness/run-checks.py.txt)
  records the local command sequence. The real-backend suite creates disposable
  `/tmp/cr-*` namespaces; the Metal example uses its own transient native window.

## Results

| Scenario | Expected | Observed | Result | Evidence |
| --- | --- | --- | --- | --- |
| Workspace regression suite | All exercised tests pass | 427 passed, one failed, seven ignored; failure is `spaces_scroll_under_the_fixed_header_and_fade_from_the_first_pixel` | Failed | [Log](../../evidence/2026-10-16-release-readiness/workspace.log), [counts](../../evidence/2026-10-16-release-readiness/workspace-summary.json) |
| Formatting | Workspace formatting passes | `cargo fmt --all --check` exited 0 | Passed | [Log](../../evidence/2026-10-16-release-readiness/format.log) |
| Release script checks on macOS | Applicable fixtures pass | 15 passed; nine Linux/Arch packaging fixtures skipped on this host | Passed | [Log](../../evidence/2026-10-16-release-readiness/scripts.log) |
| Web bridge and Wayfinder layout checks | All selected JavaScript tests pass | 10 passed | Passed | [Log](../../evidence/2026-10-16-release-readiness/javascript.log) |
| Real Herdr sessions | Exact sidecar, attach target, crash recovery and mouse/TUI behavior pass in private namespaces | All three explicitly enabled live-session tests passed | Passed | [Log](../../evidence/2026-10-16-release-readiness/live-session.log) |
| Backend terminal-identity handoff | The pinned patch preserves terminal identities and state invariants | One focused upstream test passed; the rest of the backend suite was filtered out | Passed | [Log](../../evidence/2026-10-16-release-readiness/backend-handoff.log), [command/result](../../evidence/2026-10-16-release-readiness/backend-results.json) |
| Build-time native plugin contract | The Hello module compiles with the locked dependency graph | Locked Hello module check exited 0 | Passed | [Log](../../evidence/2026-10-16-release-readiness/native-plugin.log) |
| Normal application build | A normal binary builds after test compilation | `cargo build -p chartr --locked` exited 0; the copied sidecar hash matches the pinned executable | Passed | [Log](../../evidence/2026-10-16-release-readiness/normal-build.log), [manifest](../../evidence/2026-10-16-release-readiness/manifest.json) |
| Native macOS frame source | Selected native lifetime and frame-delivery checks pass | One policy test and the native example passed: unused updates, presentation, pause, resize/resume, eight transaction recreation cycles, drop and callback-owned drop. Low Power Mode was off | Passed | [Log](../../evidence/2026-10-16-release-readiness/metal-native.log) |
| CI for this candidate | macOS, Ubuntu and Arch/AUR CI pass for the candidate revision | No runs for `c11b2dc5`; latest successful matrix is `4cfe1581` | Untested | [Remote observations](../../evidence/2026-10-16-release-readiness/remote-gates.json) |
| Shipping packages | Apple silicon macOS and Linux x86_64/ARM64 packages build for this candidate; checksums and installation smoke checks pass | Latest successful packages belong to `4cfe1581`. No candidate packages or installation checks were produced in this assessment | Untested | [Remote observations](../../evidence/2026-10-16-release-readiness/remote-gates.json) |
| Native attention interaction | Inspect an actual banner, click to the exact terminal, reject stale notices, and verify split/group focus and lifecycle variants | Not repeated here; the October 15 run explicitly leaves banner/click, native split/group, Hide, graceful Quit and Linux coverage unverified | Untested | [Previous bounded run](2026-10-15-lean-attention.md) |
| Full native acceptance matrix | Required themes, sizes, input/accessibility, plugins and installation journeys pass | Not executed for this candidate | Untested | [Required matrix](../acceptance.md) |
| Stable AUR publication configuration | Required repository secret and variable exist | Both `AUR_SSH_PRIVATE_KEY` and `AUR_SSH_KNOWN_HOSTS` are absent. Names/presence only were queried; no credentials were read | Failed | [Configuration check](../../evidence/2026-10-16-release-readiness/aur-configuration.json) |
| New release version | Candidate uses a new version with a matching unused tag | Workspace remains `0.3.0`; `v0.3.0` is already published | Failed | [Source review](../../evidence/2026-10-16-release-readiness/source-review.json), [releases](../../evidence/2026-10-16-release-readiness/remote-gates.json) |

Skipped and ignored scenarios are coverage gaps, not passes. The script-check
Passed result applies only to its applicable macOS cases; Linux/Arch coverage
remains part of the untested candidate CI/package gate. Three of the seven
ignored workspace tests were subsequently enabled in the passing live-session
run. The four others cover a downloaded native-plugin release, installed
OpenCode/Grok history fixtures and an ignored documentation example; they were
not exercised here. The native Metal example
does not prove application input latency, battery/Low Power Mode behavior, or
the complete application lifecycle.

## Failure diagnosis and follow-up

The rendering test fails at `crates/chartr/src/chrome/sidebar_tests.rs:358`
because it cannot find the gradient it expects. The
[source comparison](../../evidence/2026-10-16-release-readiness/source-review.json)
shows that the test expects 20 px of growth and an opaque region from 75% to
100% of the header. The implementation currently uses 16 px and 30% to 60%.
The feature document still describes the older 20 px treatment as well. This
establishes disagreement between current tuning, test and documentation; the
failure alone does not establish a newly reproduced native clipping regression.

Before publishing:

1. Reconcile the test and feature expectations with the intended fade tuning,
   preserving the geometry, input and clipping assertions. Rerun the affected
   checks and the required automated gate after any change.
2. Complete actual notification banner/click/stale-target checks and the
   outstanding native focus/lifecycle and release acceptance scenarios.
3. Obtain matching candidate CI and package builds on the shipping platforms,
   then perform checksum and package-installation smoke checks. Older successful
   runs are not evidence for this candidate.
4. Choose the next version, update its release metadata and use a matching tag.
5. Configure AUR publication if it is part of the stable release. The current
   workflow would fail its initial configuration check; GitHub binary packages
   can be built and downloaded independently of AUR publication.

This assessment does not change product behavior, push commits, create a tag,
publish a release or configure external accounts. Earlier verification records
remain intact. The outcome is Failed even though several independent checks
pass, because an exercised release gate fails.

## Evidence and cleanup

Exit codes, commands and elapsed times are retained for the
[Rust checks](../../evidence/2026-10-16-release-readiness/rust-results.json),
[format/script/JavaScript checks](../../evidence/2026-10-16-release-readiness/fast-results.json),
[native frame-source check](../../evidence/2026-10-16-release-readiness/metal-results.json)
and [backend regression](../../evidence/2026-10-16-release-readiness/backend-results.json).

The [cleanup observation](../../evidence/2026-10-16-release-readiness/cleanup.json)
found no remaining newly created live-session scratch directories, Metal check
manifest directories or Metal example processes. Test/build artifacts and evidence
are retained. The user's running workspace was not used as a verification fixture.
