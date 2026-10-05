---
title: Feature map
section: Features
updated: 2026-10-05
---
# Feature map

The current development baseline is the inspected implementation, as accepted in
[DEC-JSTACK-003](../decisions/index.md). Every recipe below is **unvalidated** and
no chartr product tests were run during the documentation migration. The
[shared setup](../verification/setup.md) records baseline identity and prerequisites.

| Feature | ID | Main behavior |
| --- | --- | --- |
| [Spaces and folder ownership](spaces.md) | `FEAT-spaces` | Open a folder through the space controls or the folder argument at launch |
| [Tabs, groups and pane layout](panes.md) | `FEAT-panes` | Create terminals or surfaces with the outer or pane-local add controls |
| [Views, chrome and navigation](workspace-views.md) | `FEAT-workspace-views` | Use the Tabs / Spaces / Chats selector, the sidebar, title-bar controls, and command palette |
| [Terminal interaction and search](terminals.md) | `FEAT-terminals` | Use + to create a terminal |
| [Session lifecycle, recovery and persistence](session-lifecycle.md) | `FEAT-session-lifecycle` | Close an item/group/space, quit/reopen the app, or use the Problems menu and terminal recovery actions |
| [Settings, themes and keybindings](settings.md) | `FEAT-settings` | Open the single Settings window with the gear, command palette or Cmd/Ctrl+, |
| [Plugin discovery and installation](plugin-packages.md) | `FEAT-plugin-packages` | Use Settings → Plugins to prepare/install a local folder or Git package, inspect authority, enable/disable, replace, or restart for activation. |
| [Plugin surfaces, services and permissions](plugin-runtime.md) | `FEAT-plugin-runtime` | Open enabled tools from New surface; inspect contributed settings/services |
| [Agent launching and Inbox history](agents-inbox.md) | `FEAT-agents-inbox` | Configure Agent through Settings → Plugins, open its surface, or switch to Chats for searchable Inbox/Archive and the original terminal. |
| [Skill source registry and precedence](skill-sources.md) | `FEAT-skill-sources` | Use the Skill sources gear in Settings → Plugins; register local directories or Git sources, rescan, refresh, edit, enable/disable and drag reorder. |
| [Saved prompt library](saved-prompts.md) | `FEAT-saved-prompts` | Use Saved Prompts in Settings → Plugins |
| [Markdown composition and explicit saves](markdown-prompt.md) | `FEAT-markdown-prompt` | Open Markdown Prompt from New surface in a folder space |
| [Wayfinder maps, launch and claim recovery](wayfinder.md) | `FEAT-wayfinder` | Open Wayfinder in a folder space with Agent and Skill sources enabled |
| [Background plugin status](status-bar.md) | `FEAT-status-bar` | Enable Show status bar in General or use Workspace: Toggle status bar |
| [Installation, packages and release boundaries](distribution.md) | `FEAT-distribution` | Follow the public installation guide for supported packages/source builds; contributors use the release runbook and existing CI. |

The [97-story crosswalk](story-crosswalk.md) preserves the legacy specification's
identifiers. Features added after that specification (including packaging and
status-bar behavior) have their own records above. Companion and the removed
rich-chat UI are retained in the [historical archive](../archive/index.md), not
advertised as current mapped features.

Cross-feature journeys: spaces → panes → lifecycle/persistence; Agent → Inbox;
Skills/Saved Prompts → Markdown Prompt explicit Save; Wayfinder → Agent launch →
terminal identity → claim recovery; plugin installation → activation → permissions
→ close/restoration. The [release matrix](../verification/acceptance.md) retains
these broader combinations without claiming they have passed.
