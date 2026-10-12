---
title: Last-window reopen implementation and verification
section: Verification
updated: 2026-10-12
result: Passed
---
# Last-window reopen implementation and verification

## Scope

Resolve the same-process last-window reopen failure in
[FEAT-session-lifecycle](../../features/session-lifecycle.md), reported by the
[October 11 native run](2026-10-11-metal-frame-delivery-recheck.md).
The user authorized implementation with work-until-grill on October 12.
The historical Failed result remains unchanged. A fresh
[unchanged-build reproduction](2026-10-12-window-reopen-baseline.md) also failed;
this report covers the changed build.

The app entry point never registered GPUI's optional reopen callback. The native
platform exposes that event hook, but there was no app callback to recreate a
window.
The original quit observer also retained the first window handle; after replacing
that window it would no longer flush/apply exit policy to the current workspace.

The patch registers `Application::on_reopen`, activates an existing workspace
window or restores one through the startup window factory, and finds current
workspace windows at quit. Reopening restores current persisted selection rather
than replaying the original command-line project. Fonts, global settings, plugins
and action registration remain process initialization. No renderer/backend change.

## Build and environment

- Base commit `4cfe1581cca6ec53cef79691e5add0fdf1829e37`, branch
  `fix/macos-window-reopen`, dirty product file `crates/chartr/src/main.rs`.
- Rust 1.96.0, arm64 macOS. Locked development builds with
  `CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0`.
- Product diff SHA-256
  `13c69161ae25e6837308de45c07fb48482299f2ae686b7f1e0f062898ee051ed`.
  [Product patch](../../evidence/2026-10-12-window-reopen/product.patch.txt),
  [build/bundle manifest](../../evidence/2026-10-12-window-reopen/build-manifest.json).
- Baseline and fixed ad hoc signed bundles under `/tmp/chartr-reopen.xf24t27z`,
  with separate XDG config/state/data/cache, project and backend namespaces.
  The installed app and user profile are unchanged.
- Native fixed-build executable SHA-256
  `832dee3aee1967228534dea84e269d309c6f580db9556d94fd1971fe32f4d15f`.
  Default-detach GUI PID 24261 and terminate-on-exit GUI PID 24649 used that same
  binary. [Native manifest](../../evidence/2026-10-12-window-reopen/native-build-manifest.json).

## Results

| Scenario | Expected outcome | Observed outcome | Result | Evidence |
| --- | --- | --- | --- | --- |
| Baseline and changed build | Both locked builds compile | Both development builds succeeded | Passed | [Baseline](../../evidence/2026-10-12-window-reopen/baseline-build.log), [changed](../../evidence/2026-10-12-window-reopen/fixed-build.log) |
| Workspace regression suite | Existing nonignored tests pass | 419 passed, 7 ignored; ignored native paths are not certified | Passed | [Workspace output](../../evidence/2026-10-12-window-reopen/workspace-tests.log) |
| Real private backend | Direct attach, restart recovery and mouse routing pass | 3 passed against disposable Herdr namespaces | Passed | [Live-session output](../../evidence/2026-10-12-window-reopen/live-tests.log) |
| Supporting local CI checks | Applicable release scripts, plugin tests and native example pass | Release scripts: 13 passed, 9 platform-dependent skips; JavaScript: 10 passed; native example cargo check succeeded | Passed | [Release scripts](../../evidence/2026-10-12-window-reopen/release-tests.log), [plugins](../../evidence/2026-10-12-window-reopen/plugin-tests.log), [example](../../evidence/2026-10-12-window-reopen/example-check.log) |
| Close last window and reopen | New visible workspace, unchanged app PID and disposable terminal identity, saved layout/bounds restored | Three cycles per profile: six new visible workspace windows, unchanged per-profile PID and terminal identity, successful native terminal input after each; saved layout and 960×640 bounds retained exactly | Passed | [Default-detach observations](../../evidence/2026-10-12-window-reopen/fixed-observations.json), [terminate observations](../../evidence/2026-10-12-window-reopen/fixed-terminate-observations.json), [asserted summary and terminal markers](../../evidence/2026-10-12-window-reopen/native-summary.json) |
| Repeated reopen and existing/minimized window | Opening an existing window creates no duplicate; minimized window is restored | Three additional opens per profile kept exactly one workspace with the same native window number. Minimize/open made that window visible and key again; subsequent terminal input succeeded | Passed | Native observations and summary above |
| Quit after window replacement | Default detach and configured terminate policies operate on the replacement window; final state flushes | Both GUI processes exited normally. Default detach preserved the same live terminal and saved items; configured terminate removed the backend pane and saved an empty item/layout list | Passed | Native observations and summary above |

