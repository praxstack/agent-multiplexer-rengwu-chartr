---
title: Lean attention setup attempts
section: Verification
updated: 2026-10-15
result: Failed
---
# Lean attention setup attempts

Overall **Failed** because the first native launch crashed. This preserves failed
setup and observation attempts; the [final recheck](2026-10-15-lean-attention.md)
records the corrected normal build and its bounded results.

| Attempt | Expected | Observed and disposition | Result |
| --- | --- | --- | --- |
| First native bundle | Create a terminal in a normal app build | Copied `target/debug/chartr` while test compilation was also rebuilding it. Terminal creation panicked in GPUI's test-executor path. A normal build after tests corrected the launch. | Failed |
| Upstream test toolchain | Use the pinned Rust version | Initially started with 1.95.0; stopped that exact Cargo process and reran with 1.96.0. No passing claim for the interrupted attempt. | Untested |
| Initial notification bundle | macOS accepts notification connection | `/tmp` bundle failed notification validation, including after registration. The correctly formed, signed/registered target-directory bundle was accepted; location/metadata/signing changed together. | Failed |
| First minimized observation | Keep the window minimized during completion | Inspecting Chartr reactivated it and suppressed attention. The repeat observed Finder and passed. | Untested |
| First closed-window prompt | Submit work and observe completion without a window | Prompt entry/Enter timing did not start Codex work. Initial state remained idle/unread false; a later separate Enter started work and produced revision 6. | Untested |
| Split/group setup | Put Codex beside another terminal in a group | Drag attempts did not establish the layout; one reordered disposable spaces. No native split/group result claimed. | Untested |
| Notification click | Inspect and click the actual outstanding alert | Notification Center repeatedly timed out. Permission dialog was accessible; actual alert was not, including while unread revision 6 remained outstanding. | Untested |
| Native Quit shortcut | Exit the isolated app | Tool shortcut did not establish process exit. The run made no graceful-quit claim and cleaned up through exact process identity/private socket. | Untested |

The [initial macOS rejection log](../../evidence/2026-10-15-lean-attention/initial-notifications.log),
[first crash log](../../evidence/2026-10-15-lean-attention/first-launch-failed.log),
[initial closed-window observation](../../evidence/2026-10-15-lean-attention/closed-window-attempt.json),
[unsubmitted repeat](../../evidence/2026-10-15-lean-attention/closed-window.json),
[corrected transitions](../../evidence/2026-10-15-lean-attention/closed-window-second.json),
and [cleanup](../../evidence/2026-10-15-lean-attention/cleanup.json) remain available.
No failed attempt is counted as a successful product check.
