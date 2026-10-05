---
title: Installation, packages and release boundaries
section: Features
updated: 2026-10-05
basis: Source-reviewed migration baseline
---
# Installation, packages and release boundaries

Feature ID: `FEAT-distribution`

## Intent and basis

This record uses the inspected current implementation under
[the accepted migration decision](../decisions/index.md). It describes the
working-tree baseline, not a newly executed product result. See
[baseline identity and limitations](../verification/setup.md#baseline-and-build-identity).

Architectural basis: [ADR 0002](../decisions/adr/0002-the-zed-layer.md), [ADR 0007](../decisions/adr/0007-prebuilt-native-surfaces.md). Historical portions of ADRs remain dated rationale.

Implementation owners: [.github/workflows/ci.yml](https://github.com/rengwu/chartr/blob/main/.github/workflows/ci.yml), [.github/workflows/release.yml](https://github.com/rengwu/chartr/blob/main/.github/workflows/release.yml), [.github/workflows/aur.yml](https://github.com/rengwu/chartr/blob/main/.github/workflows/aur.yml), [scripts/package-linux.sh](https://github.com/rengwu/chartr/blob/main/scripts/package-linux.sh), [scripts/build-dev-dmg.sh](https://github.com/rengwu/chartr/blob/main/scripts/build-dev-dmg.sh), [rust-toolchain.toml](https://github.com/rengwu/chartr/blob/main/rust-toolchain.toml).

## Entry points and prerequisites

Follow the public installation guide for supported packages/source builds; contributors use the release runbook and existing CI.

Use [shared setup](../verification/setup.md) and a disposable profile/project for
stateful recipes. Related areas are linked from the [feature index](index.md).

## Expected outcomes

- Current release targets are Apple silicon macOS and native Linux x86_64/ARM64. Windows is deferred; Linux uses X11/XWayland and the documented system dependencies.
- Linux formats reuse one release compilation per architecture; x86_64 additionally produces Arch/AUR artifacts. Tag/version matching, checksums and draft releases remain part of packaging.
- macOS DMGs are ad-hoc signed and unnotarized. The local development bundle identifier differs from the production identifier.
- AUR publication automation is configured in the repository for published stable releases. Its existence is not evidence that credentials are configured or the package is currently published.
- Chartr pins its Rust/Zed/Herdr inputs; there is no app frontend build or plugin compilation during installation.

## Verification recipe

**Unvalidated recipe — not executed during this migration.** Complete shared
setup first; use actual UI entry points and observe underlying state where relevant.

1. For a future release, follow the release runbook and existing CI on the shipping platforms. Record exact revisions, artifact names and checksums.
2. Install the .deb and Arch package on disposable supported systems; launch from a terminal and app menu and exercise a terminal/web pane. Extract and launch the tarball through its symlink layout.
3. Mount the macOS DMG, inspect bundle version/signing, and launch on a supported Mac. Record platform warnings as documented.
4. Keep AUR publication and release publishing separate from local validation; use the existing explicit release procedure.

Preserve the selected scenario's logs/screenshots/file observations and exact
build identity before [cleanup](../verification/setup.md#cleanup). Record a fresh
run if this recipe is exercised; source inspection alone is not a pass.

## Coverage and limitations

No package was built, installed, published or downloaded during migration. Source configuration does not establish current external release or AUR availability; public status claims were not recertified.

The [release matrix](../verification/acceptance.md) retains broader cross-feature,
platform, theme, pointer and accessibility coverage. Choose a bounded subset for
ordinary changes; release requirements remain separate from this advisory workflow.

## Verification history

No current product run was performed for this migration. Earlier
[research and audits](../archive/research/index.md) remain historical evidence for
their recorded snapshots; they do not certify this baseline.
