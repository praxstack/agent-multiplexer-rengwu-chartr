# Documentation

These guides describe the current Rust working tree. Start with
[Installation](installation.md) and [Getting started](getting-started.md).

## Current guides

| Topic | Guide |
| --- | --- |
| Source builds, supported platforms, and package installation | [Installation](installation.md) |
| First space, agent setup, and the Wayfinder workflow | [Getting started](getting-started.md) |
| Spaces, panes, terminals, settings, persistence, and data locations | [Workspace](workspace.md) |
| Chats view, Inbox/Archive, agent discovery, and session logs | [Inbox](conversations.md) |
| Package installation, prerequisites, permissions, and SDK contracts | [Plugins](plugins.md) |
| Persistent plugin activity | [Status bar](status-bar.md) |
| Ordered local and Git skill sources | [Skill sources](../plugins/skills/README.md) |
| Shared prompt library and editor modals | [Saved Prompts](../plugins/prompts/README.md) |
| Template composition and explicit project-file saves | [Markdown Prompt](../plugins/markdown-prompt/README.md) |
| Maps, ticket launch, and claim recovery | [Wayfinder](../plugins/wayfinder/README.md) |
| Markdown map/ticket format | [Tracker convention](../plugins/wayfinder/TRACKER-CONVENTION.md) |

## Development and design

Use the [development wiki](wiki/index.md), or open `docs/wiki/index.html` for its
offline reader. Public guides above remain outside the development wiki.

- [Development workflow](wiki/development/workflow.md): advisory jstack skills,
  records, reader commands and contributor entry points.
- [Feature map](wiki/features/index.md): current source baseline, stable feature
  IDs, expected outcomes, recipes and coverage limits.
- [Code map](wiki/development/code-map.md): implementation owners and entry points.
- [Release builds](wiki/development/releasing.md) and
  [acceptance](wiki/verification/acceptance.md): existing shipping runbook and gates.
- [Verification setup](wiki/verification/setup.md): build identity, isolation,
  available tools and evidence. Imported recipes are unexecuted.
- [Architecture decisions](wiki/decisions/adr/index.md): original IDs, rationale
  and supersession, linked to current features.
- [Plugin examples](../examples/plugins/README.md),
  [theme playground](../misc/theme-playground/README.md),
  [font assets](../crates/chartr/assets/fonts/README.md), and
  [icon provenance](../crates/chartr/assets/icons/README.md) retain guidance beside
  their implementation. Maintained [platform patches](../vendor/zed-platform/README.md)
  and [terminal-view patch](../vendor/zed-terminal-view/chartr-PATCH.md) stay with vendor code.

## Historical and inactive material

[Research](wiki/archive/research/index.md) contains dated proposals, experiments, screenshots,
and verification receipts. Its older feature descriptions and test counts are
historical, not current availability or release guarantees. Rich chat has been
replaced by Inbox's original-terminal view.

[Mobile Companion](../plugins/companion/README.md) and its
[development protocol](wiki/archive/companion-protocol.md) are retained for future work;
Companion is excluded from the current desktop build.

The [17 September documentation audit](wiki/archive/research/2026-09-17-documentation-audit.md)
records the review scope, corrections, validation, and remaining limits. When
changing behavior, keep its public guide, relevant feature record and acceptance
checks consistent;
preserve dated evidence and annotate it when later decisions supersede it.
