---
title: Sidebar CI fixture repair
section: Verification
updated: 2026-10-12
result: Passed
---
# Sidebar CI fixture repair

## Scope

Repair verification drift in [FEAT-workspace-views](../../features/workspace-views.md):
the sidebar test still expected an expanded empty space to contain a row after
the accepted 2026-10-08 change removed the empty-space placeholder.

The selected scope is the repaired GPUI sidebar scenario and the CI checks
available on this macOS host. Linux-only checks, external-fixture tests and a
fresh GitHub Actions matrix are outside this local result. No product behavior
or CI workflow was changed.

## Baseline evidence

[CI run 38163267097](https://github.com/rengwu/chartr/actions/runs/38163267097)
failed on both macOS and Ubuntu at the same assertion:
`collapsed.size.height < expanded.size.height`. Both fixture spaces were empty,
so the accepted header-only rendering correctly kept their height unchanged.
The [CI excerpts](../../evidence/2026-10-12-sidebar-ci/ci-failure-excerpts.log)
retain both failures. The existing Arch/AUR job passed; later steps in the macOS
and Ubuntu test jobs were skipped after this failure.

Before editing the test, the focused command below reproduced that exact failure
locally with exit code 101. Its [test output](../../evidence/2026-10-12-sidebar-ci/baseline-test.log)
is preserved separately from the repaired-tree results. This was harness drift,
not evidence that the requested empty-space UI should be reverted.

## Build and environment

- HEAD `237057e2eab39291358ae82a51e5ee329a47bf61`, dirty with the sidebar-test
  correction and wiki records. The test patch SHA-256 is
  `55983d79ea9473d3f451db2c10b3208f8e98a531056292434c61be953ab8823d`.
- [Exact test patch](../../evidence/2026-10-12-sidebar-ci/test-change.patch.txt)
  and [manifest](../../evidence/2026-10-12-sidebar-ci/build-manifest.json) record
  the source hash, commands, exit codes and environment.
- arm64 macOS 27.0, Rust 1.96.0, Node 22.19.0, Python 3.13.0.
  Cargo uses CI's `CARGO_PROFILE_DEV_DEBUG=0` and `CARGO_PROFILE_TEST_DEBUG=0`.
- GPUI's test context renders the real sidebar with two synthetic spaces and
  dispatches pointer events. The populated phase adds one terminal entry and
  installs a bare settings store. Real-session tests use their existing temporary
  private Herdr namespaces and cleanup.

## Results on the repaired tree

| Scenario | Expected outcome | Observed outcome | Result | Evidence |
| --- | --- | --- | --- | --- |
| Empty-space heading and chevron | Toggling changes collapsed state without changing either space's bounds | Both collapse and expansion preserve exact bounds; state toggles in both directions | Passed | [Focused test](../../evidence/2026-10-12-sidebar-ci/focused.log) |
| Populated collapse, add and sort | Hiding rows shrinks the space and moves its neighbor; add does not bubble; dragging reorders without collapsing | Existing assertions pass with a real entry; collapsed bounds also exactly match the empty header; a positive gap separates the spaces | Passed | Focused test above |
| Default macOS workspace suite | All enabled tests pass, including the sidebar scenario | 419 passed, zero failed; seven explicitly ignored by their existing declarations | Passed | [Workspace output](../../evidence/2026-10-12-sidebar-ci/workspace.log) |
| Real terminal sessions | Direct attach, recovery and mouse routing pass with a private backend | All three tests passed in the explicit ignored-test invocation | Passed | [Session output](../../evidence/2026-10-12-sidebar-ci/live-session.log) |
| Build-time native plugin example | Locked check succeeds | Exit code zero | Passed | [Example output](../../evidence/2026-10-12-sidebar-ci/native-example.log) |
| Host-eligible release-script tests | All tests supported on this Mac pass | 13 passed; nine Linux/Arch packaging cases skipped by their platform guards | Passed | [Script output](../../evidence/2026-10-12-sidebar-ci/release-scripts.log) |
| Web plugin bridges and layout | Both Node test files pass | 10 passed, zero failed or skipped | Passed | [JavaScript output](../../evidence/2026-10-12-sidebar-ci/javascript.log) |
| Rust formatting | Workspace formatting matches rustfmt | Exit code zero | Passed | [Format output](../../evidence/2026-10-12-sidebar-ci/format.log), manifest |

Overall **Passed** for the selected local repaired-tree scope. The original
failed baseline remains failed. The three real-session tests account for three
of the seven default workspace ignores; the other four are not selected here.

To repeat the Rust checks after setting the two Cargo environment variables above:

```sh
cargo test -p chartr --bin chartr --locked sidebar_tree_collapses_adds_and_sorts_free_sessions
cargo test --workspace --locked --no-fail-fast --timings
cargo test -p chartr --test live_session --locked -- --ignored --nocapture --test-threads=1
```

## Limits and follow-up

This is a local macOS verification result, not a green GitHub Actions run. After
the change is pushed, CI must confirm both runners, including Ubuntu's forwarded
X11 events, GTK scale/lifecycle checks and packaging tests. Those paths cannot
run on this Mac. The native example, real-session and Node checks that the failed
CI jobs had skipped were exercised locally here.

The four remaining default workspace ignores are a real native-plugin release
download, installed OpenCode/Grok fixtures and an illustrative plugin doctest.
They remain unexecuted. The broader release acceptance matrix and previously
recorded app lifecycle issues are not certified by this test-only repair.
