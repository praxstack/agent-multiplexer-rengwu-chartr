---
title: Agent attention implementation attempts
section: Verification
updated: 2026-10-14
result: Failed
---
# Agent attention implementation attempts

This preserves the failed attempts from October 13–14. The corrected build and
remaining coverage limits are in the [final recheck](2026-10-14-agent-attention.md).
Scope: [agents](../../features/agents-inbox.md), [terminals](../../features/terminals.md)
and [panes](../../features/panes.md), under DEC-ATTENTION-004.

Chartr HEAD was `402ba90a25d82b527f68f23c1dc5f29e27065036` with uncommitted
implementation changes. Backend source was Herdr
`5a2dee700eeeea68267a4d16777307632f77172f` plus the developing attention patch.
Intermediate source snapshots were not each fingerprinted; these results do not
certify the final tree. Intermediate binary identities are retained in the
[Pi probe](../../evidence/2026-10-14-agent-attention/pi-results.json),
[vendored probe](../../evidence/2026-10-14-agent-attention/pi-final-results.json)
and [native instance](../../evidence/2026-10-14-agent-attention/ui-instance.json).

| Attempt | Expected | Observed | Result |
| --- | --- | --- | --- |
| Backend build with default Zig | Compile vendored terminal library | Installed Zig 0.16 rejected; the build requires 0.15.2. Corrected by setting `ZIG=/opt/homebrew/opt/zig@0.15/bin/zig`. | Failed |
| Backend suite in one Cargo test process | Complete suite | PTY-related SIGPIPE ended the shared process. `cargo nextest` was unavailable; a per-test subprocess runner was used next. | Failed |
| Backend unit tests in isolated processes | All 2,819 pass | 2,817 passed. Generated API schema and OpenCode TUI integration version marker were stale. Both were corrected and separately rechecked. | Failed |
| First native bundle launch | Private XDG profile | Launch Services inherited an existing XDG setting over `LSEnvironment`, opening the nested existing profile. No terminal was driven. | Failed |
| Native Quit shortcut | Exit the disposable app | The shortcut did not exit and the app menu exposed no Quit action through the tool. SIGTERM was used for the later restart check and cleanup; this is not a graceful-quit pass. | Failed |

Evidence: [initial backend suite](../../evidence/2026-10-14-agent-attention/backend-tests.log),
[isolated results](../../evidence/2026-10-14-agent-attention/backend-isolated-results.json),
[compact summary](../../evidence/2026-10-14-agent-attention/backend-summary.json),
[isolated runner](../../evidence/2026-10-14-agent-attention/run-backend-tests.py.txt),
[schema build with wrong Zig](../../evidence/2026-10-14-agent-attention/backend-schema-recheck.log),
[isolation attempt](../../evidence/2026-10-14-agent-attention/ui-isolation-attempt.json)
and [native stop](../../evidence/2026-10-14-agent-attention/ui-final-stop.json).

## Isolation repair

The first test app used `/Users/rengwu/.config/chartr/chartr/`, rather than its
intended temporary profile. Startup had handed that nested backend to the test
binary and added the test project to its space list. The test app was stopped;
the nested backend was handed back to `/Applications/chartr.app/Contents/MacOS/herdr`
and its original `0.8.2-chartr.5a2dee700eee` identity was verified. Only the exact
test-created space entry was removed. No existing terminal received commands or
was closed. The primary profile's backend remained on its original version.

Subsequent launches used explicit XDG environment variables on the executable,
then verified the private socket and empty UI before fixture creation. The user
later confirmed the earlier correctly isolated “Chartr Attention” instance was
disposable; it and the final instance were stopped. Their private sockets and
run-owned processes were absent after [cleanup](../../evidence/2026-10-14-agent-attention/cleanup.json).

The failed isolation recipe is corrected in [shared setup](../setup.md).
No installed application binary was replaced and no upstream change was published.
