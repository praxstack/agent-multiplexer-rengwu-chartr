#!/bin/sh
# Exercise the real macOS frame-source module without rebuilding GPUI's test graph.
set -eu

if [ "$(uname -s)" != Darwin ]; then
    echo 'This check requires a macOS 14+ desktop session.' >&2
    exit 1
fi
CHARTR_CHECK_ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
CHARTR_CHECK_MANIFEST=$(mktemp -d "${TMPDIR:-/tmp}/chartr-metal-check.XXXXXX")
trap 'rm -rf "$CHARTR_CHECK_MANIFEST"' EXIT
trap 'exit 1' HUP INT TERM
cd "$CHARTR_CHECK_ROOT"

python3 - "$CHARTR_CHECK_ROOT" "$CHARTR_CHECK_MANIFEST" <<'PY'
import json
import pathlib
import shutil
import subprocess
import sys

root, scratch = map(pathlib.Path, sys.argv[1:])
session = subprocess.check_output(['ioreg', '-l', '-d', '1'], text=True)
if '"CGSSessionScreenIsLocked"=Yes' in session:
    sys.exit('Setup unavailable: unlock the macOS desktop before running native rendering checks.')
source = root / 'vendor/zed-platform/gpui_macos/examples/metal_display_link.rs'
(scratch / 'Cargo.toml').write_text('''[package]
name = "chartr-metal-native-check"
version = "0.0.0"
edition = "2024"
[workspace]
[dependencies]
cocoa = "=0.26.0"
core-graphics = "0.24"
foreign-types = "0.5"
metal = "0.33"
objc = "0.2"
[[bin]]
name = "chartr-metal-native-check"
path = ''' + json.dumps(str(source)) + '\n')
shutil.copy(root / 'Cargo.lock', scratch / 'Cargo.lock')
PY

export CARGO_TARGET_DIR="$CHARTR_CHECK_ROOT/target/metal-display-link-check"
# Offline resolution prunes the copied lock to this small subset. Versions remain
# those already fetched by the application build; no project lockfile is changed.
cargo test --offline --manifest-path "$CHARTR_CHECK_MANIFEST/Cargo.toml"
cargo run --locked --offline --manifest-path "$CHARTR_CHECK_MANIFEST/Cargo.toml"
