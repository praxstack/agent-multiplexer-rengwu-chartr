---
title: Verification setup
section: Verification
updated: 2026-10-05
recipe: Unvalidated
---
# Verification setup

This is shared setup for future product verification. **It was source-reviewed
but not executed during the documentation migration.** No new launch helper or
driver has been introduced. The migration checks are documented in its
[journal](../journal/2026-10-05-jstack-migration-implementation.md).

## Baseline and build identity

The migration inspected chartr HEAD `c853533c` plus five existing dirty Rust files:
`actions.rs`, `app.rs`, `app/command_palette.rs`, `app/view.rs` and `keymap.rs`
under `crates/chartr/src/`. Their hashes and the original patch fingerprint are in
the [migration manifest](../evidence/2026-10-05-docs-migration/migration-manifest.json).
Close All Sessions is a source-observed local addition, not a verified or released
feature. Cycle view modes already exists in the recorded committed baseline. GitHub main links identify owners;
they do not reproduce the dirty tree.

For a real run, record the current full `git rev-parse HEAD`, `git status --short`,
the binary's SHA-256, and a SHA-256 of `git diff --binary HEAD`. Include hashes of
relevant untracked source/fixtures separately, because Git diff excludes them.
Keep evidence free of credentials. Record OS/architecture, Rust toolchain, pinned
Herdr identity, launch flags and the exact executable path. A log line or menu
version alone does not establish which dirty binary was exercised.

## Build prerequisites and commands

Follow [source installation](https://github.com/rengwu/chartr/blob/main/docs/installation.md#build-from-source)
for system dependencies, the pinned Rust toolchain and exact sidecar. The
[release runbook](../development/releasing.md) covers distributable artifacts.
The existing project launch/build commands are:

```sh
rustup show
# Only when the pinned sidecar is not already available:
ZIG=/absolute/path/to/zig-0.15.2/zig sh vendor/herdr/fetch.sh
cargo build -p chartr --locked
# The installation guide also supports cargo run -p chartr --locked.
```

These commands are recipes; they were not run for this migration. Use the built
`target/debug/chartr` directly for a future local run so its identity is explicit.

## Instance and data isolation

Create a new absolute temporary directory for each run. Chartr resolves its
configuration/Herdr namespace, SQLite state and plugin data through the XDG
variables below. Keep each mutable instance under one driver owner.

```sh
CHARTR_RUN_ROOT=$(mktemp -d /tmp/chartr-verify.XXXXXX)
mkdir -p "$CHARTR_RUN_ROOT/config" "$CHARTR_RUN_ROOT/state" \
  "$CHARTR_RUN_ROOT/data" "$CHARTR_RUN_ROOT/project"
XDG_CONFIG_HOME="$CHARTR_RUN_ROOT/config" \
XDG_STATE_HOME="$CHARTR_RUN_ROOT/state" \
XDG_DATA_HOME="$CHARTR_RUN_ROOT/data" \
  target/debug/chartr "$CHARTR_RUN_ROOT/project"
```

Record that root and the new process identity. Absolute XDG paths isolate the
app-owned files, not the entire OS account: provider history, installed CLI tools,
system plugin roots and account credentials can still be discovered outside it.
Use disposable fixtures and inspect screenshots/logs before retaining them.
Do not change the user's HOME or drive an already-running personal workspace.

Readiness means the intended native window is responsive, the fixture folder is
the owning space, and a disposable shell can produce a known marker through the
real terminal. Check the actual XDG-owned state and private Herdr process before
performing destructive lifecycle scenarios. Record exact instance identity and
observations; do not mistake a launch request for readiness.

## Interaction tools and fixtures

Use available native desktop automation or hands-on pointer/keyboard interaction
against the real GPUI window. Browser tools can exercise the offline wiki and
standalone web fixtures, but cannot by themselves prove native terminal, Settings,
clipboard/IME or child-surface behavior. No stable native automation selector
contract is claimed; recipes name visible actions and documented keybindings.

Provide two disposable project folders, short-lived shell sessions, a known
web plugin (the local Clock example), and optional prebuilt embedded/plugin-status
fixtures. Agent/Wayfinder checks need a harmless configured adapter and test skill
source. Provider-specific checks need the corresponding installed/authenticated
tools; record missing prerequisites as Untested rather than starting paid agents
or substituting another adapter silently.

## Existing automated checks

These are available product checks, not newly imposed jstack gates. Select them
according to the change; existing release CI remains authoritative for its matrix.

```sh
cargo fmt --all --check
cargo test --workspace --locked --no-fail-fast
cargo check --manifest-path examples/plugins/hello/Cargo.toml --locked
python3 -B -m unittest discover -s scripts/tests -v
node --test crates/chartr/tests/plugin_bridge.cjs plugins/wayfinder/tests/layout.test.mjs
cargo test -p chartr --test live_session --locked -- --ignored --nocapture --test-threads=1
```

Node.js 22 is already used by CI. Real-session tests require the exact sidecar and
use a temporary namespace. Default Cargo test does not run ignored tests.
Linux CI additionally exercises forwarded X11 events and GTK scaling/child shutdown
under Xvfb at scales 1 and 2; Arch packaging/AUR fixture checks run in an Arch
container. Use the actual
[CI commands](https://github.com/rengwu/chartr/blob/main/.github/workflows/ci.yml)
on the matching platform rather than claiming macOS commands cover those paths.
No test count from an old audit establishes the current suite's result.

## Observations and evidence

Choose a unique dated run ID. Link the affected feature IDs and list the selected
success, cancel, error and persistence scenarios. Store screenshots, relevant
logs, JSON or text observations under `evidence/<run-id>/`; redact secrets. Check
underlying file contents/process state where the effect is not visible, and reopen
for persistence assertions. Distinguish product failures from setup/driver failures.

Use the [workflow's run rules](../development/workflow.md#keep-records-honest) and
keep earlier attempts after a fix. A recipe remains unvalidated until actually
executed against a recorded build; a successful documentation build cannot change
that status.

## Cleanup

Save evidence outside the temporary profile before cleanup. Close the disposable
sessions created by the run and quit its app instance. Normal quit detaches by
default, so verify no run-owned sessions remain and identify any run-owned private
Herdr process explicitly before stopping it. Never use broad process-name kills.
Remove only the recorded temporary root after confirming no process needs it;
preserve the evidence directory and any earlier runs. Restore any external test
fixture changes made by the run. Missing safe cleanup information is a setup gap,
not permission to operate on the user's normal profile.