Overall **Passed** for the changed build and selected scenarios. The earlier
blocked checkpoint below and the separate Failed baseline remain preserved.

## Initial blocked checkpoint — October 12

Before native execution this report was **Untested**: automated checks passed,
but the three selected native rows above had not run. The regular native tool
failed three startup attempts with `Sky Computer Use native pipe startup failed`.
The unlocked/on-console desktop and shell Swift Accessibility status were checked
read-only. [Original preflight](../../evidence/2026-10-12-window-reopen/desktop-preflight.json).
No permission setting was changed and no native outcome was inferred from unit tests.

The computer tool's instructions require an explicit user request to switch UI
automation technology. The [grill-form answer](../../journal/2026-10-12-grill-e4ec3c4a-3dec-4cfc-a0f5-5c9f91924f20.md)
authorized the isolated AppKit test driver. The completed additional attempt
below resolves that blocker; it does not rewrite the initial observations.

## Additional native attempt — October 12

The [AppKit driver](../../evidence/2026-10-12-window-reopen/chartr-reopen-driver.m.txt)
was compiled with `clang -dynamiclib -fobjc-arc -framework AppKit` and injected
only into the isolated GUI binaries. It removes the injection variables before
Chartr starts children. Native close/minimize/resize/keyboard/quit calls target
only that process's own window. Actual reopen events come from LaunchServices
`open -a <exact isolated bundle>`; the driver never calls Chartr's reopen handler
directly and does not synthesize global input or require Accessibility access.

The [runner](../../evidence/2026-10-12-window-reopen/chartr-reopen-native.py.txt) and
[fixed scenarios](../../evidence/2026-10-12-window-reopen/chartr-reopen-fixed-scenarios.py.txt)
retain the exercised commands. Each creates a disposable `/bin/sh` session in a
private backend before launching the GUI, then records native window state,
underlying pane/terminal identity, SQLite state and shell marker files. The
shell markers are written only by input delivered through Chartr's terminal,
including after each restored window. Zero-exit process checks and private
backend shutdown complete each run. These are one-off run drivers tied to the
recorded manifest, not new production dependencies or CI-wide infrastructure.

## Limits and follow-up

Verification used development binaries on this arm64 Mac, native window state
and terminal effects. It did not include screenshot/visual-layout review,
performance timing, release packaging, Linux native reopening, Intel/older macOS,
App Hide/menu behavior or full release acceptance. The broader matrix remains
separate. Runtime stderr was empty for the completed fixed fixtures:
[default](../../evidence/2026-10-12-window-reopen/fixed-runtime.log),
[terminate](../../evidence/2026-10-12-window-reopen/fixed-terminate-runtime.log).

## Cleanup

Both fixed GUI processes and their private backends exited with code zero. All
owned test PIDs are gone. The installed app/backend kept PIDs 1121/1123, and the
installed binary retained SHA-256
`699cbb789ad845e00e94bc2be69aa3ae9b6a2959a93ff554dedb24cd062cfcbe`.
[Final identity, marker and cleanup checks](../../evidence/2026-10-12-window-reopen/native-summary.json).
Temporary isolated bundles/profiles remain available as local test artifacts.
